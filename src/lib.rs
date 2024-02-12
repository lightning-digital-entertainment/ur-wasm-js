use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct JsEncoder {
    encoder: ur::ur::Encoder<'static>,
}

#[wasm_bindgen]
impl JsEncoder {
    #[wasm_bindgen(constructor)]
    pub fn new(data: &str, max_length: usize) -> JsEncoder {
        JsEncoder {
            encoder: ur::Encoder::bytes(data.as_bytes(), max_length).unwrap(),
        }
    }

    pub fn next_value(&mut self) -> String {
        self.encoder.next_part().unwrap() // Assuming this can be called as such
    }
}
