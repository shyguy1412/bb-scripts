use bitburner_bindings::*;
use wasm_bindgen::prelude::*;

#[bb_bindgen]
pub async fn main(ns: &'static NS) -> Result<(), JsValue> {
    // clear();

    let js_wrapper: Function = js_sys::Function::new_with_args("fn", "setTimeout(fn)").into();
    let closure = js_closure!(|_: Any| -> Result<Any, JsValue> {
        ns.tprint("timeout closure message")?;
        Ok(Any::from(()))
    });

    js_wrapper.arg(closure).call()?;

    ns.asleep(0)?.await?;

    let closure = js_closure!(|_args: Any| -> Result<Any, JsValue> {
        ns.tprint("at exit message")?;
        Ok("args".into())
    });

    ns.atExit(closure, "closure")?;
    ns.atExit(js_closure!(log_message), "global")?;

    ns.tprint("Hello World :)")?;
    let promise = ns.sleep(1000f64)?;

    promise.await?;

    ns.tprint("Resolved uwu :3")?;

    ns.toast("toast uwu", ns::ToastVariant::WARNING, ())?;

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
