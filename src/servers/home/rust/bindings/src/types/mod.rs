mod object;
pub use object::Object;

mod function;
pub use function::Function;

mod any;
pub use any::Any;

pub trait Get<T> {
  fn get(&self, key: &str) -> Result<T, wasm_bindgen::JsValue>;
}