//! SYNX WebAssembly binding — parse SYNX in the browser.
//! Returns JSON strings (most efficient across WASM boundary).

use wasm_bindgen::prelude::*;
use synx_core::{self, Mode, Options};

/// Parse a SYNX string and return a JSON string.
#[wasm_bindgen]
pub fn parse(text: &str) -> String {
    let result = synx_core::parse(text);
    synx_core::to_json(&result.root)
}

/// Parse a SYNX string with engine resolution and return a JSON string.
/// Note: :include and :env markers have limited support in the browser.
#[wasm_bindgen]
pub fn parse_active(text: &str) -> String {
    let mut result = synx_core::parse(text);
    if result.mode == Mode::Active {
        synx_core::resolve(&mut result, &Options::default());
    }
    synx_core::to_json(&result.root)
}
