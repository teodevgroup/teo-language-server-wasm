mod utils;

use once_cell::sync::Lazy;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use teo_parser::{parse, generate_json_diagnostics, jump_to_definition};
extern crate console_error_panic_hook;
use std::panic;
use std::sync::Mutex;
use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use teo_parser::utils::path::FileUtility;
use crate::utils::{file_exists_wasm, parent_directory_wasm, path_is_absolute_wasm, path_join_wasm, read_file_wasm};

#[wasm_bindgen]
pub fn lint(path: &str, unsaved_files: JsValue) -> String {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let (schema, diagnostics) = parse_internal(&path, unsaved_files);
    SCHEMA_CACHE.lock().unwrap().insert(path.to_owned(), schema);
    generate_json_diagnostics(&diagnostics, true)
}

#[wasm_bindgen]
pub fn find_definitions(path: &str, line_col_range_js: JsValue) -> JsValue {
    let definitions = if let Some(schema) = SCHEMA_CACHE.lock().unwrap().get(path) {
        let line_col: (usize, usize) = serde_wasm_bindgen::from_value(line_col_range_js).unwrap();
        jump_to_definition(&schema, &path, line_col)
    } else {
        vec![]
    };
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

static SCHEMA_CACHE: Lazy<Mutex<HashMap<String, Schema>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});