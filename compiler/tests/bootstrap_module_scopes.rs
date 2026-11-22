mod common;

#[test]
fn module_registries_do_not_collide_or_leak_private_values() {
    let dir = common::project(
        "module_scopes",
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
                "Bootstrap.Check.cinch",
                include_str!("../../bootstrap/Bootstrap.Check.cinch"),
            ),
            (
                "Bootstrap.Parse.cinch",
                include_str!("../../bootstrap/Bootstrap.Parse.cinch"),
            ),
            (
                "Bootstrap.Ir.cinch",
                include_str!("../../bootstrap/Bootstrap.Ir.cinch"),
            ),
            (
                "Main.cinch",
                include_str!("../../bootstrap/programs/Run.ModuleScopes.cinch"),
            ),
        ],
    );
    let code = common::run(&dir, "Main.cinch");
    common::cleanup(&dir);
    assert_eq!(code, 0, "module scope check: 1 private-name collision, 2 leaked private value, 3 import resolution guessed, 4 unqualified emission accepted");
}
