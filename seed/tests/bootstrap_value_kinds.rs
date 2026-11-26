//! Value-parameter carriers participate in inference, scope and Core closure.
mod common;

#[test]
fn value_kinds_retain_carriers_and_do_not_invent_runtime_witnesses() {
    let dir = common::project(
        "value_kinds",
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
                "Cinch.Type.cinch",
                include_str!("../../compiler/Cinch.Type.cinch"),
            ),
            (
                "Cinch.Core.cinch",
                include_str!("../../compiler/Cinch.Core.cinch"),
            ),
            (
                "Main.cinch",
                include_str!("../../compiler/programs/Run.ValueKinds.cinch"),
            ),
        ],
    );
    let exe =
        common::build_executable(&dir, "Main.cinch", "value-kinds").expect("build carrier-aware type engine");
    for size in [0, 1, 7, 31, 257] {
        std::fs::write(dir.join("case.txt"), vec![b'x'; size]).unwrap();
        let output = std::process::Command::new(&exe)
            .current_dir(&dir)
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(0),
            "input length {size}; project {}\n{}\n{}",
            dir.display(),
            common::read_output(&dir, "value-kinds-report.txt"),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    common::cleanup(&dir);
}
