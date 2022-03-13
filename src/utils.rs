use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen(module = "/file-save.js")]
extern "C" {
    #[wasm_bindgen(js_name = saveFile)]
    pub fn save_file(text: &str, file_type: &str, filename: &str);
}
