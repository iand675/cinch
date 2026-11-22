//! Type-check Life's module and specialize the checked functions for the board.
use cinch::analysis::{Analysis, Severity};
use std::cell::RefCell;

const INPUT_CAPACITY: usize = 4096;
const MAX_AGE: u32 = 32;
const MAX_WIDTH: u32 = 62;
const MAX_HEIGHT: u32 = 64;
const NEIGHBOUR_COUNTS: u32 = 9;
const NEIGHBOUR_MASKS: u32 = 256;
const CONTRACT: &str = r#"module Life.Check where
  import Life

  bounded :: Int -> Int -> Int
  bounded limit value = if value < 0 || value > limit then limit + 1 else value

  board-age :: Int -> Int -> Int
  board-age age count = bounded 32 (next-age age count)

  board-neighbours :: U32 -> Int
  board-neighbours mask = bounded 8 (neighbour-count mask)

  board-color :: Int -> Int
  board-color age = cell-color age

  board-step-ms :: Int
  board-step-ms = step-ms

  board-wrap :: Bool
  board-wrap = wrap-edges

  board-width :: Int
  board-width = grid-width

  board-height :: Int
  board-height = grid-height
"#;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct Settings {
    ages: Vec<u32>,
    neighbours: Vec<u32>,
    colors: Vec<u32>,
    step_ms: u32,
    wrap_edges: bool,
    width: u32,
    height: u32,
}

thread_local! {
    static COMPILER: RefCell<Analysis> = RefCell::new({
        let mut compiler = Analysis::new(".");
        compiler.set_document("Life.Check.cinch", CONTRACT);
        compiler
    });
    static INPUT: RefCell<[u8; INPUT_CAPACITY]> = const { RefCell::new([0; INPUT_CAPACITY]) };
    static OUTPUT: RefCell<String> = const { RefCell::new(String::new()) };
}

#[no_mangle]
pub extern "C" fn life_input() -> *mut u8 {
    INPUT.with(|input| input.borrow_mut().as_mut_ptr())
}

#[no_mangle]
pub extern "C" fn life_output() -> *const u8 {
    OUTPUT.with(|output| output.borrow().as_ptr())
}

#[no_mangle]
pub extern "C" fn life_compile(length: usize) -> usize {
    let result = INPUT.with(|input| {
        let input = input.borrow();
        let bytes = input
            .get(..length)
            .ok_or("The source exceeds 4096 bytes.")?;
        let text = std::str::from_utf8(bytes).map_err(|_| "The source is not valid UTF-8.")?;
        COMPILER.with(|compiler| compile(&mut compiler.borrow_mut(), text))
    });
    OUTPUT.with(|output| {
        let mut output = output.borrow_mut();
        *output = match result {
            Ok(settings) => serde_json::json!(settings).to_string(),
            Err(message) => serde_json::json!({ "error": message }).to_string(),
        };
        output.len()
    })
}

fn compile(compiler: &mut Analysis, text: &str) -> Result<Settings, String> {
    compiler.set_document("Life.cinch", text);

    let step_ms = evaluate(compiler, "board-step-ms")?
        .parse::<u32>()
        .ok()
        .filter(|value| *value <= 0x7fff_ffff)
        .ok_or("step-ms must be between 0 and 2147483647.")?;
    let wrap_edges = evaluate(compiler, "board-wrap")? == "True";
    let width = dimension(compiler, "board-width", "grid-width", MAX_WIDTH)?;
    let height = dimension(compiler, "board-height", "grid-height", MAX_HEIGHT)?;

    let mut ages = Vec::new();
    for age in 0..=MAX_AGE {
        let row = packed(
            compiler,
            MAX_AGE,
            (0..NEIGHBOUR_COUNTS).map(|n| format!("board-age {age} {n}")),
        )?;
        if let Some(n) = row.iter().position(|&next| next > MAX_AGE) {
            return Err(format!(
                "next-age {age} {n} must return an age from 0 to {MAX_AGE}."
            ));
        }
        ages.extend(row);
    }

    let mut neighbours = Vec::new();
    for start in (0..NEIGHBOUR_MASKS).step_by(8) {
        let counts = packed(
            compiler,
            NEIGHBOUR_COUNTS - 1,
            (start..start + 8).map(|mask| format!("board-neighbours {mask}")),
        )?;
        if let Some(index) = counts.iter().position(|&count| count >= NEIGHBOUR_COUNTS) {
            return Err(format!(
                "neighbour-count {} must return a count from 0 to 8.",
                start as usize + index
            ));
        }
        neighbours.extend(counts);
    }

    let colors = (1..=MAX_AGE)
        .map(|age| {
            evaluate(compiler, &format!("board-color {age}"))?
                .parse::<u32>()
                .ok()
                .filter(|value| *value <= 0xff_ffff)
                .ok_or_else(|| {
                    format!("cell-color {age} must return a colour from 0x000000 to 0xffffff.")
                })
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Settings {
        ages,
        neighbours,
        colors,
        step_ms,
        wrap_edges,
        width,
        height,
    })
}

fn dimension(compiler: &mut Analysis, probe: &str, name: &str, max: u32) -> Result<u32, String> {
    evaluate(compiler, probe)?
        .parse::<u32>()
        .ok()
        .filter(|value| (1..=max).contains(value))
        .ok_or_else(|| format!("{name} must be between 1 and {max}."))
}

// Pack each short batch into one Int so the checker can evaluate it in a
// single probe. The extra digit value marks an out-of-range result.
fn packed(
    compiler: &mut Analysis,
    limit: u32,
    calls: impl Iterator<Item = String>,
) -> Result<Vec<u32>, String> {
    let base = u64::from(limit + 2);
    let parts: Vec<_> = calls
        .enumerate()
        .map(|(i, call)| format!("({call}) * {}", base.pow(i as u32)))
        .collect();
    let mut word = evaluate(compiler, &format!("({}) :: Int", parts.join(" + ")))?
        .parse::<u64>()
        .map_err(|_| "Cinch did not return the specialized values.")?;
    Ok(parts
        .iter()
        .map(|_| {
            let digit = (word % base) as u32;
            word /= base;
            digit
        })
        .collect())
}

fn evaluate(compiler: &mut Analysis, probe: &str) -> Result<String, String> {
    let (report, outcome) = compiler.update_probe("Life.Check", probe);
    if let Some(diag) = report
        .diagnostics
        .iter()
        .find(|diag| diag.severity == Severity::Error)
    {
        return Err(if diag.file == "Life.cinch" && diag.line > 0 {
            format!("Line {}: {}", diag.line, diag.message)
        } else {
            diag.message.clone()
        });
    }
    if let Some(error) = outcome.error {
        return Err(error.message);
    }
    if !report.chain_ok {
        return Err("The checked Life module dependencies are inconsistent.".into());
    }
    outcome.value.ok_or_else(|| {
        outcome
            .eval_error
            .unwrap_or_else(|| "Cinch could not evaluate the Life module.".into())
    })
}
