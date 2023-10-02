mod utils;

use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use teo_parser::{parse, generate_json_diagnostics};
extern crate console_error_panic_hook;
use std::panic;
use teo_parser::utils::path::FileUtility;
use crate::utils::{file_exists_wasm, parent_directory_wasm, path_is_absolute_wasm, path_join_wasm, read_file_wasm};

#[wasm_bindgen]
pub fn lint(path: String, unsaved_files: JsValue) -> String {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let unsaved_hash: HashMap<String, String> = serde_wasm_bindgen::from_value(unsaved_files).unwrap();
    let (_, diagnostics) = parse(path, Some(FileUtility {
        read_file: read_file_wasm,
        file_exists: file_exists_wasm,
        path_join: path_join_wasm,
        parent_directory: parent_directory_wasm,
        path_is_absolute: path_is_absolute_wasm,
    }), Some(unsaved_hash));
    generate_json_diagnostics(&diagnostics, true)
}
