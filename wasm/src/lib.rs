#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
extern crate wasm_bindgen;
extern crate ser_test;
use wasm_bindgen::prelude::*;
use ser_test::get_res_vec;

#[wasm_bindgen]
extern {
    type Performance;
    static performance: Performance;
    #[wasm_bindgen(method)]
    fn now(this: &Performance) -> f64;
}

#[wasm_bindgen]
pub fn run_test() -> JsValue {
    let data: Vec<String> = get_res_vec(Box::new(now), "ms");
    JsValue::from_str(&data.join("\n,"))
}

fn now() -> f64 {
    performance::now()
}