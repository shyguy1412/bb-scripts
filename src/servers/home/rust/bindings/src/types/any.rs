use wasm_bindgen::JsValue;

pub struct Any(pub JsValue, pub JsValue);

impl Into<Any> for &str {
  fn into(self) -> Any {
      return Any(JsValue::from(self), JsValue::from(self));
  }
}