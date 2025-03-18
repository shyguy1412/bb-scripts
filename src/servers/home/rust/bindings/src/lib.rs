use wasm_bindgen::JsValue as _JsValue;
use js_sys::Object;
use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::describe::EXTERNREF;
use wasm_bindgen::describe::WasmDescribe;
use wasm_bindgen::describe::inform;

struct JsValue(_JsValue, Option<_JsValue>);
impl JsValue {
    pub fn get(self, key: &str) -> Self {
        let prop = js_sys::Reflect::get(&self.0, &_JsValue::from(key));

        match prop {
            Ok(v) => JsValue(v, Some(self.clone())),
            Err(v) => JsValue(v, Some(self.clone())),
        }
    }
}
impl Into<JsFunction> for JsValue {
    fn into(self) -> JsFunction {
        JsFunction(self.0.into(), self.0)
    }
}

struct JsFunction(js_sys::Function, _JsValue);

impl JsFunction {
    pub fn arg(self, arg:JsValue){
        JsFunction(self.0.bind1(se, arg1))
    }
    pub fn call(self){

    }
}

struct NSError;

impl NSError {
    pub fn new() -> Self {
        NSError {}
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
        NS {
            obj: unsafe { JsValue(wasm_bindgen::JsValue::from_abi(js)) },
        }
    }
}

impl NS {
    pub fn tprint(self, message: &str) {
        let tprint:JsFunction = self.obj.get("tprint").into();
        
        tprint.arg(message.into()).call();
        // let ns = &self.obj;
        // let tprint: Function = js_sys::Reflect::get(ns, &JsValue::from("tprint"))
        //     .unwrap()
        //     .into();
        // tprint.bind1(ns, &JsValue::from(message)).call0(ns);
    }
    pub fn new(obj: Object) -> NS {
        Self { obj }
    }
}

impl Into<JsValue> for &str{
    fn into(self) -> JsValue {
        return  JsValue(_JsValue::from(self));
    }
}
