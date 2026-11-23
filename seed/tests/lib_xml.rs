//! XML decoding and encoding against the reference SDK (`tooling/xml_vectors.go`): documents
//! decode to the reference tokens (namespace-translated, and raw) up to the reference error
//! message and line; token sequences encode to the reference bytes with and without indentation,
//! including generated attribute prefixes, or fail with the reference error; escaping matches.
//! Element trees are built, queried and rendered back.
mod common;

use serde_json::Value;

/// Hex of a string's UTF-8 bytes.
fn hex(s: &str) -> String {
    s.bytes().map(|b| format!("{b:02x}")).collect()
}

/// A Cinch string built from hex.
fn text(s: &str) -> String {
    format!("(t \"{}\")", hex(s))
}

/// A Cinch token description.
fn token(t: &Value) -> String {
    let attrs = t["attrs"].as_array().unwrap().iter().map(|a| {
        let a = a.as_array().unwrap();
        format!("({}, {}, {})", text(a[0].as_str().unwrap()), text(a[1].as_str().unwrap()), text(a[2].as_str().unwrap()))
    }).collect::<Vec<_>>();
    format!("(Tk {:?} {} {} [{}] (b \"{}\") {})", t["kind"].as_str().unwrap(), text(t["space"].as_str().unwrap()), text(t["local"].as_str().unwrap()), attrs.join(", "), t["data"].as_str().unwrap(), text(t["target"].as_str().unwrap()))
}

/// A Cinch token list.
fn tokens(v: &Value) -> String {
    format!("[{}]", v.as_array().unwrap().iter().map(token).collect::<Vec<_>>().join(", "))
}

/// Compile and run one batch of fixture rows; the exit status is the first failing row, 200 if none.
fn run_batch(rows: &[String]) {
    let dir = common::project("xml", &[
        ("Text.Xml.cinch", include_str!("../prelude/Text/Xml.cinch")),
        ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
        ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
        ("Text.Binary.cinch", include_str!("../prelude/Text/Binary.cinch")),
        ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ("Main.cinch", &format!("{HELPERS}\n  -- | Every row in this batch.\n  main :: Int\n  main = first [{}] 0\n", rows.join(",\n    "))),
    ]);
    assert_eq!(common::run(&dir, "Main.cinch"), 200, "the exit status is the first failing row index in the batch; 200 means every row passed");
    common::cleanup(&dir);
}

/// The fixture.
fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/xml-vectors.json")).unwrap()
}

/// Documents decode to the reference tokens and errors.
#[test]
fn xml_decodes_reference_tokens_and_errors() {
    let f = fixture();
    let rows = f["decode"].as_array().unwrap().iter().map(|r| format!("dec {} (b \"{}\") {} {}", if r["raw"].as_bool().unwrap() { "True" } else { "False" }, r["input"].as_str().unwrap(), tokens(&r["tokens"]), text(r["error"].as_str().unwrap()))).collect::<Vec<_>>();
    for batch in rows.chunks(35) {
        run_batch(batch);
    }
}

/// Token sequences encode to the reference bytes or errors; escaping matches; trees round-trip.
#[test]
fn xml_encodes_reference_bytes_escapes_and_builds_trees() {
    let f = fixture();
    let mut rows = f["encode"].as_array().unwrap().iter().map(|r| format!("enc {} {} {} (b \"{}\") {}", tokens(&r["tokens"]), text(r["prefix"].as_str().unwrap()), text(r["indent"].as_str().unwrap()), r["output"].as_str().unwrap(), text(r["error"].as_str().unwrap()))).collect::<Vec<_>>();
    rows.extend(f["escape"].as_array().unwrap().iter().map(|r| format!("esc (b {:?}) (b {:?})", r[0].as_str().unwrap(), r[1].as_str().unwrap())));
    rows.push("trees ()".to_string());
    for batch in rows.chunks(30) {
        run_batch(batch);
    }
}

/// The span scanners lower for 64-bit ARM, WebAssembly and a big-endian target.
#[test]
fn xml_kernels_compile_for_arm_wasm_and_big_endian_targets() {
    let directory = tempfile::tempdir().unwrap();
    for (triple, arch, endian) in [
        ("aarch64-unknown-linux-gnu", "aarch64", "little"),
        ("wasm32-unknown-unknown", "wasm32", "little"),
        ("powerpc64-unknown-linux-gnu", "powerpc64", "big"),
    ] {
        let profile = cinch::driver::target_profile(&cinch::driver::DriverOptions { target: Some(triple.into()), ..Default::default() })
            .unwrap_or_else(|_| {
                let mut profile = cinch::core::TargetProfile::native();
                profile.triple = triple.into();
                profile.arch = arch.into();
                profile.endian = endian.into();
                profile
            });
        let project = directory.path().join(arch);
        std::fs::create_dir_all(&project).unwrap();
        for (name, text) in [
            ("Text.Xml", include_str!("../prelude/Text/Xml.cinch")),
            ("Data.Bytes.Build", include_str!("../prelude/Data/Bytes/Build.cinch")),
        ] {
            std::fs::write(project.join(format!("{name}.cinch")), text).unwrap();
        }
        std::fs::write(project.join("Main.cinch"), "module Main where\n  import Text.Xml\n  -- | Reach the kernels.\n  main :: Int\n  main = case xml-tokens (xml-escape-text (bytes-new 20)) of\n    Ok ts -> 0\n    Err e -> 1\n").unwrap();
        let ir_path = project.join("program.ll");
        let entry = project.join("Main.cinch");
        let ir = std::thread::Builder::new()
            .stack_size(256 * 1024 * 1024)
            .spawn(move || {
                cinch::driver::driver_with_profile_opts(cinch::driver::Mode::EmitIr, cinch::check::StageMode::Refresh, &entry, Some(&ir_path), &profile, &cinch::driver::DriverOptions::default())
                    .map(|_| std::fs::read_to_string(&ir_path).unwrap())
            })
            .unwrap()
            .join()
            .unwrap()
            .unwrap();
        assert!(ir.contains("<16 x i8>") && ir.contains("@llvm.vector.reduce.umin.v16i8"));
        let output = std::process::Command::new(std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into()))
            .args(["-target", triple, "-O2", "-c", "-x", "ir"])
            .arg(project.join("program.ll"))
            .arg("-o")
            .arg(project.join("program.o"))
            .output()
            .unwrap();
        assert!(output.status.success(), "{triple}: {}", String::from_utf8_lossy(&output.stderr));
    }
}

/// Every public and private declaration has canonical module-owned documentation.
#[test]
fn xml_declarations_have_canonical_documentation() {
    common::assert_documented("Text.Xml", include_str!("../prelude/Text/Xml.cinch"));
}

/// Checked consumer helpers.
const HELPERS: &str = r#"-- | Reference XML vector consumers.
module Main where
  import Text.Xml
  import Text.Hex
  import Data.Bytes.Build
  -- | A token description: kind, namespace, local name, attributes, data and target.
  data Tk where
    -- | Fields in the order listed above.
    Tk :: String -> String -> String -> List (String, String, String) -> Bytes -> String -> Tk
  -- | Bytes from hex.
  b :: String -> Bytes
  b text = case hex-decode text of Ok v -> v; Err e -> bytes-new 0
  -- | A string from hex.
  t :: String -> String
  t text = case string-from-bytes (b text) of Some s -> s; None -> "invalid"
  -- | The token a description names.
  make :: Tk -> XmlToken
  make d = case d of
    Tk kind space local attrs data-bytes goal ->
      if kind == "start" then XmlStartElement (xml-name space local) (map make-attr attrs)
      else if kind == "end" then XmlEndElement (xml-name space local)
      else if kind == "chardata" then XmlCharData data-bytes
      else if kind == "comment" then XmlComment data-bytes
      else if kind == "procinst" then XmlProcInst goal data-bytes
      else XmlDirective data-bytes
  -- | An attribute from a description.
  make-attr :: (String, String, String) -> XmlAttr
  make-attr a = case a of (space, local, value) -> xml-attr (xml-name space local) value
  -- | Names agree.
  same-name :: XmlName -> XmlName -> Bool
  same-name x y = xml-name-space x == xml-name-space y && xml-name-local x == xml-name-local y
  -- | Attribute lists agree.
  same-attrs :: List XmlAttr -> List XmlAttr -> Bool
  same-attrs x y = case x of
    Nil -> (case y of Nil -> True; Cons a rest -> False)
    Cons a more -> case y of Nil -> False; Cons c rest -> same-name (xml-attr-name a) (xml-attr-name c) && xml-attr-value a == xml-attr-value c && same-attrs more rest
  -- | Tokens agree.
  same :: XmlToken -> XmlToken -> Bool
  same x y = case x of
    XmlStartElement n a -> (case y of XmlStartElement m c -> same-name n m && same-attrs a c; other -> False)
    XmlEndElement n -> (case y of XmlEndElement m -> same-name n m; other -> False)
    XmlCharData p -> (case y of XmlCharData q -> hex-encode p == hex-encode q; other -> False)
    XmlComment p -> (case y of XmlComment q -> hex-encode p == hex-encode q; other -> False)
    XmlProcInst g p -> (case y of XmlProcInst h q -> g == h && hex-encode p == hex-encode q; other -> False)
    XmlDirective p -> (case y of XmlDirective q -> hex-encode p == hex-encode q; other -> False)
  -- | Step a decoder through the expected tokens, then expect the end or the error.
  steps :: Bool -> XmlDecoder -> List Tk -> String -> Bool
  steps raw d want error = case (if raw then xml-next-raw d else xml-next d) of
    Err e -> is-nil want && matches-error e error
    Ok step -> case step of
      None -> is-nil want && error == ""
      Some (got, next) -> case want of
        Nil -> False
        Cons w rest -> same got (make w) && steps raw next rest error
  -- | Whether no tokens remain expected.
  is-nil :: List Tk -> Bool
  is-nil xs = case xs of Nil -> True; Cons x rest -> False
  -- | An error equals the reference message (encoding refusals by kind).
  matches-error :: XmlError -> String -> Bool
  matches-error e expected = case e of
    XmlUnsupportedEncoding name -> string-byte-length expected > 13 && not (expected == xml-error-message e)
    other -> xml-error-message e == expected
  -- | A decoding row.
  dec :: Bool -> Bytes -> List Tk -> String -> Bool
  dec raw input want error = steps raw (xml-decoder input) want error
  -- | An encoding row.
  enc :: List Tk -> String -> String -> Bytes -> String -> Bool
  enc tokens prefix indent output error =
    let e = if prefix == "" && indent == "" then xml-encoder else xml-encoder-indent prefix indent in
    case xml-encode e (map make tokens) of
      Ok out -> error == "" && hex-encode out == hex-encode output
      Err err -> xml-error-message err == error
  -- | An escaping row.
  esc :: Bytes -> Bytes -> Bool
  esc input output = hex-encode (xml-escape-attribute input) == hex-encode output
  -- | Trees: build, query and render a document.
  trees :: Unit -> Bool
  trees unit = case xml-document (string-to-bytes "<?xml version=\"1.0\"?><lib xmlns:x=\"urn:x\"><book id=\"1\" x:k=\"v\">One <i>two</i></book><!--c--><book id=\"2\"/></lib>") of
    Err e -> False
    Ok top -> case xml-nodes top of
      Cons decl rest -> (case rest of
        Cons root more ->
          let books = xml-child-elements root "book" in
          list-length books == 2
            && (case books of Cons first others -> is-text (xml-attribute first "id") "1" && is-none (xml-attribute first "k") && hex-encode (xml-text first) == hex-encode (string-to-bytes "One two"); Nil -> False)
            && (case xml-render xml-encoder top of Ok out -> hex-encode out == hex-encode (string-to-bytes "<?xml version=\"1.0\"?><lib xmlns:_xmlns=\"xmlns\" _xmlns:x=\"urn:x\"><book id=\"1\" xmlns:_=\"urn:x\" _:k=\"v\">One <i>two</i></book><!--c--><book id=\"2\"></book></lib>"); Err e -> False)
        Nil -> False)
      Nil -> False
  -- | Whether an option holds the given text.
  is-text :: Option String -> String -> Bool
  is-text o want = case o of Some v -> v == want; None -> False
  -- | Whether an option is empty.
  is-none :: Option String -> Bool
  is-none o = case o of Some v -> False; None -> True
  -- | Index of the first failing row, or 200 when all pass.
  first :: List Bool -> Int -> Int
  first values i = case values of Nil -> 200; Cons x rest -> if x then first rest (i + 1) else i
"#;
