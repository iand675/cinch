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
    let input = dir.join("Library.cinch");
    let roots = cinch::specialize::parse_object_roots(r#"{"schema":1,"units":[{"symbol":"Library.compare","classes":""},{"symbol":"Library.negate","classes":""}]}"#).unwrap();
    std::thread::Builder::new().stack_size(64 * 1024 * 1024).spawn({
        let object = object.clone();
        move || {
            let opts = cinch::driver::DriverOptions { object_roots: roots, ..Default::default() };
            cinch::driver::driver_with_profile_opts(cinch::driver::Mode::EmitObject, cinch::driver::StageMode::Refresh,
                &input, Some(&object), &cinch::core::TargetProfile::native(), &opts).unwrap();
        }
    }).unwrap().join().unwrap();
    let metadata = cinch::interface::parse_json(&std::fs::read_to_string(object.with_extension("abi.json")).unwrap()).unwrap();
    let fields = metadata.as_object().unwrap();
    let units = fields["units"].as_array().unwrap();
    for (name, params, result) in [("Library.compare", vec!["O", "O"], "R"), ("Library.negate", vec!["T"], "T")] {
        let unit = units.iter().map(|u| u.as_object().unwrap()).find(|u| u["symbol"].as_str().unwrap() == name).unwrap();
        assert_eq!(unit["params"].as_array().unwrap().iter().map(|v| v.as_str().unwrap()).collect::<Vec<_>>(), params);
        assert_eq!(unit["result"].as_str().unwrap(), result);
        assert_eq!(unit["closure_entry"].as_str().unwrap(), format!("{name}.object-closure"));
    }
}
