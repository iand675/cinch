//! Native choreography preserves role projection, branch/wire types and ownership.
use cinch::driver::{check_sources, run_source_with, StagingCx};

fn libraries() -> String {
    format!("{}\n{}\n", include_str!("../../experiments/malleable-syntax/libraries/LocalWire.cinch"),
        include_str!("../../experiments/malleable-syntax/libraries/ChoreographySyntax.cinch"))
}

#[track_caller]
fn run(source: &str) {
    assert_eq!(run_source_with(source, &mut StagingCx::default()).unwrap(), 42);
}

#[track_caller]
fn refuses(source: &str, needle: &str) {
    let error = check_sources(source, "native-choreography.cinch").err().expect("must refuse").to_string();
    assert!(error.contains(needle), "{error}");
}

fn two_roles() -> String {
    format!("{}{}", libraries(), r#"module Sale where
  export scopeProbe
  import stage ChoreographySyntax
  generate do
    emit (ChoreographySyntax.declare!(Flow, ([(Buyer), (Shop)]), ([(Buyer, Shop, Order)]), Shop,
      ([(Approved, ([(Shop, Buyer, Receipt)])), (Declined, ([(Shop, Buyer, Reason)]))])))
  scopeProbe :: Nat
  scopeProbe = case MkFlowBuyerP1 42 of
    MkFlowBuyerP1 value -> value
module Main where
  import Sale
  main :: Int
  main =
    let buyer = start-Flow-Buyer () in
    let shop = start-Flow-Shop () in
    case send-Flow-Buyer-Order buyer 41 of
      (buyerWaiting, order) -> case receive-Flow-Shop-Order shop order of
        (shopReady, amount) -> case choose-Flow-Approved shopReady of
          (shopBranch, buyerChoice) -> case offer-Flow-Buyer buyerWaiting buyerChoice of
            FlowApprovedBuyer buyerBranch -> case send-Flow-Approved-Shop-Receipt shopBranch (amount + 1) of
              (shopDone, receipt) ->
                let shopCount = close-Flow-Approved-Shop shopDone in
                case receive-Flow-Approved-Buyer-Receipt buyerBranch receipt of
                  (buyerDone, value) ->
                    let buyerCount = close-Flow-Approved-Buyer buyerDone in
                    if value == 42 && shopCount + buyerCount == 4 && scopeProbe == 42 then 42 else 0
            _ -> 0
"#)
}

#[test]
fn two_role_projection_choice_offer_and_private_constructor_control_execute() {
    run(&two_roles());
}

#[test]
fn wrong_state_wire_branch_replay_and_private_constructor_are_refused() {
    let source = two_roles();
    refuses(&source.replace("choose-Flow-Approved shopReady", "choose-Flow-Approved (start-Flow-Shop ())"), "type mismatch");
    refuses(&source.replace("receive-Flow-Approved-Buyer-Receipt buyerBranch receipt", "receive-Flow-Approved-Buyer-Receipt buyerBranch (delay (\\() -> (42 :: Nat)))"), "type mismatch");
    refuses(&source.replace("close-Flow-Approved-Shop shopDone", "close-Flow-Declined-Shop shopDone"), "type mismatch");
    refuses(&source.replace("let shopCount = close-Flow-Approved-Shop shopDone in", "let repeated = close-Flow-Approved-Shop shopDone in\n                let shopCount = close-Flow-Approved-Shop shopDone in"), "%aff");
    refuses(&source.replace("scopeProbe == 42", "(case Sale.MkFlowBuyerP1 42 of { _ -> True })"), "MkFlowBuyerP1");
    refuses(&source.replace("scopeProbe = case MkFlowBuyerP1 42", "scopeProbe = case NotAnActualConstructor 42"), "NotAnActualConstructor");
}

#[test]
fn four_roles_three_branches_preserve_flat_order_and_zero_local_counters() {
    let source = format!("{}{}", libraries(), r#"module Dealer where
  import stage ChoreographySyntax
  generate do
    emit (ChoreographySyntax.declare!(Four, ([(A), (B), (C), (D)]), ([(A, B, Ping)]), C,
      ([(Alpha, ([(C, D, Done)])), (Beta, ([(C, A, Back)])), (Gamma, ([(D, B, Later)]))])))
module Main where
  import Dealer
  main :: Int
  main =
    let a = start-Four-A () in
    let b = start-Four-B () in
    let c = start-Four-C () in
    let d = start-Four-D () in
    case send-Four-A-Ping a 42 of
      (aWaiting, ping) -> case receive-Four-B-Ping b ping of
        (bWaiting, amount) -> case choose-Four-Gamma c of
          (cBranch, aChoice, bChoice, dChoice) ->
            let aOffer = offer-Four-A aWaiting aChoice in
            let bOffer = offer-Four-B bWaiting bChoice in
            let dOffer = offer-Four-D d dChoice in
            case (aOffer, bOffer, dOffer) of
              (FourGammaA aBranch, FourGammaB bBranch, FourGammaD dBranch) ->
                let aCount = close-Four-Gamma-A aBranch in
                let cCount = close-Four-Gamma-C cBranch in
                case send-Four-Gamma-D-Later dBranch amount of
                  (dDone, later) ->
                    let dCount = close-Four-Gamma-D dDone in
                    case receive-Four-Gamma-B-Later bBranch later of
                      (bDone, value) ->
                        let bCount = close-Four-Gamma-B bDone in
                        if value == 42 && aCount == 1 && bCount == 2 && cCount == 0 && dCount == 1 then 42 else 0
              _ -> 0
"#);
    run(&source);
    refuses(&source.replace("(cBranch, aChoice, bChoice, dChoice)", "(cBranch, bChoice, aChoice, dChoice)"), "type mismatch");
}

#[test]
fn original_three_role_checkout_runs_both_branches_with_native_declarations() {
    let root = r#"module Checkout where
  import stage ChoreographySyntax
  generate do
    emit (ChoreographySyntax.declare!(Checkout, ([(Buyer), (Shop), (Bank)]),
      ([(Buyer, Shop, Order), (Shop, Bank, Payment)]), Bank,
      ([(Approved, ([(Bank, Shop, Receipt), (Shop, Buyer, Confirmation)])), (Declined, ([(Bank, Shop, Reason), (Shop, Buyer, Rejection)]))])))
"#;
    let main = include_str!("../../experiments/malleable-syntax/examples/Choreography.cinch");
    let source = format!("{}{root}\n{main}", libraries());
    let program = check_sources(&source, "native-checkout.cinch").unwrap();
    let names = program.fns.iter().map(|function| function.id.0.as_str()).collect::<Vec<_>>();
    for required in ["Checkout.send-Checkout-Buyer-Order", "Checkout.receive-Checkout-Shop-Order",
                     "Checkout.choose-Checkout-Approved", "Checkout.offer-Checkout-Shop"] {
        assert!(names.contains(&required), "missing checked participant API {required}");
    }
    assert!(!names.contains(&"Checkout.send-Checkout-Bank-Order"));
    assert_eq!(run_source_with(&source, &mut StagingCx::default()).unwrap(), 0);
}

#[test]
fn definition_site_wire_helpers_survive_a_caller_alias_decoy() {
    let source = two_roles().replace("module Sale where", r#"module DecoyWire where
  suspend :: Bool
  suspend = False
  take :: Bool
  take = True
module Sale where
  import DecoyWire as LocalWire
  dispatch-Flow-Buyer :: Bool
  dispatch-Flow-Buyer = False"#);
    run(&source);
}

#[test]
fn direct_native_calls_validate_roles_messages_and_branch_shapes() {
    let source = two_roles();
    for invalid in [source.replace("([(Buyer), (Shop)])", "([(Buyer)])"),
                    source.replace("([(Buyer), (Shop)])", "([(Buyer), (Buyer)])"),
                    source.replace("(Buyer, Shop, Order)", "(Buyer, Missing, Order)"),
                    source.replace(", Shop,\n", ", Missing,\n"),
                    source.replace("(Declined, ([(Shop, Buyer, Reason)]))", "(Declined, ([]))")] {
        refuses(&invalid, "invalid choreography");
    }
}

#[test]
fn tuple_constructors_first_used_during_staging_are_registered_again_for_runtime() {
    for width in 2..=5 {
        let types = vec!["Int"; width].join(", ");
        let values = (0..width).map(|index| if index + 1 == width { "42" } else { "1" }).collect::<Vec<_>>().join(", ");
        let names = (0..width).map(|index| format!("value{index}")).collect::<Vec<_>>();
        let fields = names.iter().map(|name| format!("Syntax.packCode (Stage.quoteInt {name})")).collect::<Vec<_>>().join(", ");
        let pattern = names.join(", ");
        run(&format!(r#"module Main where
  stage shape :: ({types})
  stage shape = ({values})
  macro output :: Code ({types})
  macro output = case shape of
    ({pattern}) -> Syntax.tupleCode [{fields}]
  main :: Int
  main = case output!() of
    ({pattern}) -> {}
"#, names.last().unwrap()));
    }
}

#[test]
fn self_messages_project_once_and_an_inactive_peer_keeps_zero_state() {
    let source = format!("{}{}", libraries(), r#"module LoopProtocol where
  import stage ChoreographySyntax
  generate do
    emit (ChoreographySyntax.declare!(Loop, ([(A), (B)]), ([(A, A, Tick)]), A,
      ([(Looping, ([(A, A, Again)])), (Elsewhere, ([(B, A, Other)]))])))
module Main where
  import LoopProtocol
  main :: Int
  main =
    let a = start-Loop-A () in
    let b = start-Loop-B () in
    case send-Loop-A-Tick a 1 of
      (aReady, _) -> case choose-Loop-Looping aReady of
        (aBranch, bChoice) -> case offer-Loop-B b bChoice of
          LoopLoopingB bBranch -> case send-Loop-Looping-A-Again aBranch 2 of
            (aDone, _) ->
              let aCount = close-Loop-Looping-A aDone in
              let bCount = close-Loop-Looping-B bBranch in
              if aCount == 2 && bCount == 0 then 42 else 0
          _ -> 0
"#);
    run(&source);
}
