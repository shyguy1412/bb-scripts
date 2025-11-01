use super::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsError, JsValue};

/** This struct represents any possible JS value */
pub struct Any {
    pub(crate) value: JsValue,
    pub(crate) context: JsValue,
    inner: Option<Rc<RefCell<Inner>>>,
}

/** Stolen from wasm_bindgen_futures (dont tell anyone x.x) */
struct Inner {
    result: Option<Result<Any, JsValue>>,
    task: Option<std::task::Waker>,
    callbacks: Option<(Closure<dyn FnMut(JsValue)>, Closure<dyn FnMut(JsValue)>)>,
}

impl Any {
    pub fn new(value: JsValue, context: JsValue) -> Self {
        Any {
            value,
            context,
            inner: None,
        }
    }
    pub fn unwrap(&self) -> JsValue {
        self.value.clone()
    }
}

//this is appearently unstable but there its required for FromWasmAbi which is pretty fucked
impl wasm_bindgen::describe::WasmDescribe for Any {
    fn describe() {
        wasm_bindgen::describe::inform(wasm_bindgen::describe::EXTERNREF);
    }
}

//This lets rust functions exposed to JS take Any as parameter
impl FromWasmAbi for Any {
    type Abi = u32;

    unsafe fn from_abi(js: Self::Abi) -> Self {
        let value = unsafe { JsValue::from_abi(js) };
        Any::new(value, JsValue::undefined())
    }
}

//This lets JS functions bound to rust take Any as parameter
impl IntoWasmAbi for Any {
    type Abi = u32;

    fn into_abi(self) -> Self::Abi {
        self.value.into_abi()
    }
}

impl From<JsValue> for Any {
    fn from(value: JsValue) -> Self {
        Any::new(value, JsValue::undefined())
    }
}

impl From<std::convert::Infallible> for crate::Any {
    fn from(_: std::convert::Infallible) -> Self {
        ().into()
    }
}

impl std::ops::Deref for Any {
    type Target = wasm_bindgen::JsValue;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/**
 * This macro implements casting from Any into any other type
 * Actual validation of types is handled by the corresponding wrapper struct
 */
macro_rules! try_from {
  ($(($t:ident, $ts:literal) => $m:literal)*) => ($(
    impl TryFrom<Any> for $t {
        type Error = JsValue;

        fn try_from(any: Any) -> Result<$t, Self::Error> {

            if any.js_typeof() != $ts {
                return Err(JsError::new(
                    format!(
                        $m,
                        any.value.js_typeof().as_string().unwrap()
                    )
                    .as_str(),
                ).into());
            };

            return Ok($t::new(any.value.into(), any.context));
        }
    }
  )*)
}

//fuck english so fucking much
//since error messages cant be unique I need to pass to the macro ._.
//(I also pass the corresponding typeof result cause I cant be fucked to make this a procedural macro )
try_from! {
  (Function, "function") => "Cannot cast {} into a function"
  (Object, "object") => "Cannot cast {} into an object"
  (Undefined, "undefined") => "{} is not undefined"
  (BigInt, "bigInt") =>  "Can not cast {} into a bigint"
  (Boolean, "boolean") => "Can not cast {} into a boolean"
  (Symbol, "symbol") => "Can not cast {} into a symbol"
  (String, "string") => "Can not cast {} into a string"
  (Number, "number") => "Can not cast {} into a number"
}

//Also stolen from wasm_bindgen_future :3
impl Future for Any {
    type Output = Result<Any, JsValue>;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Self::Output> {
        if self.inner.is_none() {
            //Stolen from JsFuture
            let state = Rc::new(RefCell::new(Inner {
                result: None,
                task: None,
                callbacks: None,
            }));

            fn finish(state: &RefCell<Inner>, val: Result<Any, JsValue>) {
                let task = {
                    let mut state = state.borrow_mut();
                    debug_assert!(state.callbacks.is_some());
                    debug_assert!(state.result.is_none());

                    // First up drop our closures as they'll never be invoked again and
                    // this is our chance to clean up their state.
                    drop(state.callbacks.take());

                    // Next, store the value into the internal state.
                    state.result = Some(val);
                    state.task.take()
                };

                // And then finally if any task was waiting on the value wake it up and
                // let them know it's there.
                if let Some(task) = task {
                    task.wake()
                }
            }

            let resolve = {
                let state = state.clone();
                Closure::once(move |val| finish(&state, Ok(Any::new(val, JsValue::undefined()))))
            };

            let reject = {
                let state = state.clone();
                Closure::once(move |val| finish(&state, Err(val)))
            };

            let _ = js_sys::Promise::resolve(&self.value).then2(&resolve, &reject);

            state.borrow_mut().callbacks = Some((resolve, reject));
            self.inner = Some(state);
        }

        match &self.inner {
            Some(inner) => {
                // If our value has come in then we return it...
                if let Some(val) = inner.borrow_mut().result.take() {
                    return std::task::Poll::Ready(val);
                }

                // ... otherwise we arrange ourselves to get woken up once the value
                // does come in
                inner.borrow_mut().task = Some(cx.waker().clone());
                std::task::Poll::Pending
            }
            None => panic!("wtf?"),
        }
    }
}
