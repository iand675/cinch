mod common;

#[test]
fn module_registries_do_not_collide_or_leak_private_values() {
    let dir = common::project(
        "module_scopes",
        &[
            (
                "Cinch.Ast.cinch",
                include_str!("../../compiler/Cinch.Ast.cinch"),
            ),
            (
                "Cinch.Report.cinch",
                include_str!("../../compiler/Cinch.Report.cinch"),
            ),
            (
                "Cinch.Check.cinch",
                include_str!("../../compiler/Cinch.Check.cinch"),
            ),
            (
                "Cinch.Parse.cinch",
                include_str!("../../compiler/Cinch.Parse.cinch"),
            ),
            (
                "Cinch.Ir.cinch",
                include_str!("../../compiler/Cinch.Ir.cinch"),
            ),
            (
                "Main.cinch",
                include_str!("../../compiler/programs/Run.ModuleScopes.cinch"),
            ),
        ],
    );
    let code = common::run(&dir, "Main.cinch");
    common::cleanup(&dir);
    assert_eq!(code, 0, "module scope check: 1 private-name collision, 2 leaked private value, 3 import resolution guessed, 4 unqualified emission accepted");
}
