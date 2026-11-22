//! Certificates are checked in Cinch against post-compilation inputs.
mod common;

#[test]
fn arithmetic_search_requires_independently_rechecked_evidence() {
    let dir = common::project(
        "proof",
        &[
            (
                "Bootstrap.Proof.cinch",
                include_str!("../../bootstrap/Bootstrap.Proof.cinch"),
            ),
            (
                "Main.cinch",
                include_str!("../../bootstrap/programs/Run.Proof.cinch"),
            ),
        ],
    );
    let exe = common::build(&dir, "Main.cinch", "proof").expect("build Cinch certificate checker");
    for input in ["", "a", "integer", "three term byte bounds"] {
        std::fs::write(
            dir.join("case.txt"),
            format!(r#"{{"mode":"selftest","input":{}}}"#, input.len()),
        )
        .unwrap();
        let out = std::process::Command::new(&exe)
            .current_dir(&dir)
            .output()
            .unwrap();
        assert_eq!(
            out.status.code(),
            Some(0),
            "input {input:?}; project {}\n{}\n{}",
            dir.display(),
            common::read_output(&dir, "proof-report.txt"),
            String::from_utf8_lossy(&out.stderr)
        );
    }
    // These complete contexts and certificates exist only after compilation.
    // The Cinch executable must decode them, then reconstruct each cited row.
    let x = r#"["var","x"]"#;
    let y = r#"["var","y"]"#;
    let seven = r#"["int",7]"#;
    let eight = r#"["int",8]"#;
    let bound = |a: &str, b: &str| format!(r#"["le",{a},{b}]"#);
    let cert = r#"["refute",["farkas",[[1,["hyp",0,0]],[1,["goal",0]]]]]"#;
    let packet = |facts: &[String], goal: &str, proof: &str| {
        format!(
            r#"{{"mode":"check","facts":[{}],"goal":{goal},"proof":{proof}}}"#,
            facts.join(",")
        )
    };
    let dx = bound(x, seven);
    let mut packets = vec![
        ("actual certificate", packet(&[dx.clone()], &dx, cert), 0),
        ("changed fact", packet(&[bound(x, eight)], &dx, cert), 4),
        (
            "changed binding identity",
            packet(&[bound(y, seven)], &dx, cert),
            4,
        ),
        (
            "changed goal",
            packet(&[dx.clone()], &format!(r#"["lt",{x},{seven}]"#), cert),
            4,
        ),
        (
            "negative multiplier",
            packet(
                &[bound(x, seven), bound(x, eight)],
                "false",
                r#"["refute",["farkas",[[1,["hyp",0,0]],[-1,["hyp",1,0]]]]]"#,
            ),
            4,
        ),
        (
            "zero multiplier",
            packet(
                &["false".into()],
                "true",
                r#"["refute",["farkas",[[1,["hyp",0,0]],[0,["goal",0]]]]]"#,
            ),
            4,
        ),
        (
            "missing conjunct",
            packet(
                &[dx.clone()],
                &dx,
                r#"["refute",["farkas",[[1,["hyp",0,1]],[1,["goal",0]]]]]"#,
            ),
            4,
        ),
        (
            "absent branch",
            packet(&[], "true", r#"["refute",["farkas",[[1,["case",0,0]]]]]"#),
            4,
        ),
        (
            "negative citation index",
            packet(
                &[dx.clone()],
                &dx,
                r#"["refute",["farkas",[[1,["hyp",-1,0]],[1,["goal",0]]]]]"#,
            ),
            2,
        ),
        (
            "unexpected proof tag",
            packet(&[], "true", r#"["trust-me"]"#),
            2,
        ),
        (
            "false conjunction branch",
            packet(
                &[dx.clone()],
                &format!(r#"["and",{dx},["lt",{x},{seven}]]"#),
                &format!(r#"["both",{cert},{cert}]"#),
            ),
            4,
        ),
        (
            "search actual context",
            format!(r#"{{"mode":"search","facts":[{dx}],"goal":{dx}}}"#),
            0,
        ),
        (
            "search does not assume natural",
            format!(r#"{{"mode":"search","facts":[],"goal":["le",["int",0],{x}]}}"#),
            4,
        ),
    ];
    // An exact lexical hypothesis must remain usable when unrelated facts
    // exceed the bounded arithmetic elimination budget. Citations still
    // refer to the original context, including a conjunct at a large index.
    let mut crowded: Vec<String> = (0..300)
        .map(|i| bound(&format!(r#"["var","unrelated-{i}"]"#), seven))
        .collect();
    let unrelated = crowded.clone();
    crowded.push(format!(r#"["and",["le",{y},{eight}],{dx}]"#));
    packets.push((
        "crowded context retains direct bound",
        format!(r#"{{"mode":"search","facts":[{}],"goal":{dx}}}"#, crowded.join(",")),
        0,
    ));
    packets.push((
        "crowded context does not invent a strict bound",
        format!(r#"{{"mode":"search","facts":[{}],"goal":["lt",{x},{seven}]}}"#, crowded.join(",")),
        4,
    ));
    packets.push((
        "crowded context does not invent a missing bound",
        format!(r#"{{"mode":"search","facts":[{}],"goal":{dx}}}"#, unrelated.join(",")),
        4,
    ));
    let huge = r#"["int",340282366920938463463374607431768211456]"#;
    let huge_bound = bound(x, huge);
    packets.push((
        "integer larger than i128",
        packet(&[huge_bound.clone()], &huge_bound, cert),
        0,
    ));
    let disjunction = format!(r#"["or",["lt",{x},["int",0]],["gt",{x},["int",0]]]"#);
    let goal = format!(r#"["ne",{x},["int",0]]"#);
    let left = r#"["farkas",[[1,["case",0,0]],[1,["goal",1]]]]"#;
    let right = r#"["farkas",[[1,["case",0,0]],[1,["goal",0]]]]"#;
    packets.push((
        "both split contexts",
        packet(
            &[disjunction.clone()],
            &goal,
            &format!(r#"["refute",["split",["hyp",0,0],{left},{right}]]"#),
        ),
        0,
    ));
    packets.push((
        "sibling branch certificate",
        packet(
            &[disjunction],
            &goal,
            &format!(r#"["refute",["split",["hyp",0,0],{left},{left}]]"#),
        ),
        4,
    ));
    for (name, input, status) in packets {
        std::fs::write(dir.join("case.txt"), input).unwrap();
        let out = std::process::Command::new(&exe)
            .current_dir(&dir)
            .output()
            .unwrap();
        assert_eq!(
            out.status.code(),
            Some(status),
            "packet {name}; project {}\n{}\n{}",
            dir.display(),
            common::read_output(&dir, "proof-report.txt"),
            String::from_utf8_lossy(&out.stderr)
        );
    }
    common::cleanup(&dir);
}
