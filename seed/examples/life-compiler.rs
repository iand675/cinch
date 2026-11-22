//! In-browser compiler for Life's finite rule interface.
use cinch::analysis::{Analysis, Severity};
use std::cell::RefCell;

const INPUT_CAPACITY: usize = 4096;
const PREFIX: &str = "module Life where\n  data Cell where\n    Dead :: Cell\n    Alive :: Cell\n";
const CONTRACT: &str =
    "  life-checked-rule :: Cell -> Int -> Cell\n  life-checked-rule cell count = rule cell count\n";

thread_local! {
    static COMPILER: RefCell<Analysis> = RefCell::new(Analysis::new("."));
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
        let bytes = input.get(..length).ok_or("The rule exceeds 4096 bytes.")?;
        let text = std::str::from_utf8(bytes).map_err(|_| "The rule is not valid UTF-8.")?;
        COMPILER.with(|compiler| compile(&mut compiler.borrow_mut(), text))
    });
    OUTPUT.with(|output| {
        let mut output = output.borrow_mut();
        *output = match result {
            Ok(table) => serde_json::json!({ "table": table }).to_string(),
            Err(message) => serde_json::json!({ "error": message }).to_string(),
        };
        output.len()
    })
}

fn compile(compiler: &mut Analysis, text: &str) -> Result<u32, String> {
    let mut module = PREFIX.to_owned();
    for line in text.lines() {
        module.push_str("  ");
        module.push_str(line);
        module.push('\n');
    }
    module.push_str(CONTRACT);
    compiler.set_document("Life.cinch", &module);

    // Specialize the checked pure function over its complete input domain.
    // Each bit is the result of evaluating a call in Cinch's Core evaluator.
    let probe = ["Dead", "Alive"]
        .into_iter()
        .enumerate()
        .flat_map(|(cell, name)| {
            (0..9).map(move |count| {
                let bit = 1 << (cell * 9 + count);
                format!("(case life-checked-rule {name} {count} of {{ Alive -> {bit}; Dead -> 0 }})")
            })
        })
        .collect::<Vec<_>>()
        .join(" + ");
    let (report, outcome) = compiler.update_probe("Life", &format!("({probe}) :: Int"));
    if let Some(diag) = report.diagnostics.iter().find(|diag| diag.severity == Severity::Error) {
        let editor_line = diag.line.saturating_sub(PREFIX.lines().count() as u32);
        return Err(if editor_line > 0 && editor_line <= text.lines().count() as u32 {
            format!("Line {editor_line}: {}", diag.message)
        } else {
            diag.message.clone()
        });
    }
    if let Some(error) = outcome.error {
        return Err(error.message);
    }
    if !report.chain_ok {
        return Err("The checked rule dependencies are inconsistent.".into());
    }
    outcome.value.and_then(|value| value.parse().ok()).ok_or_else(|| {
        outcome.eval_error.unwrap_or_else(|| "The rule could not be specialized.".into())
    })
}
