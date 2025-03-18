use wasm_bindgen::prelude::*;
use bitburner_bindings::NS;

#[wasm_bindgen]
pub fn main(ns:NS) {
  ns.tprint("message");
}
