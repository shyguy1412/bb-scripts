use bitburner_bindings::*;
use wasm_bindgen::prelude::*;

#[bb_bindgen]
pub async fn main(ns: &'static NS) -> Result<(), JsValue> {
    clear();

    let js_wrapper = js_sys::Function::new_with_args("fn", "setTimeout(fn)");
    let js_wrapper = Function::new(js_wrapper.into(), JsValue::undefined());
    let closure = js_closure!(|_: Any| -> Result<Any, JsValue> {
        // ns.tprint("timeout closure message".into())?;
        Ok(Any::from(()))
    });

    js_wrapper.arg(closure.into()).call()?;

    let closure = js_closure!(|_args: Any| -> Result<Any, JsValue> {
        ns.tprint("at exit message".into())?;
        Ok("args".into())
    });

    ns.atExit(closure.into(), "closure".into())?;
    ns.atExit(js_closure!(log_message).into(), "global".into())?;

    let message: String = "Hello World :)".into(); //ns.args.get(&"0")?;

    ns.tprint(message.into())?;
    // let promise1 = ns.sleep(1000f64.into())?;
    // let promise = ns.sleep(Number::new("1000".into(), JsValue::undefined()).into())?;

    // // promise1.await?;
    // promise.await?;

    ns.tprint("Resolved uwu :3".into())?;

    ns.toast(
        "toast uwu".into(),
        ns::ToastVariant::WARNING.into(),
        ().into(),
    )?;

    // Err(JsValue::undefined())
    Ok(())
}

fn log_message(args: Any) -> Result<Any, JsValue> {
    log("global function message");
    Ok(args)
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn clear();

    #[wasm_bindgen(js_namespace = console, js_name = error)]
    pub fn log_error(s: Any);
}
