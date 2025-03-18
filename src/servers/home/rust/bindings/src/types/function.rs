use wasm_bindgen::{JsError, JsValue};

use super::any::Any;

pub struct Function(js_sys::Function, JsValue);

impl Function {
    pub fn new(function: js_sys::Function, context: JsValue) -> Self {
        Function(function, context)
    }

    pub fn arg(&self, arg: Any) -> Self {
        let this = if self.is_bound() {
            JsValue::undefined()
        } else {
            self.1.clone()
        };

        Self(self.0.bind1(&this, &arg.0), this)
    }
    pub fn call(self) -> Result<Any, JsValue> {
        match self.0.call0(&self.1) {
            Ok(v) => Ok(Any(v, JsValue::undefined())),
            Err(v) => Err(v),
        }
    }
    fn is_bound(&self) -> bool {
        return self.0.has_own_property(&JsValue::from("prototype"));
    }
}

impl TryFrom<Any> for Function {
    type Error = JsError;

    fn try_from(value: Any) -> Result<Self, Self::Error> {
        if !value.0.is_function() {
            return Err(JsError::new(&"not functiony enough"));
        }

        Ok(Function(value.0.into(), value.1.clone()))
    }
}
