use std::ops::Index;

use js_sys::Function;
use js_sys::Object;
use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::describe::EXTERNREF;
use wasm_bindgen::describe::WasmDescribe;
use wasm_bindgen::describe::inform;
use wasm_bindgen::JsCast;

struct JsValue(wasm_bindgen::JsValue);

struct NSError;

impl NSError {
    pub fn new() -> Self {
        NSError {}
    }
}

pub struct NS {
    obj: Object,
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
            obj: unsafe { Object::from_abi(js) },
        }
    }
}

impl NS {
    pub fn tprint(self, message: &str) {
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

impl Index<&str> for JsValue {
    type Output = JsValue;

    fn index(&self, index: &str) -> &Self::Output {
        let prop = js_sys::Reflect::get(self.0.dyn_ref().unwrap(), &wasm_bindgen::JsValue::from(index));

        match prop {
            Ok(v) => &JsValue(v),
            Err(v) => &JsValue(v),
        }
    }
}
