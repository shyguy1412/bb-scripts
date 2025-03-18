use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::describe::EXTERNREF;
use wasm_bindgen::describe::WasmDescribe;
use wasm_bindgen::describe::inform;

mod types;
pub use types::*;
use wasm_bindgen::JsValue;

pub struct NS {
    _ns: Object,
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
            _ns: Object::from(js),
        }
    }
}


impl NS {
    pub fn tprint(self, message: &str) -> Result<(), JsValue>{
        let tprint: Function = self._ns.get("tprinst")?;

        tprint.arg(message.into()).call()?;
        Ok(())
    }
}