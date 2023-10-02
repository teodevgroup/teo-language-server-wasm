use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/utils.js")]
extern "C" {
    #[wasm_bindgen(js_name = readFileSync, catch)]
    fn read_file_sync(path: &str) -> Result<String, JsValue>;
}

#[wasm_bindgen(module = "fs")]
extern "C" {
    #[wasm_bindgen(js_name = existsSync, catch)]
    fn exists_sync(path: &str) -> Result<bool, JsValue>;
}

#[wasm_bindgen(module = "path")]
extern "C" {
    #[wasm_bindgen(js_name = join, catch)]
    fn join(base: &str, item: &str) -> Result<String, JsValue>;

    #[wasm_bindgen(js_name = dirname, catch)]
    fn dirname(path: &str) -> Result<String, JsValue>;

    #[wasm_bindgen(js_name = isAbsolute, catch)]
    fn is_absolute(path: &str) -> Result<bool, JsValue>;
}

pub(super) fn read_file_wasm(file_path: &str) -> Option<String> {
    match read_file_sync(file_path) {
        Ok(buffer) => {
            Some(buffer)
        },
        Err(_) => {
            None
        }
    }
}

pub(super) fn file_exists_wasm(file_path: &str) -> bool {
    match exists_sync(file_path) {
        Ok(b) => b,
        Err(_) => false,
    }
}

pub(super) fn path_join_wasm(base: &str, item: &str) -> String {
    match join(base, item) {
        Ok(s) => s,
        Err(_) => panic!("Cannot join path"),
    }
}

pub(super) fn parent_directory_wasm(file_path: &str) -> String {
    match dirname(file_path) {
        Ok(s) => s,
        Err(_) => panic!("Cannot get parent directory"),
    }
}

pub(super) fn path_is_absolute_wasm(file_path: &str) -> bool {
    match is_absolute(file_path) {
        Ok(b) => b,
        Err(_) => panic!("Cannot get directory is absolute or not"),
    }
}