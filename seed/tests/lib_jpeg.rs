//! JPEG decoding and encoding against the reference SDK (`tooling/jpeg_vectors.go`): files from an
//! independent encoder (`tooling/jpeg_inputs.py`: every common chroma layout, progressive and
//! restart-interval coding, grayscale, CMYK, YCCK and RGB-coded files) and the SDK's own test images
//! (4:1:1, 4:1:0 and 4:4:0 chroma, separate DC progressions, truncated progressive data) decode to
//! identical samples, RGB conversions and configurations; truncated and corrupted files fail as the
//! reference does; rasters of every pixel format and YCbCr planes of every chroma ratio encode to the
//! reference's exact bytes at qualities from 1 to 100. The kernels also lower for other targets.
mod common;

use serde_json::Value;

/// Compile and run one batch of fixture rows; the exit status is the first failing row, 200 if none.
fn run_batch(rows: &[String]) {
    let dir = common::project("jpeg", &[
        ("Image.Jpeg.cinch", include_str!("../prelude/Image/Jpeg.cinch")),
        ("Image.Raster.cinch", include_str!("../prelude/Image/Raster.cinch")),
        ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
        ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
        ("Text.Binary.cinch", include_str!("../prelude/Text/Binary.cinch")),
        ("Main.cinch", &format!("{HELPERS}\n  -- | Every row in this batch.\n  main :: Int\n  main = first [{}] 0\n", rows.join(",\n    "))),
    ]);
    assert_eq!(common::run(&dir, "Main.cinch"), 200, "the exit status is the first failing row index in the batch; 200 means every row passed");
    common::cleanup(&dir);
}

/// The fixture.
fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/jpeg-vectors.json")).unwrap()
}

/// A string field as a Cinch literal.
fn s(v: &Value, k: &str) -> String {
    format!("{:?}", v[k].as_str().unwrap())
}

/// The decoding rows as Cinch calls; the reference's end-of-data error is this codec's truncation error.
pub fn decode_rows() -> Vec<String> {
    fixture()["decode"].as_array().unwrap().iter().map(|r| {
        let err = r["error"].as_str().unwrap().replace("unexpected EOF", "unexpected end of JPEG data");
        let config = r["config"].as_str().unwrap().replace("unexpected EOF", "unexpected end of JPEG data");
        format!("decodes {} {:?} {} {} {} {} {} {} {:?}", s(r, "data"), err, s(r, "kind"), r["width"], r["height"], s(r, "ratio"), s(r, "planes"), s(r, "rgb"), config)
    }).collect()
}

/// The encoding rows as Cinch calls.
pub fn encode_rows() -> Vec<String> {
    fixture()["encode"].as_array().unwrap().iter().map(|r| {
        format!("encodes {} {} {} {} {} {} {} {} {}", s(r, "kind"), r["width"], r["height"], s(r, "pix"), s(r, "palette"), s(r, "ratio"), r["quality"], s(r, "digest"), r["length"])
    }).collect()
}

/// Every file decodes (or fails) exactly as the reference decoder.
#[test]
fn jpeg_decoding_matches_reference() {
    for batch in decode_rows().chunks(40) {
        run_batch(batch);
    }
}

/// Every raster and YCbCr image encodes to the reference encoder's bytes.
#[test]
fn jpeg_encoding_matches_reference() {
    for batch in encode_rows().chunks(30) {
        run_batch(batch);
    }
}

/// The kernels lower for 64-bit ARM, WebAssembly and a big-endian target.
#[test]
fn jpeg_kernels_compile_for_arm_wasm_and_big_endian_targets() {
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
            ("Image.Jpeg", include_str!("../prelude/Image/Jpeg.cinch")),
            ("Image.Raster", include_str!("../prelude/Image/Raster.cinch")),
            ("Data.Bytes.Build", include_str!("../prelude/Data/Bytes/Build.cinch")),
            ("Data.Bytes.Word", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ] {
            std::fs::write(project.join(format!("{name}.cinch")), text).unwrap();
        }
        std::fs::write(project.join("Main.cinch"), "module Main where\n  import Image.Jpeg\n  import Image.Raster\n  -- | Reach the kernels.\n  main :: Int\n  main = case jpeg-encode 75 (raster-new Rgba8 9 9) of\n    Ok encoded -> (case jpeg-decode encoded of Ok m -> (if raster-width (jpeg-to-raster m) == 9 then 0 else 3); Err e -> 1)\n    Err e -> 2\n").unwrap();
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
        assert!(ir.contains("<4 x i32>") && ir.contains("<8 x i16>"));
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
fn jpeg_declarations_have_canonical_documentation() {
    common::assert_documented("Image.Jpeg", include_str!("../prelude/Image/Jpeg.cinch"));
}

/// Checked consumer helpers.
const HELPERS: &str = r#"-- | Reference JPEG consumers.
module Main where
  import Image.Jpeg
  import Image.Raster
  import Data.Bytes.Build
  import Text.Hex
  import Text.Show (nat-to-string)
  -- | Bytes from hex.
  b :: String -> Bytes
  b text = case hex-decode text of Ok v -> v; Err e -> bytes-new 0
  -- | A SHA-256 digest in hex.
  digest :: Bytes -> String
  digest x = hex-encode (sha256-bytes x)
  -- | Rows y onwards of a plane: w bytes each at the stride.
  rows :: Bytes -> Nat -> Nat -> Nat -> Nat -> List Bytes
  rows p stride w h y = if y >= h then Nil else Cons (part p (y * stride) w) (rows p stride w h (y + 1))
  -- | n bytes at an offset.
  part :: Bytes -> Nat -> Nat -> Bytes
  part p o n = let out = bytes-new n in let c = (if o + n <= bytes-len p then bytes-copy-range out p 0 o n else ()) in out
  -- | The visible samples in the reference's order.
  samples :: JpegImage -> Bytes
  samples m = case m of
    JpegGray r -> raster-pixels r
    JpegRgb r -> raster-pixels r
    JpegCmyk c -> cmyk-pixels c
    JpegYcbcr y ->
      let w = ycbcr-width y in
      let h = ycbcr-height y in
      let cw = chroma-width (ycbcr-ratio y) w in
      let ch = chroma-height (ycbcr-ratio y) h in
      bytes-concat [bytes-concat (rows (ycbcr-y y) (ycbcr-y-stride y) w h 0), bytes-concat (rows (ycbcr-cb y) (ycbcr-c-stride y) cw ch 0), bytes-concat (rows (ycbcr-cr y) (ycbcr-c-stride y) cw ch 0)]
  -- | The image's kind and chroma ratio names.
  kind-of :: JpegImage -> (String, String)
  kind-of m = case m of
    JpegGray r -> ("gray", "")
    JpegRgb r -> ("rgb", "")
    JpegCmyk c -> ("cmyk", "")
    JpegYcbcr y -> ("ycbcr", ratio-name (ycbcr-ratio y))
  -- | The first of a pair.
  fst :: (String, String) -> String
  fst p = case p of (a, c) -> a
  -- | The second of a pair.
  snd :: (String, String) -> String
  snd p = case p of (a, c) -> c
  -- | A chroma ratio's name.
  ratio-name :: ChromaRatio -> String
  ratio-name r = case r of
    Chroma444 -> "444"
    Chroma422 -> "422"
    Chroma420 -> "420"
    Chroma440 -> "440"
    Chroma411 -> "411"
    Chroma410 -> "410"
  -- | The configuration reader's answer as text.
  config-text :: Bytes -> String
  config-text input = case jpeg-decode-config input of
    Err e -> string-concat "error: " (jpeg-error-message e)
    Ok c ->
      let model = (case jpeg-config-model c of JpegGrayModel -> "gray"; JpegYcbcrModel -> "ycbcr"; JpegRgbModel -> "rgb"; JpegCmykModel -> "cmyk") in
      string-concat model (string-concat " " (string-concat (nat-to-string (jpeg-config-width c)) (string-concat " " (nat-to-string (jpeg-config-height c)))))
  -- | A decoding row: the error, or the kind, size, ratio and digests; and the configuration.
  decodes :: String -> String -> String -> Nat -> Nat -> String -> String -> String -> String -> Bool
  decodes input err kind w h ratio planes rgb config =
    let file = b input in
    config-text file == config && (case jpeg-decode file of
      Err e -> jpeg-error-message e == err
      Ok m -> err == "" && fst (kind-of m) == kind && snd (kind-of m) == ratio && raster-width (jpeg-to-raster m) == w && raster-height (jpeg-to-raster m) == h
        && digest (samples m) == planes && digest (raster-pixels (jpeg-to-raster m)) == rgb)
  -- | The pixel format named by a row.
  format-of :: String -> PixelFormat
  format-of k =
    if k == "gray8" then Gray8 else if k == "rgb8" then Rgb8 else if k == "rgba8" then Rgba8 else if k == "gray16" then Gray16
    else if k == "rgba16" then Rgba16 else if k == "rgb16" then Rgb16 else if k == "indexed" then Indexed
    else if k == "grayalpha8" then GrayAlpha8 else GrayAlpha16
  -- | The ratio named by a row.
  ratio-of :: String -> ChromaRatio
  ratio-of r = if r == "422" then Chroma422 else if r == "420" then Chroma420 else if r == "440" then Chroma440 else if r == "411" then Chroma411 else if r == "410" then Chroma410 else Chroma444
  -- | An encoding row: the digest and length of the encoded bytes.
  encodes :: String -> Nat -> Nat -> String -> String -> String -> Nat -> String -> Nat -> Bool
  encodes kind w h pix palette ratio quality want size =
    let out = (if kind == "ycbcr" then ycbcr-encode w h (b pix) (ratio-of ratio) quality else raster-encode (format-of kind) w h (b pix) (b palette) quality) in
    case out of
      Ok encoded -> digest encoded == want && bytes-len encoded == size
      Err e -> False
  -- | Encode a raster.
  raster-encode :: PixelFormat -> Nat -> Nat -> Bytes -> Bytes -> Nat -> Result JpegError Bytes
  raster-encode f w h pix palette quality = case raster f w h pix palette of
    Some r -> jpeg-encode quality r
    None -> Err JpegTooLarge
  -- | Encode YCbCr planes given back to back with tight strides.
  ycbcr-encode :: Nat -> Nat -> Bytes -> ChromaRatio -> Nat -> Result JpegError Bytes
  ycbcr-encode w h pix r quality =
    let cw = chroma-width r w in
    let ch = chroma-height r h in
    case ycbcr-from-planes r w h w cw (part pix 0 (w * h)) (part pix (w * h) (cw * ch)) (part pix (w * h + cw * ch) (cw * ch)) of
      Some m -> jpeg-encode-ycbcr quality m
      None -> Err JpegTooLarge
  -- | The index of the first failing row, or 200.
  first :: List Bool -> Int -> Int
  first values i = case values of Nil -> 200; Cons x rest -> if x then first rest (i + 1) else i
"#;
