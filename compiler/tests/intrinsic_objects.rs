//! Public intrinsic values retain callable source identities in library objects.
mod common;

#[test]
fn checked_intrinsic_exports_are_object_roots_with_exact_word_classes() {
    let dir = common::project("intrinsic_object_roots", &[("Library.cinch", r#"module Library where
  export compare, negate
  foreign import ccall "cinch.runtime.bytesCompare" compare :: Bytes -> Bytes -> I64
  foreign import ccall "cinch.runtime.intNeg" negate :: Int -> Int
"#)]);
    let object = dir.join("library.o");
    let roots = cinch::specialize::parse_object_roots(r#"{"schema":1,"units":[{"symbol":"Library.compare","classes":""},{"symbol":"Library.negate","classes":""}]}"#).unwrap();
    let opts = cinch::driver::DriverOptions { object_roots: roots, ..Default::default() };
    common::drive_opts(cinch::driver::Mode::EmitObject, &dir, "Library.cinch", Some(object.clone()), &opts).unwrap();
    let metadata = std::fs::read_to_string(object.with_extension("abi.json")).unwrap();
    for (name, params, result) in [("Library.compare", "[\"O\",\"O\"]", "R"), ("Library.negate", "[\"T\"]", "T")] {
        // Unit rows are flat objects in the actual canonical ABI artifact.
        let key = format!("\"symbol\":\"{name}\"");
        let row = metadata.split('{').filter_map(|row| row.split('}').next())
            .find(|row| row.contains(&key) && row.contains("\"closure_entry\":")).unwrap();
        assert!(row.contains(&format!("\"params\":{params}")), "{row}");
        assert!(row.contains(&format!("\"result\":\"{result}\"")), "{row}");
        assert!(row.contains(&format!("\"closure_entry\":\"{name}.object-closure\"")), "{row}");
    }
}
