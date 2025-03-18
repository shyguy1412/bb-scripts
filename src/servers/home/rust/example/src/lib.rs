use wasm_bindgen::prelude::*;
use bitburner_bindings::NS;

#[wasm_bindgen]
pub fn main(ns:NS) -> Result<(), JsError>{
  ns.tprint("Hello World :)2")?;
  Ok(())
}
