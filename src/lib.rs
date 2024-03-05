mod utils;
mod console;

use std::borrow::Cow;
use once_cell::sync::Lazy;
use std::collections::{BTreeMap};
use wasm_bindgen::prelude::*;
use teo_parser::{parse, generate_json_diagnostics, jump_to_definition, auto_complete_items, format_document as format_document_parser};
extern crate console_error_panic_hook;
use std::panic;
use std::sync::Mutex;
use teo_parser::ast::schema::Schema;
use teo_parser::diagnostics::diagnostics::Diagnostics;
use teo_parser::utils::path::FileUtility;
use crate::utils::{file_exists_wasm, file_is_directory, parent_directory_wasm, path_is_absolute_wasm, path_join_wasm, read_file_wasm};

#[wasm_bindgen]
pub fn lint(path: &str, unsaved_files: JsValue) -> String {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let (schema, diagnostics) = parse_internal(&path, unsaved_files);
    if let Ok(mut locked) = SCHEMA_CACHE.lock() {
        locked.insert(path.to_owned(), schema);
    }
    generate_json_diagnostics(&diagnostics, true)
}

#[wasm_bindgen]
pub fn find_definitions(path: &str, line_col_range_js: JsValue) -> JsValue {
    if let Ok(locked) = SCHEMA_CACHE.lock() {
        let definitions = if let Some(schema) = locked.get(path) {
            let line_col: (usize, usize) = serde_wasm_bindgen::from_value(line_col_range_js).unwrap();
            jump_to_definition(&schema, &path, line_col)
        } else {
            vec![]
        };
        serde_wasm_bindgen::to_value(&definitions).unwrap()
    } else {
        serde_wasm_bindgen::to_value(&Vec::<String>::new()).unwrap()
    }
}

#[wasm_bindgen]
pub fn completion_items(path: &str, line_col_range_js: JsValue, unsaved_files: JsValue) -> JsValue {
    let line_col: (usize, usize) = serde_wasm_bindgen::from_value(line_col_range_js).unwrap();
    if let Ok(locked) = SCHEMA_CACHE.lock() {
        let completions = if let Some(cached_schema) = locked.get(path) {
            auto_complete_items(cached_schema, &path, line_col)
        } else {
            let (schema, _) = parse_internal(&path, unsaved_files);
            auto_complete_items(&schema, &path, line_col)
        };
        serde_wasm_bindgen::to_value(&completions).unwrap()
    } else {
        serde_wasm_bindgen::to_value(&Vec::<String>::new()).unwrap()
    }

}

#[wasm_bindgen]
pub fn remove_cached_schema(path: &str) {
    if let Ok(mut locked) = SCHEMA_CACHE.lock() {
        locked.remove(path);
    }
}

#[wasm_bindgen]
pub fn format_document(path: &str, unsaved_files: JsValue) -> String {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let (schema, diagnostics) = parse_internal(&path, unsaved_files);
    format_document_parser(&schema, path)
}

fn parse_internal(path: &str, unsaved_files: JsValue) -> (Schema, Diagnostics) {
    let unsaved_map: BTreeMap<String, String> = serde_wasm_bindgen::from_value(unsaved_files).unwrap();
    parse(path, Some(FileUtility {
        read_file: read_file_wasm,
        file_exists: file_exists_wasm,
        path_join: path_join_wasm,
        parent_directory: parent_directory_wasm,
        path_is_absolute: path_is_absolute_wasm,
        file_is_directory,
    }), Some(unsaved_map))
}

static SCHEMA_CACHE: Lazy<Mutex<BTreeMap<String, Schema>>> = Lazy::new(|| {
    Mutex::new(BTreeMap::new())
});