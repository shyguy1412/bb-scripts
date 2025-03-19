use bitburner_bindings::*;
use wasm_bindgen::JsValue;

#[bb_bindgen]
pub fn main(ns:NS) -> Result<(), JsValue>{
  let message: String = ns.args.get(&"0")?;
  let message: std::string::String = message.into();
  ns.tprint(message.as_str())?;
  Ok(())
}
