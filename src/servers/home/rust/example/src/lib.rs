use wasm_bindgen::prelude::*;
use bitburner_bindings::NS;

#[wasm_bindgen]
pub fn main(ns:NS) -> Result<(), JsValue>{
  ns.tprint("Hello World :)2")?;
  Ok(())
}
