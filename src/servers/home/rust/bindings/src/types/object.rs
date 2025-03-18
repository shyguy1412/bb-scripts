use wasm_bindgen::{convert::FromWasmAbi, JsError, JsValue};

use super::{Function, Get};

pub struct Object(JsValue);

impl Get<Function> for Object {
    fn get(&self, key: &str) -> Result<Function, JsValue> {
        let prop = js_sys::Reflect::get(&self.0, &JsValue::from(key)).unwrap();

        if !prop.is_function() {
            return Err(JsError::new(&("Property '".to_owned() + key + "' is not a function")).into());
        };

        Ok(Function::new(prop.into(), self.0.clone()))
    }
}

impl From<u32> for Object {
    fn from(value: u32) -> Self {
        Object(unsafe { JsValue::from_abi(value) })
    }
}
