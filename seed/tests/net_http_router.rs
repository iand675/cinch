//! Integration tests for `Net.Http.Router`: startup-frozen flat-array compressed
//! radix trie HTTP router with static segments, `:param` captures, `*wildcard`
//! catch-alls, per-method trees (0..8), `405 Method Not Allowed` (`Allow` bitmask),
//! query-string stripping, and `#[no-alloc]` zero-allocation hot-path matching.
//!
//! Routed through `common::project` / `common::drive_with_profile` /
//! `common::check_interfaces` / `common::build_executable` so the exact same
//! suite runs against both the Rust seed and the Stage 3 self-hosted compiler.

mod common;

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{Mode, Package};
use std::process::Command;
use std::time::Instant;

const PRIM_ATOMIC: &str = include_str!("../prelude/Prim/Atomic.cinch");
const ENGINE_TYPES: &str = include_str!("../prelude/Engine/Types.cinch");
const NET_HTTP_ROUTER: &str = include_str!("../prelude/Net/Http/Router.cinch");
const STDLIB_PKG: &str = include_str!("../prelude/.cinchpkg");

fn project_with_router(tag: &str, main_src: &str) -> std::path::PathBuf {
    common::project(
        tag,
        &[
            ("Prim.Atomic.cinch", PRIM_ATOMIC),
            ("Engine.Types.cinch", ENGINE_TYPES),
            ("Net.Http.Router.cinch", NET_HTTP_ROUTER),
            ("Main.cinch", main_src),
        ],
    )
}

fn run_leak_checked(tag: &str, src: &str, expected: i32) -> (String, std::time::Duration) {
    let dir = project_with_router(tag, src);
    let exe = common::build_executable(&dir, "Main.cinch", "net-http-router-test")
        .unwrap_or_else(|e| panic!("{tag}: build failed: {e}"));
    let started = Instant::now();
    let out = Command::new(&exe)
        .env("CINCH_RC_DEBUG", "1")
        .env("CINCH_LEAK_CHECK", "1")
        .env("CINCH_HEAP_LIMIT", "128M")
        .output()
        .expect("run built binary");
    let elapsed = started.elapsed();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    assert_eq!(
        out.status.code(),
        Some(expected),
        "{tag}: expected exit {expected}, got {:?};\nstdout:\n{stdout}\nstderr:\n{stderr}",
        out.status.code()
    );
    assert!(
        stderr.contains("cinch-leak: live=0 immortal=0 leaked=0"),
        "{tag}: expected clean leak report; stderr:\n{stderr}"
    );
    common::cleanup(&dir);
    (stdout, elapsed)
}

#[test]
fn net_http_router_registered_in_stdlib_package_manifest() {
    let pkg = Package::parse(STDLIB_PKG).expect("parse seed/prelude/.cinchpkg");
    let entry = pkg
        .modules
        .iter()
        .find(|m| m.name == "Net.Http.Router")
        .expect("Net.Http.Router listed in seed/prelude/.cinchpkg modules");
    assert_eq!(entry.path, "Net/Http/Router.cinch");
    assert!(
        pkg.order.iter().any(|m| m == "Net.Http.Router"),
        "Net.Http.Router listed in seed/prelude/.cinchpkg order"
    );
}

#[test]
fn net_http_router_abi_layouts_and_no_alloc_check() {
    let src = r#"
module Main where
  import Data.Machine
  import Engine.Types
  import Net.Http.Router

  #[no-alloc]
  #[inline(never)]
  id-param :: ParamSpan -> ParamSpan
  id-param p = p

  #[no-alloc]
  #[inline(never)]
  id-match :: RouteMatch -> RouteMatch
  id-match m = m

  main :: Int
  main =
    let p = id-param (param-span-make 4 12) in
    let m = id-match (route-match-none ()) in
    if eq-u32 p.param-len 12 && eq-u8 m.match-kind route-not-found then 0 else 1
"#;

    let dir = project_with_router("net-http-router-abi", src);

    // 1. Check via `common::drive_with_profile` (routes through active stage frontend).
    common::drive_with_profile(
        Mode::Check,
        StageMode::Refresh,
        &dir,
        "Main.cinch",
        None,
        &TargetProfile::native(),
    )
    .expect("Net.Http.Router type & #[no-alloc] check must succeed");

    // 2. Verify `.cinchif` POD ABI layouts for ParamSpan (8B) and RouteMatch (8B).
    let ifaces = common::check_interfaces(&dir, "Main.cinch").expect("check_interfaces");
    let rmod = ifaces
        .iter()
        .find(|i| i.module == "Net.Http.Router")
        .expect("Net.Http.Router interface emitted");

    let layout_of = |name: &str| {
        rmod.layouts
            .iter()
            .find(|l| l.name == name)
            .unwrap_or_else(|| panic!("missing layout for {name} in Net.Http.Router"))
    };

    let param_l = layout_of("ParamSpan");
    assert_eq!(param_l.size, 8, "ParamSpan layout: {param_l:?}");
    assert_eq!(param_l.align, 4, "ParamSpan align: {param_l:?}");
    assert_eq!(param_l.fields.len(), 2);
    assert_eq!(param_l.fields[0].offset, 0);
    assert_eq!(param_l.fields[1].offset, 4);

    let match_l = layout_of("RouteMatch");
    assert_eq!(match_l.size, 8, "RouteMatch layout: {match_l:?}");
    assert_eq!(match_l.align, 4, "RouteMatch align: {match_l:?}");
    assert_eq!(match_l.fields.len(), 4);
    assert_eq!(match_l.fields[0].offset, 0);
    assert_eq!(match_l.fields[1].offset, 1);
    assert_eq!(match_l.fields[2].offset, 2);
    assert_eq!(match_l.fields[3].offset, 4);

    // 3. Verify LLVM IR register-passing for 8B POD ParamSpan & RouteMatch (`i64`).
    let ir_path = dir.join("program.ll");
    common::drive_with_profile(
        Mode::EmitIr,
        StageMode::Refresh,
        &dir,
        "Main.cinch",
        Some(ir_path.clone()),
        &TargetProfile::native(),
    )
    .expect("emit IR");
    let ir = std::fs::read_to_string(&ir_path).expect("read IR");
    let def_match = ir
        .lines()
        .find(|l| l.starts_with("define ") && l.contains("Main.id-match"))
        .expect("Main.id-match in IR");
    assert!(
        def_match.contains("i64"),
        "RouteMatch should pass/return in a single i64 register: {def_match}"
    );

    common::cleanup(&dir);
}

#[test]
fn net_http_router_120_route_github_k8s_suite_and_validation() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Core
  import Data.Machine
  import Data.Nat
  import Data.Result
  import Data.String
  import Engine.Types
  import Net.Http.Router
  import Prim.Atomic

  add-r :: Result String RouterBuilder -> U8 -> String -> U32 -> Result String RouterBuilder
  add-r res m pat hid = case res of
    Err e -> Err e
    Ok b -> router-add b m pat hid

  -- Build a realistic 120-route GitHub + Kubernetes REST API router.
  build-rest-routes :: Unit -> Result String RouterBuilder
  build-rest-routes _ =
    let r0 = Ok (router-builder-new ()) in
    -- Core & health/observability static routes with tight prefix splits (1..16)
    let r1 = add-r r0 route-method-get "/" 1 in
    let r2 = add-r r1 route-method-get "/health" 2 in
    let r3 = add-r r2 route-method-get "/healthz" 3 in
    let r4 = add-r r3 route-method-get "/ready" 4 in
    let r5 = add-r r4 route-method-get "/readyz" 5 in
    let r6 = add-r r5 route-method-get "/livez" 6 in
    let r7 = add-r r6 route-method-get "/metrics" 7 in
    let r8 = add-r r7 route-method-get "/version" 8 in
    let r9 = add-r r8 route-method-get "/openapi/v2" 9 in
    let r10 = add-r r9 route-method-get "/openapi/v3" 10 in
    let r11 = add-r r10 route-method-get "/api" 11 in
    let r12 = add-r r11 route-method-get "/api/v1" 12 in
    let r13 = add-r r12 route-method-get "/api/v1/us" 13 in
    let r14 = add-r r13 route-method-get "/api/v1/user" 14 in
    let r15 = add-r r14 route-method-get "/api/v1/users" 15 in
    let r16 = add-r r15 route-method-get "/api/v1/usage" 16 in
    -- GitHub Auth / Users / Orgs routes (17..38)
    let r17 = add-r r16 route-method-get "/user" 17 in
    let r18 = add-r r17 route-method-patch "/user" 18 in
    let r19 = add-r r18 route-method-get "/user/emails" 19 in
    let r20 = add-r r19 route-method-post "/user/emails" 20 in
    let r21 = add-r r20 route-method-delete "/user/emails" 21 in
    let r22 = add-r r21 route-method-get "/user/keys" 22 in
    let r23 = add-r r22 route-method-post "/user/keys" 23 in
    let r24 = add-r r23 route-method-get "/user/repos" 24 in
    let r25 = add-r r24 route-method-post "/user/repos" 25 in
    let r26 = add-r r25 route-method-get "/users" 26 in
    let r27 = add-r r26 route-method-get "/users/me" 27 in
    let r28 = add-r r27 route-method-get "/users/:username" 28 in
    let r29 = add-r r28 route-method-get "/users/:username/repos" 29 in
    let r30 = add-r r29 route-method-get "/users/:username/gists" 30 in
    let r31 = add-r r30 route-method-get "/users/:username/followers" 31 in
    let r32 = add-r r31 route-method-get "/users/:username/following" 32 in
    let r33 = add-r r32 route-method-get "/users/:username/starred" 33 in
    let r34 = add-r r33 route-method-get "/orgs/:org" 34 in
    let r35 = add-r r34 route-method-patch "/orgs/:org" 35 in
    let r36 = add-r r35 route-method-get "/orgs/:org/repos" 36 in
    let r37 = add-r r36 route-method-post "/orgs/:org/repos" 37 in
    let r38 = add-r r37 route-method-get "/orgs/:org/teams/:team/members/:username" 38 in
    -- GitHub Repos / Issues / Pulls / Commits / Releases (39..72)
    let r39 = add-r r38 route-method-get "/repos/official/featured" 39 in
    let r40 = add-r r39 route-method-get "/repos/:owner/:repo" 40 in
    let r41 = add-r r40 route-method-patch "/repos/:owner/:repo" 41 in
    let r42 = add-r r41 route-method-delete "/repos/:owner/:repo" 42 in
    let r43 = add-r r42 route-method-options "/repos/:owner/:repo" 43 in
    let r44 = add-r r43 route-method-get "/repos/:owner/:repo/issues" 44 in
    let r45 = add-r r44 route-method-post "/repos/:owner/:repo/issues" 45 in
    let r46 = add-r r45 route-method-get "/repos/:owner/:repo/issues/:num" 46 in
    let r47 = add-r r46 route-method-patch "/repos/:owner/:repo/issues/:num" 47 in
    let r48 = add-r r47 route-method-put "/repos/:owner/:repo/issues/:num" 48 in
    let r49 = add-r r48 route-method-delete "/repos/:owner/:repo/issues/:num" 49 in
    let r50 = add-r r49 route-method-get "/repos/:owner/:repo/issues/:num/comments" 50 in
    let r51 = add-r r50 route-method-post "/repos/:owner/:repo/issues/:num/comments" 51 in
    let r52 = add-r r51 route-method-get "/repos/:owner/:repo/issues/:num/comments/:cid" 52 in
    let r53 = add-r r52 route-method-patch "/repos/:owner/:repo/issues/:num/comments/:cid" 53 in
    let r54 = add-r r53 route-method-delete "/repos/:owner/:repo/issues/:num/comments/:cid" 54 in
    let r55 = add-r r54 route-method-get "/repos/:owner/:repo/issues/:num/labels" 55 in
    let r56 = add-r r55 route-method-post "/repos/:owner/:repo/issues/:num/labels" 56 in
    let r57 = add-r r56 route-method-get "/repos/:owner/:repo/pulls" 57 in
    let r58 = add-r r57 route-method-post "/repos/:owner/:repo/pulls" 58 in
    let r59 = add-r r58 route-method-get "/repos/:owner/:repo/pulls/:num" 59 in
    let r60 = add-r r59 route-method-patch "/repos/:owner/:repo/pulls/:num" 60 in
    let r61 = add-r r60 route-method-put "/repos/:owner/:repo/pulls/:num/merge" 61 in
    let r62 = add-r r61 route-method-get "/repos/:owner/:repo/pulls/:num/reviews/:rid" 62 in
    let r63 = add-r r62 route-method-get "/repos/:owner/:repo/commits" 63 in
    let r64 = add-r r63 route-method-get "/repos/:owner/:repo/commits/:sha" 64 in
    let r65 = add-r r64 route-method-get "/repos/:owner/:repo/commits/:sha/check-runs" 65 in
    let r66 = add-r r65 route-method-get "/repos/:owner/:repo/branches" 66 in
    let r67 = add-r r66 route-method-get "/repos/:owner/:repo/branches/:branch" 67 in
    let r68 = add-r r67 route-method-get "/repos/:owner/:repo/releases" 68 in
    let r69 = add-r r68 route-method-post "/repos/:owner/:repo/releases" 69 in
    let r70 = add-r r69 route-method-get "/repos/:owner/:repo/releases/latest" 70 in
    let r71 = add-r r70 route-method-get "/repos/:owner/:repo/releases/:id" 71 in
    let r72 = add-r r71 route-method-delete "/repos/:owner/:repo/releases/:id" 72 in
    -- Kubernetes Core v1 + Apps/Batch/Networking APIs (73..110)
    let r73 = add-r r72 route-method-get "/api/v1/nodes" 73 in
    let r74 = add-r r73 route-method-get "/api/v1/nodes/:name" 74 in
    let r75 = add-r r74 route-method-get "/api/v1/nodes/:name/status" 75 in
    let r76 = add-r r75 route-method-get "/api/v1/namespaces" 76 in
    let r77 = add-r r76 route-method-post "/api/v1/namespaces" 77 in
    let r78 = add-r r77 route-method-get "/api/v1/namespaces/:ns" 78 in
    let r79 = add-r r78 route-method-delete "/api/v1/namespaces/:ns" 79 in
    let r80 = add-r r79 route-method-get "/api/v1/namespaces/:ns/pods" 80 in
    let r81 = add-r r80 route-method-post "/api/v1/namespaces/:ns/pods" 81 in
    let r82 = add-r r81 route-method-get "/api/v1/namespaces/:ns/pods/:name" 82 in
    let r83 = add-r r82 route-method-head "/api/v1/namespaces/:ns/pods/:name" 83 in
    let r84 = add-r r83 route-method-put "/api/v1/namespaces/:ns/pods/:name" 84 in
    let r85 = add-r r84 route-method-patch "/api/v1/namespaces/:ns/pods/:name" 85 in
    let r86 = add-r r85 route-method-delete "/api/v1/namespaces/:ns/pods/:name" 86 in
    let r87 = add-r r86 route-method-get "/api/v1/namespaces/:ns/pods/:name/log" 87 in
    let r88 = add-r r87 route-method-post "/api/v1/namespaces/:ns/pods/:name/exec" 88 in
    let r89 = add-r r88 route-method-get "/api/v1/namespaces/:ns/services" 89 in
    let r90 = add-r r89 route-method-post "/api/v1/namespaces/:ns/services" 90 in
    let r91 = add-r r90 route-method-get "/api/v1/namespaces/:ns/services/:name" 91 in
    let r92 = add-r r91 route-method-get "/api/v1/namespaces/:ns/configmaps" 92 in
    let r93 = add-r r92 route-method-get "/api/v1/namespaces/:ns/configmaps/:name" 93 in
    let r94 = add-r r93 route-method-get "/api/v1/namespaces/:ns/secrets" 94 in
    let r95 = add-r r94 route-method-get "/api/v1/namespaces/:ns/secrets/:name" 95 in
    let r96 = add-r r95 route-method-get "/api/v1/namespaces/:ns/events" 96 in
    let r97 = add-r r96 route-method-get "/apis/apps/v1/namespaces/:ns/deployments" 97 in
    let r98 = add-r r97 route-method-post "/apis/apps/v1/namespaces/:ns/deployments" 98 in
    let r99 = add-r r98 route-method-get "/apis/apps/v1/namespaces/:ns/deployments/:name" 99 in
    let r100 = add-r r99 route-method-patch "/apis/apps/v1/namespaces/:ns/deployments/:name" 100 in
    let r101 = add-r r100 route-method-delete "/apis/apps/v1/namespaces/:ns/deployments/:name" 101 in
    let r102 = add-r r101 route-method-get "/apis/apps/v1/namespaces/:ns/deployments/:name/scale" 102 in
    let r103 = add-r r102 route-method-put "/apis/apps/v1/namespaces/:ns/deployments/:name/scale" 103 in
    let r104 = add-r r103 route-method-get "/apis/apps/v1/namespaces/:ns/deployments/:name/pods/:pod/logs/:container" 104 in
    let r105 = add-r r104 route-method-get "/apis/apps/v1/namespaces/:ns/statefulsets/:name" 105 in
    let r106 = add-r r105 route-method-get "/apis/apps/v1/namespaces/:ns/daemonsets/:name" 106 in
    let r107 = add-r r106 route-method-get "/apis/batch/v1/namespaces/:ns/jobs/:name" 107 in
    let r108 = add-r r107 route-method-get "/apis/batch/v1/namespaces/:ns/cronjobs/:name/status" 108 in
    let r109 = add-r r108 route-method-get "/apis/networking.k8s.io/v1/namespaces/:ns/ingresses/:name" 109 in
    let r110 = add-r r109 route-method-get "/apis/rbac.authorization.k8s.io/v1/clusterroles/:name" 110 in
    -- Static/Param/Wildcard precedence, catch-alls, CONNECT & TRACE (111..120)
    let r111 = add-r r110 route-method-get "/static/css/main.css" 111 in
    let r112 = add-r r111 route-method-get "/static/:dir/index.html" 112 in
    let r113 = add-r r112 route-method-get "/static/*filepath" 113 in
    let r114 = add-r r113 route-method-head "/static/*filepath" 114 in
    let r115 = add-r r114 route-method-get "/docs/*filepath" 115 in
    let r116 = add-r r115 route-method-get "/raw/:owner/:repo/:ref/*filepath" 116 in
    let r117 = add-r r116 route-method-get "/blobs/*digest" 117 in
    let r118 = add-r r117 route-method-connect "/proxy/:target" 118 in
    let r119 = add-r r118 route-method-trace "/debug/echo" 119 in
    add-r r119 route-method-get "/search/code" 120

  -- Verify a captured ParamSpan matches an expected string slice inside `path`.
  span-eq-str :: Bytes -> ParamSpan -> String -> Bool
  span-eq-str path sp expected =
    let eb = string-to-bytes expected in
    let elen = bytes-len-i64 eb in
    let slen = u32-to-i64 sp.param-len in
    if ne-i64 elen slen then False
    else span-bytes-eq path (u32-to-i64 sp.param-off) eb 0 elen

  span-bytes-eq :: Bytes -> I64 -> Bytes -> I64 -> I64 -> Bool
  span-bytes-eq a aoff b boff rem =
    if le-i64 rem 0 then True
    else if 0 <= aoff && aoff < bytes-len-i64 a && 0 <= boff && boff < bytes-len-i64 b then
      if eq-u8 (bytes-get-u8-at a aoff) (bytes-get-u8-at b boff) then
        span-bytes-eq a (wrapping-add-i64 aoff 1) b (wrapping-add-i64 boff 1) (wrapping-sub-i64 rem 1)
      else False
    else False

  check-match :: RouterFrozen -> I64 -> U8 -> String -> U32 -> U8 ->{MVar} Bool
  check-match router scratch m path exp-id exp-params =
    let pb = string-to-bytes path in
    let res = router-match-bytes router m pb scratch in
    eq-u8 res.match-kind route-matched
      && eq-u32 res.match-handler-id exp-id
      && eq-u8 res.match-param-count exp-params
      && router-allow-has res.match-allow-mask m

  check-not-found :: RouterFrozen -> I64 -> U8 -> String ->{MVar} Bool
  check-not-found router scratch m path =
    let pb = string-to-bytes path in
    let res = router-match-bytes router m pb scratch in
    eq-u8 res.match-kind route-not-found
      && eq-u16 res.match-allow-mask 0
      && eq-u32 res.match-handler-id 0

  check-method-not-allowed :: RouterFrozen -> I64 -> U8 -> String -> U16 ->{MVar} Bool
  check-method-not-allowed router scratch m path exp-mask =
    let pb = string-to-bytes path in
    let res = router-match-bytes router m pb scratch in
    eq-u8 res.match-kind route-method-not-allowed
      && eq-u16 res.match-allow-mask exp-mask
      && eq-u32 res.match-handler-id 0

  -- Test all builder validation & conflict rejection rules.
  test-validation :: Unit -> Int
  test-validation _ =
    let b0 = router-builder-new () in
    let ok1 = is-err (router-add b0 route-method-get "" 1) in
    let ok2 = is-err (router-add b0 route-method-get "no-leading-slash" 1) in
    let ok3 = is-err (router-add b0 9 "/bad-method" 1) in
    let ok4 = is-err (router-add b0 route-method-get "/users?q=1" 1) in
    let ok5 = is-err (router-add b0 route-method-get "/users/:" 1) in
    let ok6 = is-err (router-add b0 route-method-get "/files/*path/more" 1) in
    let ok7 = is-err (router-add b0 route-method-get "/a/:p1/:p2/:p3/:p4/:p5/:p6/:p7/:p8/:p9" 1) in
    let ok8 = is-err (router-add b0 route-method-get "/bad:mid" 1) in
    let ok9 = (case router-add b0 route-method-get "/dup/path" 1 of
      Err _ -> False
      Ok b1 -> is-err (router-add b1 route-method-get "/dup/path" 2)) in
    let ok10 = (case router-add b0 route-method-get "/items/:id/info" 1 of
      Err _ -> False
      Ok b1 -> is-err (router-add b1 route-method-get "/items/:item_id/meta" 2)) in
    let ok11 = (case router-add b0 route-method-get "/files/*a" 1 of
      Err _ -> False
      Ok b1 -> is-err (router-add b1 route-method-get "/files/*b" 2)) in
    if ok1 && ok2 && ok3 && ok4 && ok5 && ok6 && ok7 && ok8 && ok9 && ok10 && ok11 then 0 else 2

  test-suite :: RouterFrozen -> I64 ->{MVar} Int
  test-suite router scratch =
    -- 1. Static & prefix-split routes
    if not (check-match router scratch route-method-get "/" 1 0) then 101
    else if not (check-match router scratch route-method-get "/health" 2 0) then 102
    else if not (check-match router scratch route-method-get "/healthz" 3 0) then 103
    else if not (check-match router scratch route-method-get "/ready" 4 0) then 104
    else if not (check-match router scratch route-method-get "/readyz" 5 0) then 105
    else if not (check-match router scratch route-method-get "/api/v1/us" 13 0) then 106
    else if not (check-match router scratch route-method-get "/api/v1/user" 14 0) then 107
    else if not (check-match router scratch route-method-get "/api/v1/users" 15 0) then 108
    else if not (check-match router scratch route-method-get "/api/v1/usage" 16 0) then 109
    else if not (check-match router scratch route-method-trace "/debug/echo" 119 0) then 110
    else if not (check-match router scratch route-method-get "/search/code" 120 0) then 111
    else
      -- 2. Nested GitHub `:owner/:repo/issues/:num` + exact ParamSpan capture
      let gh-path = string-to-bytes "/repos/octocat/cinch-lang/issues/4201" in
      let gh-res = router-match-bytes router route-method-get gh-path scratch in
      let gh-ok = (
        eq-u8 gh-res.match-kind route-matched
        && eq-u32 gh-res.match-handler-id 46
        && eq-u8 gh-res.match-param-count 3
        && span-eq-str gh-path (router-param-get scratch 0) "octocat"
        && span-eq-str gh-path (router-param-get scratch 1) "cinch-lang"
        && span-eq-str gh-path (router-param-get scratch 2) "4201") in
      if not gh-ok then 11
      else
        -- 3. Deep 4-param Kubernetes route
        let k8s-path = string-to-bytes "/apis/apps/v1/namespaces/prod-us-east/deployments/edge-gateway/pods/gw-7f9b/logs/envoy" in
        let k8s-res = router-match-bytes router route-method-get k8s-path scratch in
        let k8s-ok = (
          eq-u8 k8s-res.match-kind route-matched
          && eq-u32 k8s-res.match-handler-id 104
          && eq-u8 k8s-res.match-param-count 4
          && span-eq-str k8s-path (router-param-get scratch 0) "prod-us-east"
          && span-eq-str k8s-path (router-param-get scratch 1) "edge-gateway"
          && span-eq-str k8s-path (router-param-get scratch 2) "gw-7f9b"
          && span-eq-str k8s-path (router-param-get scratch 3) "envoy") in
        if not k8s-ok then 12
        else
          -- 4. Static-over-param precedence AND full backtracking out of static prefix into :param
          let bt1-ok = check-match router scratch route-method-get "/users/me" 27 0 in
          let bt2-path = string-to-bytes "/users/me/repos" in
          let bt2-res = router-match-bytes router route-method-get bt2-path scratch in
          let bt2-ok = (
            eq-u8 bt2-res.match-kind route-matched
            && eq-u32 bt2-res.match-handler-id 29
            && eq-u8 bt2-res.match-param-count 1
            && span-eq-str bt2-path (router-param-get scratch 0) "me") in
          let bt3-ok = check-match router scratch route-method-get "/repos/official/featured" 39 0 in
          let bt4-path = string-to-bytes "/repos/official/featured/issues/7" in
          let bt4-res = router-match-bytes router route-method-get bt4-path scratch in
          let bt4-ok = (
            eq-u8 bt4-res.match-kind route-matched
            && eq-u32 bt4-res.match-handler-id 46
            && eq-u8 bt4-res.match-param-count 3
            && span-eq-str bt4-path (router-param-get scratch 0) "official"
            && span-eq-str bt4-path (router-param-get scratch 1) "featured"
            && span-eq-str bt4-path (router-param-get scratch 2) "7") in
          let bt5-ok = check-match router scratch route-method-get "/repos/alice/app/releases/latest" 70 2 in
          let bt6-path = string-to-bytes "/repos/alice/app/releases/v2.0" in
          let bt6-res = router-match-bytes router route-method-get bt6-path scratch in
          let bt6-ok = (
            eq-u8 bt6-res.match-kind route-matched
            && eq-u32 bt6-res.match-handler-id 71
            && eq-u8 bt6-res.match-param-count 3
            && span-eq-str bt6-path (router-param-get scratch 2) "v2.0") in
          if not (bt1-ok && bt2-ok && bt3-ok && bt4-ok && bt5-ok && bt6-ok) then 13
          else
            -- 5. Priority static > :param > *wildcard + nested param+wildcard
            let w1-ok = check-match router scratch route-method-get "/static/css/main.css" 111 0 in
            let w2-path = string-to-bytes "/static/css/index.html" in
            let w2-res = router-match-bytes router route-method-get w2-path scratch in
            let w2-ok = (
              eq-u8 w2-res.match-kind route-matched
              && eq-u32 w2-res.match-handler-id 112
              && eq-u8 w2-res.match-param-count 1
              && span-eq-str w2-path (router-param-get scratch 0) "css") in
            let w3-path = string-to-bytes "/static/css/theme/dark.css" in
            let w3-res = router-match-bytes router route-method-get w3-path scratch in
            let w3-ok = (
              eq-u8 w3-res.match-kind route-matched
              && eq-u32 w3-res.match-handler-id 113
              && eq-u8 w3-res.match-param-count 1
              && span-eq-str w3-path (router-param-get scratch 0) "css/theme/dark.css") in
            let w4-path = string-to-bytes "/raw/torvalds/linux/master/net/ipv4/tcp.c" in
            let w4-res = router-match-bytes router route-method-get w4-path scratch in
            let w4-ok = (
              eq-u8 w4-res.match-kind route-matched
              && eq-u32 w4-res.match-handler-id 116
              && eq-u8 w4-res.match-param-count 4
              && span-eq-str w4-path (router-param-get scratch 0) "torvalds"
              && span-eq-str w4-path (router-param-get scratch 1) "linux"
              && span-eq-str w4-path (router-param-get scratch 2) "master"
              && span-eq-str w4-path (router-param-get scratch 3) "net/ipv4/tcp.c") in
            if not (w1-ok && w2-ok && w3-ok && w4-ok) then 14
            else
              -- 6. Query-string stripping (`?foo=bar`) on static, param, and wildcard routes
              let q1-ok = check-match router scratch route-method-get "/?probe=1" 1 0 in
              let q2-ok = check-match router scratch route-method-get "/healthz?verbose=true" 3 0 in
              let q3-path = string-to-bytes "/repos/octocat/cinch/issues/99?state=open&per_page=50" in
              let q3-res = router-match-bytes router route-method-get q3-path scratch in
              let q3-ok = (
                eq-u8 q3-res.match-kind route-matched
                && eq-u32 q3-res.match-handler-id 46
                && eq-u8 q3-res.match-param-count 3
                && span-eq-str q3-path (router-param-get scratch 2) "99") in
              let q4-path = string-to-bytes "/docs/guide/getting-started.md?lang=en" in
              let q4-res = router-match-bytes router route-method-get q4-path scratch in
              let q4-ok = (
                eq-u8 q4-res.match-kind route-matched
                && eq-u32 q4-res.match-handler-id 115
                && eq-u8 q4-res.match-param-count 1
                && span-eq-str q4-path (router-param-get scratch 0) "guide/getting-started.md") in
              if not (q1-ok && q2-ok && q3-ok && q4-ok) then 15
              else
                -- 7. 404 Not Found cases
                let nf-ok = (
                  check-not-found router scratch route-method-get "/does/not/exist"
                  && check-not-found router scratch route-method-get "/users//repos"
                  && check-not-found router scratch route-method-get "/repos/only-owner"
                  && check-not-found router scratch route-method-get "/static/"
                  && check-not-found router scratch route-method-get "?only-query=1"
                  && check-not-found router scratch route-method-post "/unknown/path") in
                if not nf-ok then 16
                else
                  -- 8. 405 Method Not Allowed with exact `allow-mask`
                  -- `/healthz`: GET(0) => bit 0 = 1
                  let m1 = check-method-not-allowed router scratch route-method-post "/healthz" 1 in
                  -- `/repos/:owner/:repo/issues`: GET(0) + POST(2) => 1 | 4 = 5
                  let m2 = check-method-not-allowed router scratch route-method-delete "/repos/a/b/issues" 5 in
                  -- `/repos/:owner/:repo/issues/:num`: GET(0) + PUT(3) + DELETE(4) + PATCH(5) => 1 | 8 | 16 | 32 = 57
                  let m3 = check-method-not-allowed router scratch route-method-post "/repos/a/b/issues/123" 57 in
                  -- `/api/v1/namespaces/:ns/pods/:name`: GET(0) + HEAD(1) + PUT(3) + DELETE(4) + PATCH(5) => 1|2|8|16|32 = 59
                  let m4 = check-method-not-allowed router scratch route-method-post "/api/v1/namespaces/default/pods/web-0" 59 in
                  -- `/static/*filepath`: GET(0) + HEAD(1) => 1 | 2 = 3
                  let m5 = check-method-not-allowed router scratch route-method-put "/static/img/logo.png" 3 in
                  -- `/proxy/:target`: CONNECT(7) => 128
                  let m6 = check-method-not-allowed router scratch route-method-get "/proxy/example.com:443" 128 in
                  if not (m1 && m2 && m3 && m4 && m5 && m6) then 17
                  else if eq-u32 (router-node-count router) 0 || eq-u32 (router-pool-len router) 0 then 18
                  else 0

  main :: Unit ->{IO, MVar} Int
  main _ =
    let v = test-validation () in
    if v /= 0 then v
    else
      case build-rest-routes () of
        Err _ -> 3
        Ok builder ->
          case router-freeze builder of
            Err _ -> 4
            Ok router ->
              let scratch-u64 = engine-dma-alloc 4096 4096 in
              if eq-u64 scratch-u64 0 then
                let _ = router-destroy router in
                5
              else
                let scratch = wrapping-u64-to-i64 scratch-u64 in
                let rc = test-suite router scratch in
                let _ = engine-dma-free scratch-u64 4096 in
                let _ = router-destroy router in
                rc
"#;

    run_leak_checked("net-http-router-120-suite", src, 0);
}

#[test]
fn net_http_router_200k_hot_loop_zero_alloc_zero_atomic_rc() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Core
  import Data.Machine
  import Data.Result
  import Data.String
  import Engine.Types
  import Net.Http.Router
  import Prim.Atomic

  add-r :: Result String RouterBuilder -> U8 -> String -> U32 -> Result String RouterBuilder
  add-r res m pat hid = case res of
    Err e -> Err e
    Ok b -> router-add b m pat hid

  build-bench-router :: Unit -> Result String RouterBuilder
  build-bench-router _ =
    let r0 = Ok (router-builder-new ()) in
    let r1 = add-r r0 route-method-get "/" 1 in
    let r2 = add-r r1 route-method-get "/healthz" 2 in
    let r3 = add-r r2 route-method-get "/metrics" 3 in
    let r4 = add-r r3 route-method-get "/api/v1/users" 4 in
    let r5 = add-r r4 route-method-get "/api/v1/usage" 5 in
    let r6 = add-r r5 route-method-get "/users/me" 6 in
    let r7 = add-r r6 route-method-get "/users/:username/repos" 7 in
    let r8 = add-r r7 route-method-get "/repos/:owner/:repo/issues" 8 in
    let r9 = add-r r8 route-method-post "/repos/:owner/:repo/issues" 9 in
    let r10 = add-r r9 route-method-get "/repos/:owner/:repo/issues/:num" 10 in
    let r11 = add-r r10 route-method-patch "/repos/:owner/:repo/issues/:num" 11 in
    let r12 = add-r r11 route-method-delete "/repos/:owner/:repo/issues/:num" 12 in
    let r13 = add-r r12 route-method-get "/apis/apps/v1/namespaces/:ns/deployments/:name/pods/:pod/logs/:container" 13 in
    let r14 = add-r r13 route-method-get "/static/css/main.css" 14 in
    let r15 = add-r r14 route-method-get "/static/*filepath" 15 in
    add-r r15 route-method-get "/raw/:owner/:repo/:ref/*filepath" 16

  copy-str-to-dma :: U64 -> String ->{MVar} U32
  copy-str-to-dma dst s =
    let b = string-to-bytes s in
    let n = bytes-len-i64 b in
    let _ = copy-loop dst b 0 n in
    wrapping-i64-to-u32 n

  copy-loop :: U64 -> Bytes -> I64 -> I64 ->{MVar} Unit
  copy-loop dst b i n =
    if ge-i64 i n then ()
    else if 0 <= i && i < bytes-len-i64 b then
      let _ = raw-store-u8 (wrapping-u64-to-i64 (wrapping-add-u64 dst (wrapping-i64-to-u64 i))) (bytes-get-u8-at b i) in
      copy-loop dst b (wrapping-add-i64 i 1) n
    else ()

  -- 100% #[no-alloc] hot loop performing 200,000 mixed route lookups (50,000 rounds x 4):
  --   1. Deep nested 3-param GitHub route with query string
  --   2. Deep nested 4-param Kubernetes route
  --   3. Wildcard catch-all route with backtracking past static sibling
  --   4. 405 Method Not Allowed fallback with Allow bitmask check
  #[no-alloc]
  bench-loop
    :: RouterFrozen
    -> I64
    -> I64 -> U32
    -> I64 -> U32
    -> I64 -> U32
    -> I64 -> U32
    -> U32
    -> U64
    ->{MVar} U64
  bench-loop router scratch p0 l0 p1 l1 p2 l2 p3 l3 rem acc =
    if eq-u32 rem 0 then acc
    else
      let m0 = router-match-dma router route-method-get p0 l0 scratch in
      let sp0 = router-param-get scratch 2 in
      let m1 = router-match-dma router route-method-get p1 l1 scratch in
      let sp1 = router-param-get scratch 3 in
      let m2 = router-match-dma router route-method-get p2 l2 scratch in
      let m3 = router-match-dma router route-method-post p3 l3 scratch in
      let s0 = wrapping-add-u64 (u32-to-u64 m0.match-handler-id) (u32-to-u64 sp0.param-len) in
      let s1 = wrapping-add-u64 (u32-to-u64 m1.match-handler-id) (u32-to-u64 sp1.param-len) in
      let s2 = wrapping-add-u64 (u32-to-u64 m2.match-handler-id) (u16-to-u64 m3.match-allow-mask) in
      let step = wrapping-add-u64 s0 (wrapping-add-u64 s1 s2) in
      bench-loop router scratch p0 l0 p1 l1 p2 l2 p3 l3 (wrapping-sub-u32 rem 1) (wrapping-add-u64 acc step)

  main :: Unit ->{IO, MVar} Int
  main _ =
    case build-bench-router () of
      Err _ -> 1
      Ok builder ->
        case router-freeze builder of
          Err _ -> 2
          Ok router ->
            let dma = engine-dma-alloc 4096 4096 in
            if eq-u64 dma 0 then
              let _ = router-destroy router in
              3
            else
              -- Layout in DMA scratch page:
              --   +0    : 64-byte params-out scratch
              --   +64   : path 0 (GitHub 3-param + query)
              --   +256  : path 1 (Kubernetes 4-param)
              --   +512  : path 2 (Wildcard with static sibling backtrack)
              --   +768  : path 3 (405 Method Not Allowed on GET+PATCH+DELETE)
              let scratch = wrapping-u64-to-i64 dma in
              let p0 = wrapping-u64-to-i64 (wrapping-add-u64 dma 64) in
              let p1 = wrapping-u64-to-i64 (wrapping-add-u64 dma 256) in
              let p2 = wrapping-u64-to-i64 (wrapping-add-u64 dma 512) in
              let p3 = wrapping-u64-to-i64 (wrapping-add-u64 dma 768) in
              let l0 = copy-str-to-dma (wrapping-add-u64 dma 64) "/repos/octocat/cinch/issues/4201?state=open&page=2" in
              let l1 = copy-str-to-dma (wrapping-add-u64 dma 256) "/apis/apps/v1/namespaces/prod/deployments/api/pods/p0/logs/app" in
              let l2 = copy-str-to-dma (wrapping-add-u64 dma 512) "/static/css/theme/dark.css" in
              let l3 = copy-str-to-dma (wrapping-add-u64 dma 768) "/repos/octocat/cinch/issues/99" in
              -- Reset per-thread alloc & atomic RC counters immediately before the 200,000-lookup hot loop!
              let _ = engine-thread-counters-reset () in
              let acc = bench-loop router scratch p0 l0 p1 l1 p2 l2 p3 l3 50000 0 in
              let allocs = engine-thread-alloc-count () in
              let atomic-rcs = engine-thread-atomic-rc-count () in
              let _ = engine-dma-free dma 4096 in
              let _ = router-destroy router in
              -- Expected per-round step:
              --   m0.hid(10) + sp0.len(4: "4201") + m1.hid(13) + sp1.len(3: "app")
              --   + m2.hid(15) + m3.allow(GET|DELETE|PATCH = 1|16|32 = 49) = 94
              --   50000 * 94 = 4,700,000
              if ne-u64 acc 4700000 then 10
              else if ne-u64 allocs 0 then 11
              else if ne-u64 atomic-rcs 0 then 12
              else 0
"#;

    let (_stdout, elapsed) = run_leak_checked("net-http-router-200k-bench", src, 0);
    let secs = elapsed.as_secs_f64();
    let lookups_per_sec = 200_000.0 / secs.max(1e-9);
    let ns_per_lookup = (secs * 1e9) / 200_000.0;
    eprintln!(
        "Net.Http.Router 200,000 mixed lookups (incl. process+setup): {:.2} ms ({:.2}M/s, {:.1} ns/lookup), allocs=0, atomic_rcs=0, leaked=0",
        secs * 1000.0,
        lookups_per_sec / 1e6,
        ns_per_lookup
    );
}
