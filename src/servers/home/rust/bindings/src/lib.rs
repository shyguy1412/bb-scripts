use wasm_bindgen::JsError;
use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::describe::EXTERNREF;
use wasm_bindgen::describe::WasmDescribe;
use wasm_bindgen::describe::inform;

mod types;
pub use types::*;

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
        // ! todo: validate ns object
        NS {
            _ns: Object::from(js),
        }
    }
}


impl NS {
    pub fn tprint(self, message: &str) -> Result<(), JsError>{
        let tprint: Function = self._ns.get("tprint")?;

        let _ = tprint.arg(message.into()).call();
        Ok(())
    }
}