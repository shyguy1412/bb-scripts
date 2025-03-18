use wasm_bindgen::JsValue as _JsValue;
use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::describe::EXTERNREF;
use wasm_bindgen::describe::WasmDescribe;
use wasm_bindgen::describe::inform;

mod types;

struct JsValue(_JsValue, _JsValue);
impl JsValue {
    pub fn get(&self, key: &str) -> Self {
        let prop = js_sys::Reflect::get(&self.0, &_JsValue::from(key));

        match prop {
            Ok(v) => JsValue(v, self.0.clone()),
            Err(v) => JsValue(v, self.0.clone()),
        }
    }
}

impl Into<JsFunction> for JsValue {
    fn into(self) -> JsFunction {
        JsFunction(self.0.clone().into(), self.1.clone())
    }
}

struct JsFunction(js_sys::Function, _JsValue);

impl JsFunction {
    pub fn arg(&self, arg: JsValue) -> JsFunction {
        let this = if self.is_bound() {
            _JsValue::undefined()
        } else {
            self.1.clone()
        };

        JsFunction(self.0.bind1(&this, &arg.0), this)
    }
    pub fn call(self) -> Result<JsValue, JsValue> {
        match self.0.call0(&self.1) {
            Ok(v) => Ok(JsValue(v, _JsValue::undefined())),
            Err(v) => Err(JsValue(v, _JsValue::undefined())),
        }
    }
    fn is_bound(&self) -> bool {
        return self.0.has_own_property(&_JsValue::from("prototype"));
    }
}

pub struct NS {
    obj: JsValue,
}

impl WasmDescribe for NS {
    fn describe() {
        inform(EXTERNREF)
    }
}

impl FromWasmAbi for NS {
    type Abi = u32;

    unsafe fn from_abi(js: Self::Abi) -> Self {
        let value = unsafe { _JsValue::from_abi(js) };
        NS {
            obj: JsValue(value.clone(), value.clone()),
        }
    }
}

impl NS {
    pub fn tprint(self, message: &str) {
        let tprint: JsFunction = self.obj.get("tprint").into();
        let _ = tprint.arg(message.into()).call();
    }
}

impl Into<JsValue> for &str {
    fn into(self) -> JsValue {
        return JsValue(_JsValue::from(self), _JsValue::from(self));
    }
}
