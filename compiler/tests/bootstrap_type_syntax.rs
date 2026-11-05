//! Check actual signature trees, so dropping a proof bound cannot look like
//! an improvement in the parser's unmodeled-node counter.
mod common;

#[test]
fn predicates_indices_and_refined_results_keep_their_structure() {
    let dir = common::project(
        "type_syntax",
        &[
            (
                "Bootstrap.Ast.cinch",
                include_str!("../../bootstrap/Bootstrap.Ast.cinch"),
            ),
            (
                "Bootstrap.Report.cinch",
                include_str!("../../bootstrap/Bootstrap.Report.cinch"),
            ),
            (
                "Bootstrap.Parse.cinch",
                include_str!("../../bootstrap/Bootstrap.Parse.cinch"),
            ),
            (
                "Bootstrap.Check.cinch",
                include_str!("../../bootstrap/Bootstrap.Check.cinch"),
            ),
            (
                "Main.cinch",
                include_str!("../../bootstrap/programs/Run.TypeSyntax.cinch"),
            ),
        ],
    );
    let code = common::run(&dir, "Main.cinch");
    common::cleanup(&dir);
    assert_eq!(code, 0, "syntax tree check failed: 1 parse, 2 modules, 3 mixed bounds/refined result, 4 empty context refusal, 5 index arguments, 6 nested predicate, 7 single bound, 8 unmodeled syntax, 9 checker refusal, 10/11 malformed syntax, 12 tuple checking, 13 handler tree, 14 declaration/import metadata, 15 refinement/quantifier/export metadata, 16 malformed declaration refusal, 17 effect rows/capability supply trees, 18 malformed rows/supplies, 19 structured row type arguments, 20 malformed row argument refusal");
}
