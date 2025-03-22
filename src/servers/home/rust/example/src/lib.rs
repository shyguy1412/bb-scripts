use bitburner_bindings::*;
use wasm_bindgen::JsValue;

#[bb_bindgen]
pub async fn main(ns: &'static NS) -> Result<(), JsValue> {
    let closure = js_closure!(|args: Any| -> Result<Any, JsValue> {
        ns.tprint("message".into())?;
        Ok(args)
    });

    ns.atExit(closure, "closure".into())?;
    ns.atExit(js_closure!(logMessage), "closure".into())?;

    let message: String = ns.args.get(&"0")?;

    ns.tprint(message)?;
    let promise = ns.sleep(1000f64.into())?;
    log("I am a log test");

    promise.await?;

    ns.tprint("Resolved uwu :3".into())?;


    Ok(())
}

fn logMessage(args: Any) -> Result<Any, JsValue> {
    log("I am a global function");
    Ok(Any::new(JsValue::undefined(), JsValue::undefined()))
}

#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
