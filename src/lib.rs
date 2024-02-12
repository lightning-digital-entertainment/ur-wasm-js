use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct UrEncoder {
    encoder: ur::ur::Encoder<'static>,
}

#[wasm_bindgen]
pub struct UrDecoder {
    decoder: ur::ur::Decoder,
}

#[wasm_bindgen]
impl UrEncoder {
    #[wasm_bindgen(constructor)]
    pub fn new(data: &str, max_length: usize) -> UrEncoder {
        UrEncoder {
            encoder: ur::Encoder::bytes(data.as_bytes(), max_length).unwrap(),
        }
    }

    pub fn next_value(&mut self) -> String {
        self.encoder.next_part().unwrap()
    }
}

#[wasm_bindgen]
impl UrDecoder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> UrDecoder {
        UrDecoder {
            decoder: ur::Decoder::default(),
        }
    }

    pub fn complete(&self) -> bool {
        self.decoder.complete()
    }
    pub fn receive(&mut self, part: &str) {
        self.decoder.receive(part).unwrap()
    }
    pub fn message(&self) -> Option<Vec<u8>> {
        self.decoder.message().unwrap()
    }
}
