//! GIF decoding and encoding against the reference SDK (`tooling/gif_vectors.go`): animations
//! with global and local palettes, transparency, sub-rectangle frames, delays, disposal methods
//! and every loop mode are encoded byte for byte as the reference encoder writes them and decoded
//! to the reference frames; interlaced and GIF87a files written by PIL decode to the reference
//! frames; malformed files fail with the reference error kind.
mod common;

use serde_json::Value;

/// Compile and run one batch of fixture rows; the exit status is the first failing row, 200 if none.
fn run_batch(rows: &[String]) {
    let dir = common::project("gif", &[
        ("Image.Gif.cinch", include_str!("../prelude/Image/Gif.cinch")),
        ("Image.Raster.cinch", include_str!("../prelude/Image/Raster.cinch")),
        ("Text.Lzw.cinch", include_str!("../prelude/Text/Lzw.cinch")),
        ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
        ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
        ("Text.Binary.cinch", include_str!("../prelude/Text/Binary.cinch")),
        ("Main.cinch", &format!("{HELPERS}\n  -- | Every row in this batch.\n  main :: Int\n  main = first [{}] 0\n", rows.join(",\n    "))),
    ]);
    assert_eq!(common::run(&dir, "Main.cinch"), 200, "the exit status is the first failing row index in the batch; 200 means every row passed");
    common::cleanup(&dir);
}

/// A Cinch description of an animation: screen, loop, background, global palette and frames.
fn animation(g: &Value) -> String {
    let frames = g["frames"].as_array().unwrap().iter().map(|f| {
        format!("(Spec {} {} {} {} \"{}\" \"{}\" {} {})", f["left"], f["top"], f["width"], f["height"], f["pix"].as_str().unwrap(), f["palette"].as_str().unwrap(), f["delay"], f["disposal"])
    }).collect::<Vec<_>>();
    format!("(Want {} {} ({}) {} \"{}\" [{}])", g["width"], g["height"], g["loop"], g["background"], g["global"].as_str().unwrap(), frames.join(", "))
}

/// The fixture.
fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/gif-vectors.json")).unwrap()
}

/// Every animation encodes to exactly the reference bytes.
#[test]
fn gif_encodes_reference_bytes() {
    let f = fixture();
    let rows = f["encode"].as_array().unwrap().iter().map(|r| format!("encodes {} \"{}\"", animation(&r["gif"]), r["bytes"].as_str().unwrap())).collect::<Vec<_>>();
    for batch in rows.chunks(12) {
        run_batch(batch);
    }
}

/// Every file decodes to the reference frames or fails with the reference kind.
#[test]
fn gif_decodes_reference_frames_and_errors() {
    let f = fixture();
    let mut rows = f["decode"].as_array().unwrap().iter().map(|r| {
        if r["error"].as_str().unwrap().is_empty() {
            format!("decodes \"{}\" {}", r["bytes"].as_str().unwrap(), animation(&r["gif"]))
        } else {
            format!("rejects \"{}\" {:?}", r["bytes"].as_str().unwrap(), r["error"].as_str().unwrap())
        }
    }).collect::<Vec<_>>();
    rows.push("single ()".to_string());
    for batch in rows.chunks(14) {
        run_batch(batch);
    }
}

/// The index validator lowers for 64-bit ARM, WebAssembly and a big-endian target.
#[test]
fn gif_kernels_compile_for_arm_wasm_and_big_endian_targets() {
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
            ("Image.Gif", include_str!("../prelude/Image/Gif.cinch")),
            ("Image.Raster", include_str!("../prelude/Image/Raster.cinch")),
            ("Text.Lzw", include_str!("../prelude/Text/Lzw.cinch")),
            ("Data.Bytes.Build", include_str!("../prelude/Data/Bytes/Build.cinch")),
            ("Data.Bytes.Word", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ] {
            std::fs::write(project.join(format!("{name}.cinch")), text).unwrap();
        }
        std::fs::write(project.join("Main.cinch"), "module Main where\n  import Image.Gif\n  import Image.Raster\n  -- | Reach the kernels.\n  main :: Int\n  main = case gif-decode (bytes-new 3) of\n    Ok r -> 0\n    Err e -> 1\n").unwrap();
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
        assert!(ir.contains("@llvm.vector.reduce.umax.v16i8"));
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
fn gif_declarations_have_canonical_documentation() {
    common::assert_documented("Image.Gif", include_str!("../prelude/Image/Gif.cinch"));
}

/// Checked consumer helpers.
const HELPERS: &str = r#"-- | Reference GIF vector consumers.
module Main where
  import Image.Gif
  import Image.Raster
  import Text.Hex
  -- | One expected frame: left, top, width, height, indices, RGBA palette, delay and disposal number.
  data Spec where
    -- | Fields in the order listed above.
    Spec :: Nat -> Nat -> Nat -> Nat -> String -> String -> Nat -> Nat -> Spec
  -- | An expected animation: width, height, loop count (-1 once, 0 forever), background, global palette and frames.
  data Want where
    -- | Fields in the order listed above.
    Want :: Nat -> Nat -> Int -> Nat -> String -> List Spec -> Want
  -- | Bytes from hex.
  b :: String -> Bytes
  b text = case hex-decode text of Ok v -> v; Err e -> bytes-new 0
  -- | The loop setting for a reference loop count.
  loop-of :: Int -> GifLoop
  loop-of n = if n < 0 then GifPlayOnce else if n == 0 then GifLoopForever else GifLoopTimes (int-to-nat n)
  -- | A non-negative integer as a natural number.
  int-to-nat :: Int -> Nat
  int-to-nat n = if n <= 0 then 0 else 1 + int-to-nat (n - 1)
  -- | The disposal for its number.
  disposal :: Nat -> GifDisposal
  disposal n = if n == 0 then GifDisposalUnspecified else if n == 1 then GifDisposalNone else if n == 2 then GifDisposalBackground else if n == 3 then GifDisposalPrevious else GifDisposalReserved n
  -- | The disposal's number.
  number :: GifDisposal -> Nat
  number d = case d of GifDisposalUnspecified -> 0; GifDisposalNone -> 1; GifDisposalBackground -> 2; GifDisposalPrevious -> 3; GifDisposalReserved n -> n
  -- | Build the frame a spec describes.
  build :: Spec -> GifFrame
  build s = case s of
    Spec left top w h pix pal delay d -> case raster Indexed w h (b pix) (b pal) of
      Some r -> gif-frame r left top delay (disposal d)
      None -> gif-frame (raster-new Indexed 0 0) left top delay (disposal d)
  -- | Build the animation a description gives.
  animation :: Want -> Gif
  animation w = case w of Want width height loop bg global frames -> gif width height (map build frames) (loop-of loop) bg (b global)
  -- | An encoding row.
  encodes :: Want -> String -> Bool
  encodes w expected = case gif-encode-all (animation w) of Ok out -> hex-encode out == expected; Err e -> False
  -- | Loops agree.
  same-loop :: GifLoop -> Int -> Bool
  same-loop l n = case l of GifPlayOnce -> n < 0; GifLoopForever -> n == 0; GifLoopTimes k -> n > 0 && k == int-to-nat n
  -- | A decoded frame equals its spec.
  frame-is :: GifFrame -> Spec -> Bool
  frame-is f s = case s of
    Spec left top w h pix pal delay d ->
      gif-frame-left f == left && gif-frame-top f == top && raster-width (gif-frame-raster f) == w && raster-height (gif-frame-raster f) == h
        && hex-encode (raster-pixels (gif-frame-raster f)) == pix && hex-encode (raster-palette (gif-frame-raster f)) == pal
        && gif-frame-delay f == delay && number (gif-frame-disposal f) == d
  -- | Frames agree in order.
  frames-are :: List GifFrame -> List Spec -> Bool
  frames-are got want = case got of
    Nil -> (case want of Nil -> True; Cons s rest -> False)
    Cons f more -> case want of Nil -> False; Cons s rest -> frame-is f s && frames-are more rest
  -- | A decoding row.
  decodes :: String -> Want -> Bool
  decodes input w = case w of
    Want width height loop bg global frames -> case gif-decode-all (b input) of
      Err e -> False
      Ok g -> gif-width g == width && gif-height g == height && same-loop (gif-loop g) loop && gif-background-index g == bg
        && hex-encode (gif-palette g) == global && frames-are (gif-frames g) frames
        && (case gif-config (b input) of Ok (cw, ch, cp) -> cw == width && ch == height && hex-encode cp == global; Err e -> False)
  -- | The fixture's kind for an error.
  kind :: GifError -> String
  kind e = case e of
    GifBadHeader -> "header"
    GifTruncated -> "truncated"
    GifUnknownBlock n -> "block"
    GifUnknownExtension n -> "extension"
    GifBadGraphicControl -> "graphic-control"
    GifFrameOutOfBounds -> "bounds"
    GifNoColorTable -> "no-color-table"
    GifBadLiteralWidth n -> "literal-width"
    GifNotEnoughData -> "not-enough"
    GifTooMuchData -> "too-much"
    GifBadPixel -> "bad-pixel"
    GifBadImageData cause -> "image-data"
    GifMissingImage -> "missing-image"
    other -> "encoder"
  -- | A rejection row.
  rejects :: String -> String -> Bool
  rejects input expected = case gif-decode-all (b input) of Ok g -> False; Err e -> kind e == expected
  -- | Single-image encoding and decoding, and encoder refusals.
  single :: Unit -> Bool
  single unit = case raster Indexed 3 2 (bytes-from-u8s [0, 1, 1, 0, 2, 2]) (bytes-from-u8s [255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 0]) of
    None -> False
    Some r -> (case gif-encode r of
        Err e -> False
        Ok out -> (case gif-decode out of Ok back -> hex-encode (raster-pixels back) == "000101000202" && hex-encode (raster-palette back) == "ff0000ff00ff00ff00000000000000ff"; Err e -> False))
      && (case gif-encode (raster-new Rgb8 2 2) of Err GifNotIndexed -> True; other -> False)
      && (case gif-encode-all (gif 2 2 Nil GifPlayOnce 0 (bytes-new 0)) of Err GifNoFrames -> True; other -> False)
      && (case gif-encode-all (gif 2 1 (Cons (gif-frame r 0 0 0 GifDisposalNone) Nil) GifPlayOnce 0 (bytes-new 0)) of Err GifTooLarge -> True; other -> False)
      && gif-error-message GifTooMuchData == "gif: too much image data"
  -- | Index of the first failing row, or 200 when all pass.
  first :: List Bool -> Int -> Int
  first values i = case values of Nil -> 200; Cons x rest -> if x then first rest (i + 1) else i
"#;
