//! PNG decoding against independent vectors from `tooling/png_vectors.py` (an encoder written from
//! the specification covering every color type, bit depth, filter, Adam7 interlacing and tRNS form,
//! plus files written by PIL), malformed-file rejection, and encode/decode round trips.
mod common;

use serde_json::Value;

/// Compile and run checks; the exit status is the first failing check, 200 if none.
fn run(checks: &[String]) {
    let dir = common::project("png", &[
        ("Image.Png.cinch", include_str!("../prelude/Image/Png.cinch")),
        ("Image.Raster.cinch", include_str!("../prelude/Image/Raster.cinch")),
        ("Text.Zlib.cinch", include_str!("../prelude/Text/Zlib.cinch")),
        ("Text.Checksum.cinch", include_str!("../prelude/Text/Checksum.cinch")),
        ("Text.Adler32.cinch", include_str!("../prelude/Text/Adler32.cinch")),
        ("Text.Crc64.cinch", include_str!("../prelude/Text/Crc64.cinch")),
        ("Text.Binary.cinch", include_str!("../prelude/Text/Binary.cinch")),
        ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
        ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
        ("Main.cinch", &format!("{HELPERS}\n  -- | Every check.\n  main :: Int\n  main = first [{}] 0\n", checks.join(",\n    "))),
    ]);
    assert_eq!(common::run(&dir, "Main.cinch"), 200, "the exit status is the first failing check; 200 means every check passed");
    common::cleanup(&dir);
}

/// Every vector decodes to its expected raster and re-encodes losslessly.
#[test]
fn png_decodes_every_format_filter_and_interlace() {
    let f: Value = serde_json::from_str(include_str!("fixtures/png-vectors.json")).unwrap();
    let checks = f["vectors"].as_array().unwrap().iter()
        .map(|v| format!("png-case {:?} {} {} {} {:?} {:?}", v["png"].as_str().unwrap(), v["format"], v["width"], v["height"], v["pixels"].as_str().unwrap(), v["palette"].as_str().unwrap()))
        .collect::<Vec<_>>();
    for batch in checks.chunks(140) {
        run(batch);
    }
}

/// Malformed files are rejected.
#[test]
fn png_rejects_malformed_files() {
    let f: Value = serde_json::from_str(include_str!("fixtures/png-vectors.json")).unwrap();
    let checks = f["invalid"].as_array().unwrap().iter().map(|v| format!("rejected {:?}", v.as_str().unwrap())).collect::<Vec<_>>();
    run(&checks);
}

/// Every public and private declaration has canonical module-owned documentation.
#[test]
fn png_declarations_have_canonical_documentation() {
    common::assert_documented("Image.Png", include_str!("../prelude/Image/Png.cinch"));
    common::assert_documented("Image.Raster", include_str!("../prelude/Image/Raster.cinch"));
}

/// Checked consumer helpers.
const HELPERS: &str = r#"-- | PNG vector consumers.
module Main where
  import Image.Png
  import Image.Raster
  import Text.Hex
  -- | Parse recorded bytes.
  input :: String -> Bytes
  input t = case hex-decode t of Ok b -> b; Err e -> bytes-new 0
  -- | The fixture's format code.
  code :: PixelFormat -> Nat
  code f = case f of
    Gray8 -> 0
    Gray16 -> 1
    GrayAlpha8 -> 2
    GrayAlpha16 -> 3
    Rgb8 -> 4
    Rgb16 -> 5
    Rgba8 -> 6
    Rgba16 -> 7
    Indexed -> 8
  -- | Decoding yields the expected raster, and encoding it decodes back to the same pixels and palette.
  png-case :: String -> Nat -> Nat -> Nat -> String -> String -> Bool
  png-case png format width height pixels palette = case png-decode (input png) of
    Err e -> False
    Ok r -> code (raster-format r) == format && raster-width r == width && raster-height r == height
      && (case png-config (input png) of Ok (w, h, f) -> w == width && h == height && code f == format; Err e -> False)
      && hex-encode (raster-pixels r) == pixels && hex-encode (raster-palette r) == palette
      && (case png-decode (png-encode r) of
        Ok back -> code (raster-format back) == format && hex-encode (raster-pixels back) == pixels && hex-encode (raster-palette back) == palette
        Err e -> False)
  -- | A malformed file fails.
  rejected :: String -> Bool
  rejected png = case png-decode (input png) of Ok r -> False; Err e -> True
  -- | Index of the first failing check, or 200 when all pass.
  first :: List Bool -> Int -> Int
  first values i = case values of Nil -> 200; Cons x rest -> if x then first rest (i + 1) else i
"#;

/// The SIMD unfiltering kernels (including the big-endian Sub bodies) compile for AArch64,
/// WebAssembly and big-endian PowerPC; foreign execution is not claimed.
#[test]
fn png_kernels_compile_for_arm_wasm_and_big_endian_targets() {
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
            ("Image.Png", include_str!("../prelude/Image/Png.cinch")),
            ("Image.Raster", include_str!("../prelude/Image/Raster.cinch")),
            ("Text.Zlib", include_str!("../prelude/Text/Zlib.cinch")),
            ("Text.Checksum", include_str!("../prelude/Text/Checksum.cinch")),
            ("Text.Adler32", include_str!("../prelude/Text/Adler32.cinch")),
            ("Text.Crc64", include_str!("../prelude/Text/Crc64.cinch")),
            ("Text.Binary", include_str!("../prelude/Text/Binary.cinch")),
            ("Data.Bytes.Build", include_str!("../prelude/Data/Bytes/Build.cinch")),
            ("Data.Bytes.Word", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ] {
            std::fs::write(project.join(format!("{name}.cinch")), text).unwrap();
        }
        std::fs::write(project.join("Main.cinch"), "module Main where\n  import Image.Png\n  import Image.Raster\n  -- | Reach the kernels.\n  main :: Int\n  main = case png-decode (png-encode (raster-new Rgb8 9 3)) of\n    Ok r -> if raster-width r == 9 then 0 else 1\n    Err e -> 2\n").unwrap();
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
        assert!(ir.contains("<8 x i8>"));
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
