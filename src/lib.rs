use wasm_bindgen::prelude::*;
use teo_parser::{parse, generate_json_diagnostics};

#[wasm_bindgen]
pub fn lint(file: String) -> String {
    let (_, diagnostics) = parse(file);
    generate_json_diagnostics(&diagnostics, true)
}