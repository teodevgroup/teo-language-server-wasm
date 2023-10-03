mod utils;

use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use teo_parser::{parse, generate_json_diagnostics, jump_to_definition};
extern crate console_error_panic_hook;
use std::panic;
use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use teo_parser::utils::path::FileUtility;
use crate::utils::{file_exists_wasm, parent_directory_wasm, path_is_absolute_wasm, path_join_wasm, read_file_wasm};

#[wasm_bindgen]
pub fn lint(path: String, unsaved_files: JsValue) -> String {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let (_, diagnostics) = parse_internal(&path, unsaved_files);
    generate_json_diagnostics(&diagnostics, true)
}

#[wasm_bindgen]
pub fn find_definitions(path: String, unsaved_files: JsValue, line_col_range_js: JsValue) -> JsValue {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let (schema, _) = parse_internal(&path, unsaved_files);
    let line_col: (usize, usize) = serde_wasm_bindgen::from_value(line_col_range_js).unwrap();
    let definitions = jump_to_definition(&schema, &path, line_col);
    serde_wasm_bindgen::to_value(&definitions).unwrap()
}

fn parse_internal(path: &str, unsaved_files: JsValue) -> (Schema, Diagnostics) {
    let unsaved_hash: HashMap<String, String> = serde_wasm_bindgen::from_value(unsaved_files).unwrap();
    parse(path, Some(FileUtility {
        read_file: read_file_wasm,
        file_exists: file_exists_wasm,
        path_join: path_join_wasm,
        parent_directory: parent_directory_wasm,
        path_is_absolute: path_is_absolute_wasm,
    }), Some(unsaved_hash))
}