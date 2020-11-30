use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn hujlog(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    hujlog(&format!("fonarj ebanarj - {}", name));
}

/**
 * allocate `bytes` bytes in RAM and fill them with
 * random values, return reference to the allocated memory
 */
#[wasm_bindgen]
pub fn rand_malloc(length: usize) -> usize {
    let mut arr: Vec<u8> = vec![0; length];
    for i in 0..length {
        std::mem::replace(&mut arr[i], 42);
    }
    let handle = &arr as *const Vec<u8>;
    return handle as usize;
}

#[wasm_bindgen]
pub fn free_by_address(address: usize) {
    let handle = address as *const Vec<u8>;
    std::mem::drop(handle);
}

#[wasm_bindgen]
pub struct Parent {
    bytes: Vec<u8>,
}

#[wasm_bindgen]
impl Parent {
    pub fn new(length: usize) -> Self {
        let mut arr: Vec<u8> = vec![0; length];
        for i in 0..length {
            std::mem::replace(&mut arr[i], 42);
        }
        Self { bytes: arr }
    }

    pub fn xor(&self) -> u8 {
        let mut result: u8 = 0;
        for byte in &self.bytes {
            result = result ^ byte;
        }
        return result;
    }

    pub fn at(&self, pos: usize) -> u8 {
        self.bytes[pos]
    }
}
