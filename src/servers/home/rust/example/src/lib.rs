use bitburner_bindings::*;
use wasm_bindgen::JsValue;

#[bb_bindgen]
pub async fn main(ns: NS) -> Result<(), JsValue> {
    let message: String = ns.args.get(&"0")?;
    ns.tprint(message)?;

    let promise = ns.sleep(Number::new(
        JsValue::from_f64(5000f64),
        JsValue::undefined(),
    ))?;

    let any:Any = promise.into();

    any.await?;

    // let jsval:JsValue = Into::<Any>::into(promise).clone();

    // wasm_bindgen_futures::JsFuture::from(js_sys::Promise::from(jsval.clone())).await?;

    ns.tprint(String::new(JsValue::from_str("Resolved uwu :3"), JsValue::undefined()))?;

    // Err(jsval)
    Ok(())
}
