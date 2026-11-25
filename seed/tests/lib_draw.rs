//! Compositing against the reference SDK's draw package (`tooling/draw_vectors.go`): Src and
//! Over, with and without masks and with uniform sources, over clipped rectangles, for 8- and
//! 16-bit RGBA, 8- and 16-bit gray and indexed destinations and sources, and Floyd-Steinberg
//! dithering onto indexed rasters. Every destination pixel must match.
mod common;

use serde_json::Value;

/// Compile and run one batch of fixture rows; the exit status is the first failing row, 200 if none.
fn run_batch(rows: &[String]) {
    let dir = common::project("draw", &[
        ("Image.Draw.cinch", include_str!("../prelude/Image/Draw.cinch")),
        ("Image.Raster.cinch", include_str!("../prelude/Image/Raster.cinch")),
        ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
        ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
        ("Text.Binary.cinch", include_str!("../prelude/Text/Binary.cinch")),
        ("Main.cinch", &format!("{HELPERS}\n  -- | Every row in this batch.\n  main :: Int\n  main = first [{}] 0\n", rows.join(",\n    "))),
    ]);
    assert_eq!(common::run(&dir, "Main.cinch"), 200, "the exit status is the first failing row index in the batch; 200 means every row passed");
    common::cleanup(&dir);
}

/// A Cinch raster description.
fn image(v: &Value) -> String {
    format!("(Img {:?} {} {} \"{}\" \"{}\")", v["kind"].as_str().unwrap(), v["width"], v["height"], v["pix"].as_str().unwrap(), v["palette"].as_str().unwrap())
}

/// A signed integer literal.
fn int(v: &Value) -> String {
    format!("({})", v.as_i64().unwrap())
}

/// Every reference draw produces the reference destination pixels.
#[test]
fn draw_matches_reference_compositing() {
    let rows: Value = serde_json::from_str(include_str!("fixtures/draw-vectors.json")).unwrap();
    let checks = rows.as_array().unwrap().iter().map(|r| {
        let rect = r["rect"].as_array().unwrap().iter().map(int).collect::<Vec<_>>().join(" ");
        let pt = |k: &str| format!("({}, {})", int(&r[k][0]), int(&r[k][1]));
        let mask = if r["mask"].is_null() { "None".to_string() } else { format!("(Some {})", image(&r["mask"])) };
        let uniform = if r["uniform"].is_null() { "None".to_string() } else { let u = r["uniform"].as_array().unwrap(); format!("(Some ({}, {}, {}, {}))", u[0], u[1], u[2], u[3]) };
        format!("row {} {} {} {} {} {} {} {:?} {} \"{}\"", image(&r["dst"]), image(&r["src"]), mask, uniform, rect, pt("sp"), pt("mp"), r["op"].as_str().unwrap(), if r["dither"].as_bool().unwrap() { "True" } else { "False" }, r["out"].as_str().unwrap())
    }).collect::<Vec<_>>();
    for batch in checks.chunks(22) {
        run_batch(batch);
    }
}

/// The four-pixel compositing kernels lower for 64-bit ARM, WebAssembly and a big-endian target.
#[test]
fn draw_kernels_compile_for_arm_wasm_and_big_endian_targets() {
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
            ("Image.Draw", include_str!("../prelude/Image/Draw.cinch")),
            ("Image.Raster", include_str!("../prelude/Image/Raster.cinch")),
            ("Data.Bytes.Word", include_str!("../prelude/Data/Bytes/Word.cinch")),
            ("Data.Bytes.Build", include_str!("../prelude/Data/Bytes/Build.cinch")),
        ] {
            std::fs::write(project.join(format!("{name}.cinch")), text).unwrap();
        }
        std::fs::write(project.join("Main.cinch"), "module Main where\n  import Image.Draw\n  import Image.Raster\n  -- | Reach the kernels.\n  main :: Int\n  main = let d = raster-new Rgba8 8 2 in if bytes-len (raster-pixels (draw (draw d (draw-rect 0 0 8 2) d (0, 0) DrawOver) (draw-rect 0 0 8 2) d (0, 0) DrawSrc)) == 64 then 0 else 1\n").unwrap();
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
        assert!(ir.contains("<4 x i32>") && ir.contains("load i128"));
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
fn draw_declarations_have_canonical_documentation() {
    common::assert_documented("Image.Draw", include_str!("../prelude/Image/Draw.cinch"));
}

/// Checked consumer helpers.
const HELPERS: &str = r#"-- | Reference compositing consumers.
module Main where
  import Image.Draw
  import Image.Raster
  import Text.Hex
  -- | A raster description: kind, width, height, pixels and palette in hex.
  data Img where
    -- | Fields in the order listed above.
    Img :: String -> Nat -> Nat -> String -> String -> Img
  -- | Bytes from hex.
  b :: String -> Bytes
  b text = case hex-decode text of Ok v -> v; Err e -> bytes-new 0
  -- | The raster format for a reference image kind.
  format :: String -> PixelFormat
  format k = if k == "nrgba" then Rgba8 else if k == "nrgba64" then Rgba16 else if k == "gray" then Gray8 else if k == "gray16" then Gray16 else Indexed
  -- | The raster a description names.
  image :: Img -> Raster
  image i = case i of Img k w h pix pal -> case raster (format k) w h (b pix) (b pal) of Some r -> r; None -> raster-new Gray8 0 0
  -- | The operator for its name.
  op-of :: String -> DrawOp
  op-of n = if n == "src" then DrawSrc else DrawOver
  -- | One row: draw as the reference did and compare the destination pixels.
  row :: Img -> Img -> Option Img -> Option (Nat, Nat, Nat, Nat) -> Int -> Int -> Int -> Int -> (Int, Int) -> (Int, Int) -> String -> Bool -> String -> Bool
  row dst src mask uniform x0 y0 x1 y1 sp mp op dither out =
    let r = draw-rect x0 y0 x1 y1 in
    let result = if dither then draw-floyd-steinberg (image dst) r (image src) sp else drawn (image dst) r (image src) mask uniform sp mp (op-of op) in
    hex-encode (raster-pixels result) == out
  -- | The non-dithering draws.
  drawn :: Raster -> DrawRect -> Raster -> Option Img -> Option (Nat, Nat, Nat, Nat) -> (Int, Int) -> (Int, Int) -> DrawOp -> Raster
  drawn dst r src mask uniform sp mp op = case uniform of
    Some c -> draw-uniform dst r c op
    None -> case mask of Some m -> draw-mask dst r src sp (image m) mp op; None -> draw dst r src sp op
  -- | Index of the first failing row, or 200 when all pass.
  first :: List Bool -> Int -> Int
  first values i = case values of Nil -> 200; Cons x rest -> if x then first rest (i + 1) else i
"#;
