#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
mod types {
    mod any {
        use super::*;
        use std::cell::RefCell;
        use std::rc::Rc;
        use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::{JsError, JsValue};
        /// This struct represents any possible JS value
        pub struct Any {
            pub(crate) value: JsValue,
            pub(crate) context: JsValue,
            inner: Option<Rc<RefCell<Inner>>>,
        }
        /// Stolen from wasm_bindgen_futures (dont tell anyone x.x)
        struct Inner {
            result: Option<Result<Any, JsValue>>,
            task: Option<std::task::Waker>,
            callbacks: Option<
                (Closure<dyn FnMut(JsValue)>, Closure<dyn FnMut(JsValue)>),
            >,
        }
        impl Any {
            pub fn new(value: JsValue, context: JsValue) -> Self {
                Any { value, context, inner: None }
            }
            pub fn unwrap(&self) -> JsValue {
                self.value.clone()
            }
        }
        impl wasm_bindgen::describe::WasmDescribe for Any {
            fn describe() {
                wasm_bindgen::describe::inform(wasm_bindgen::describe::EXTERNREF);
            }
        }
        impl FromWasmAbi for Any {
            type Abi = u32;
            unsafe fn from_abi(js: Self::Abi) -> Self {
                let value = unsafe { JsValue::from_abi(js) };
                Any::new(value, JsValue::undefined())
            }
        }
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
        impl TryFrom<Any> for Function {
            type Error = JsValue;
            fn try_from(any: Any) -> Result<Function, Self::Error> {
                if any.js_typeof() != "function" {
                    return Err(
                        JsError::new(
                                ::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "Cannot cast {0} into a function",
                                                any.value.js_typeof().as_string().unwrap(),
                                            ),
                                        );
                                        res
                                    })
                                    .as_str(),
                            )
                            .into(),
                    );
                }
                return Ok(Function::new(any.value.into(), any.context));
            }
        }
        impl TryFrom<Any> for Object {
            type Error = JsValue;
            fn try_from(any: Any) -> Result<Object, Self::Error> {
                if any.js_typeof() != "object" {
                    return Err(
                        JsError::new(
                                ::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "Cannot cast {0} into an object",
                                                any.value.js_typeof().as_string().unwrap(),
                                            ),
                                        );
                                        res
                                    })
                                    .as_str(),
                            )
                            .into(),
                    );
                }
                return Ok(Object::new(any.value.into(), any.context));
            }
        }
        impl TryFrom<Any> for Undefined {
            type Error = JsValue;
            fn try_from(any: Any) -> Result<Undefined, Self::Error> {
                if any.js_typeof() != "undefined" {
                    return Err(
                        JsError::new(
                                ::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "{0} is not undefined",
                                                any.value.js_typeof().as_string().unwrap(),
                                            ),
                                        );
                                        res
                                    })
                                    .as_str(),
                            )
                            .into(),
                    );
                }
                return Ok(Undefined::new(any.value.into(), any.context));
            }
        }
        impl TryFrom<Any> for BigInt {
            type Error = JsValue;
            fn try_from(any: Any) -> Result<BigInt, Self::Error> {
                if any.js_typeof() != "bigInt" {
                    return Err(
                        JsError::new(
                                ::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "Can not cast {0} into a bigint",
                                                any.value.js_typeof().as_string().unwrap(),
                                            ),
                                        );
                                        res
                                    })
                                    .as_str(),
                            )
                            .into(),
                    );
                }
                return Ok(BigInt::new(any.value.into(), any.context));
            }
        }
        impl TryFrom<Any> for Boolean {
            type Error = JsValue;
            fn try_from(any: Any) -> Result<Boolean, Self::Error> {
                if any.js_typeof() != "boolean" {
                    return Err(
                        JsError::new(
                                ::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "Can not cast {0} into a boolean",
                                                any.value.js_typeof().as_string().unwrap(),
                                            ),
                                        );
                                        res
                                    })
                                    .as_str(),
                            )
                            .into(),
                    );
                }
                return Ok(Boolean::new(any.value.into(), any.context));
            }
        }
        impl TryFrom<Any> for Symbol {
            type Error = JsValue;
            fn try_from(any: Any) -> Result<Symbol, Self::Error> {
                if any.js_typeof() != "symbol" {
                    return Err(
                        JsError::new(
                                ::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "Can not cast {0} into a symbol",
                                                any.value.js_typeof().as_string().unwrap(),
                                            ),
                                        );
                                        res
                                    })
                                    .as_str(),
                            )
                            .into(),
                    );
                }
                return Ok(Symbol::new(any.value.into(), any.context));
            }
        }
        impl TryFrom<Any> for String {
            type Error = JsValue;
            fn try_from(any: Any) -> Result<String, Self::Error> {
                if any.js_typeof() != "string" {
                    return Err(
                        JsError::new(
                                ::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "Can not cast {0} into a string",
                                                any.value.js_typeof().as_string().unwrap(),
                                            ),
                                        );
                                        res
                                    })
                                    .as_str(),
                            )
                            .into(),
                    );
                }
                return Ok(String::new(any.value.into(), any.context));
            }
        }
        impl TryFrom<Any> for Number {
            type Error = JsValue;
            fn try_from(any: Any) -> Result<Number, Self::Error> {
                if any.js_typeof() != "number" {
                    return Err(
                        JsError::new(
                                ::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "Can not cast {0} into a number",
                                                any.value.js_typeof().as_string().unwrap(),
                                            ),
                                        );
                                        res
                                    })
                                    .as_str(),
                            )
                            .into(),
                    );
                }
                return Ok(Number::new(any.value.into(), any.context));
            }
        }
        impl Future for Any {
            type Output = Result<Any, JsValue>;
            fn poll(
                mut self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context,
            ) -> std::task::Poll<Self::Output> {
                if self.inner.is_none() {
                    let state = Rc::new(
                        RefCell::new(Inner {
                            result: None,
                            task: None,
                            callbacks: None,
                        }),
                    );
                    fn finish(state: &RefCell<Inner>, val: Result<Any, JsValue>) {
                        let task = {
                            let mut state = state.borrow_mut();
                            if true {
                                if !state.callbacks.is_some() {
                                    ::core::panicking::panic(
                                        "assertion failed: state.callbacks.is_some()",
                                    )
                                }
                            }
                            if true {
                                if !state.result.is_none() {
                                    ::core::panicking::panic(
                                        "assertion failed: state.result.is_none()",
                                    )
                                }
                            }
                            drop(state.callbacks.take());
                            state.result = Some(val);
                            state.task.take()
                        };
                        if let Some(task) = task {
                            task.wake()
                        }
                    }
                    let resolve = {
                        let state = state.clone();
                        Closure::once(move |val| finish(
                            &state,
                            Ok(Any::new(val, JsValue::undefined())),
                        ))
                    };
                    let reject = {
                        let state = state.clone();
                        Closure::once(move |val| finish(&state, Err(val)))
                    };
                    let _ = js_sys::Promise::resolve(&self.value)
                        .then2(&resolve, &reject);
                    state.borrow_mut().callbacks = Some((resolve, reject));
                    self.inner = Some(state);
                }
                match &self.inner {
                    Some(inner) => {
                        if let Some(val) = inner.borrow_mut().result.take() {
                            return std::task::Poll::Ready(val);
                        }
                        inner.borrow_mut().task = Some(cx.waker().clone());
                        std::task::Poll::Pending
                    }
                    None => {
                        ::core::panicking::panic_fmt(format_args!("wtf?"));
                    }
                }
            }
        }
    }
    pub use any::Any;
    use wasm_bindgen::JsValue;
    pub trait Get<T> {
        fn get(&self, key: &str) -> Result<T, wasm_bindgen::JsValue>;
    }
    ///Simple wrapper for a JS
    ///string
    pub struct String {
        pub(crate) value: wasm_bindgen::JsValue,
        pub(crate) context: wasm_bindgen::JsValue,
    }
    impl std::ops::Deref for String {
        type Target = wasm_bindgen::JsValue;
        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }
    impl From<String> for Any {
        fn from(value: String) -> Any {
            Any::new(value.value.into(), value.context)
        }
    }
    impl wasm_bindgen::describe::WasmDescribe for String {
        fn describe() {
            wasm_bindgen::describe::inform(wasm_bindgen::describe::EXTERNREF);
        }
    }
    impl wasm_bindgen::convert::FromWasmAbi for String {
        type Abi = u32;
        unsafe fn from_abi(js: Self::Abi) -> Self {
            let any = unsafe { Any::from_abi(js) };
            let value: Result<String, JsValue> = any.try_into();
            match value {
                Ok(v) => v,
                Err(v) => {
                    log_error(v);
                    {
                        #[cold]
                        #[track_caller]
                        #[inline(never)]
                        const fn panic_cold_explicit() -> ! {
                            ::core::panicking::panic_explicit()
                        }
                        panic_cold_explicit();
                    }
                }
            }
        }
    }
    impl wasm_bindgen::convert::IntoWasmAbi for String {
        type Abi = u32;
        fn into_abi(self) -> Self::Abi {
            self.value.into_abi()
        }
    }
    impl IntoFuture for String {
        type Output = Result<Any, JsValue>;
        type IntoFuture = Any;
        fn into_future(self) -> Self::IntoFuture {
            self.into()
        }
    }
    impl String {
        pub fn new(
            value: wasm_bindgen::JsValue,
            context: wasm_bindgen::JsValue,
        ) -> Self {
            Self { value, context }
        }
        pub fn unwrap(&self) -> JsValue {
            self.value.clone()
        }
    }
    impl From<JsValue> for String {
        fn from(val: JsValue) -> String {
            String::new(val, JsValue::undefined())
        }
    }
    impl Get<String> for String {
        fn get(&self, key: &str) -> Result<String, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "string" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a string", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(String::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<String> for Object {
        fn get(&self, key: &str) -> Result<String, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "string" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a string", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(String::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<String> for Function {
        fn get(&self, key: &str) -> Result<String, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "string" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a string", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(String::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<String> for Boolean {
        fn get(&self, key: &str) -> Result<String, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "string" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a string", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(String::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<String> for Undefined {
        fn get(&self, key: &str) -> Result<String, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "string" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a string", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(String::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<String> for Symbol {
        fn get(&self, key: &str) -> Result<String, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "string" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a string", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(String::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<String> for BigInt {
        fn get(&self, key: &str) -> Result<String, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "string" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a string", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(String::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<String> for Number {
        fn get(&self, key: &str) -> Result<String, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "string" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a string", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(String::new(prop.into(), self.value.clone().into()))
        }
    }
    ///Simple wrapper for a JS
    ///object
    pub struct Object {
        pub(crate) value: wasm_bindgen::JsValue,
        pub(crate) context: wasm_bindgen::JsValue,
    }
    impl std::ops::Deref for Object {
        type Target = wasm_bindgen::JsValue;
        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }
    impl From<Object> for Any {
        fn from(value: Object) -> Any {
            Any::new(value.value.into(), value.context)
        }
    }
    impl wasm_bindgen::describe::WasmDescribe for Object {
        fn describe() {
            wasm_bindgen::describe::inform(wasm_bindgen::describe::EXTERNREF);
        }
    }
    impl wasm_bindgen::convert::FromWasmAbi for Object {
        type Abi = u32;
        unsafe fn from_abi(js: Self::Abi) -> Self {
            let any = unsafe { Any::from_abi(js) };
            let value: Result<Object, JsValue> = any.try_into();
            match value {
                Ok(v) => v,
                Err(v) => {
                    log_error(v);
                    {
                        #[cold]
                        #[track_caller]
                        #[inline(never)]
                        const fn panic_cold_explicit() -> ! {
                            ::core::panicking::panic_explicit()
                        }
                        panic_cold_explicit();
                    }
                }
            }
        }
    }
    impl wasm_bindgen::convert::IntoWasmAbi for Object {
        type Abi = u32;
        fn into_abi(self) -> Self::Abi {
            self.value.into_abi()
        }
    }
    impl IntoFuture for Object {
        type Output = Result<Any, JsValue>;
        type IntoFuture = Any;
        fn into_future(self) -> Self::IntoFuture {
            self.into()
        }
    }
    impl Object {
        pub fn new(
            value: wasm_bindgen::JsValue,
            context: wasm_bindgen::JsValue,
        ) -> Self {
            Self { value, context }
        }
        pub fn unwrap(&self) -> JsValue {
            self.value.clone()
        }
    }
    impl From<JsValue> for Object {
        fn from(val: JsValue) -> Object {
            Object::new(val, JsValue::undefined())
        }
    }
    impl Get<Object> for String {
        fn get(&self, key: &str) -> Result<Object, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "object" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not an object", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Object::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Object> for Object {
        fn get(&self, key: &str) -> Result<Object, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "object" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not an object", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Object::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Object> for Function {
        fn get(&self, key: &str) -> Result<Object, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "object" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not an object", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Object::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Object> for Boolean {
        fn get(&self, key: &str) -> Result<Object, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "object" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not an object", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Object::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Object> for Undefined {
        fn get(&self, key: &str) -> Result<Object, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "object" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not an object", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Object::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Object> for Symbol {
        fn get(&self, key: &str) -> Result<Object, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "object" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not an object", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Object::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Object> for BigInt {
        fn get(&self, key: &str) -> Result<Object, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "object" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not an object", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Object::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Object> for Number {
        fn get(&self, key: &str) -> Result<Object, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "object" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not an object", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Object::new(prop.into(), self.value.clone().into()))
        }
    }
    ///Simple wrapper for a JS
    ///function
    pub struct Function {
        pub(crate) value: wasm_bindgen::JsValue,
        pub(crate) context: wasm_bindgen::JsValue,
    }
    impl std::ops::Deref for Function {
        type Target = wasm_bindgen::JsValue;
        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }
    impl From<Function> for Any {
        fn from(value: Function) -> Any {
            Any::new(value.value.into(), value.context)
        }
    }
    impl wasm_bindgen::describe::WasmDescribe for Function {
        fn describe() {
            wasm_bindgen::describe::inform(wasm_bindgen::describe::EXTERNREF);
        }
    }
    impl wasm_bindgen::convert::FromWasmAbi for Function {
        type Abi = u32;
        unsafe fn from_abi(js: Self::Abi) -> Self {
            let any = unsafe { Any::from_abi(js) };
            let value: Result<Function, JsValue> = any.try_into();
            match value {
                Ok(v) => v,
                Err(v) => {
                    log_error(v);
                    {
                        #[cold]
                        #[track_caller]
                        #[inline(never)]
                        const fn panic_cold_explicit() -> ! {
                            ::core::panicking::panic_explicit()
                        }
                        panic_cold_explicit();
                    }
                }
            }
        }
    }
    impl wasm_bindgen::convert::IntoWasmAbi for Function {
        type Abi = u32;
        fn into_abi(self) -> Self::Abi {
            self.value.into_abi()
        }
    }
    impl IntoFuture for Function {
        type Output = Result<Any, JsValue>;
        type IntoFuture = Any;
        fn into_future(self) -> Self::IntoFuture {
            self.into()
        }
    }
    impl Function {
        pub fn new(
            value: wasm_bindgen::JsValue,
            context: wasm_bindgen::JsValue,
        ) -> Self {
            Self { value, context }
        }
        pub fn unwrap(&self) -> JsValue {
            self.value.clone()
        }
    }
    impl From<JsValue> for Function {
        fn from(val: JsValue) -> Function {
            Function::new(val, JsValue::undefined())
        }
    }
    impl Get<Function> for String {
        fn get(&self, key: &str) -> Result<Function, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "function" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a function", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Function::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Function> for Object {
        fn get(&self, key: &str) -> Result<Function, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "function" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a function", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Function::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Function> for Function {
        fn get(&self, key: &str) -> Result<Function, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "function" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a function", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Function::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Function> for Boolean {
        fn get(&self, key: &str) -> Result<Function, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "function" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a function", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Function::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Function> for Undefined {
        fn get(&self, key: &str) -> Result<Function, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "function" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a function", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Function::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Function> for Symbol {
        fn get(&self, key: &str) -> Result<Function, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "function" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a function", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Function::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Function> for BigInt {
        fn get(&self, key: &str) -> Result<Function, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "function" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a function", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Function::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Function> for Number {
        fn get(&self, key: &str) -> Result<Function, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "function" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a function", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Function::new(prop.into(), self.value.clone().into()))
        }
    }
    ///Simple wrapper for a JS
    ///bool
    pub struct Boolean {
        pub(crate) value: wasm_bindgen::JsValue,
        pub(crate) context: wasm_bindgen::JsValue,
    }
    impl std::ops::Deref for Boolean {
        type Target = wasm_bindgen::JsValue;
        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }
    impl From<Boolean> for Any {
        fn from(value: Boolean) -> Any {
            Any::new(value.value.into(), value.context)
        }
    }
    impl wasm_bindgen::describe::WasmDescribe for Boolean {
        fn describe() {
            wasm_bindgen::describe::inform(wasm_bindgen::describe::EXTERNREF);
        }
    }
    impl wasm_bindgen::convert::FromWasmAbi for Boolean {
        type Abi = u32;
        unsafe fn from_abi(js: Self::Abi) -> Self {
            let any = unsafe { Any::from_abi(js) };
            let value: Result<Boolean, JsValue> = any.try_into();
            match value {
                Ok(v) => v,
                Err(v) => {
                    log_error(v);
                    {
                        #[cold]
                        #[track_caller]
                        #[inline(never)]
                        const fn panic_cold_explicit() -> ! {
                            ::core::panicking::panic_explicit()
                        }
                        panic_cold_explicit();
                    }
                }
            }
        }
    }
    impl wasm_bindgen::convert::IntoWasmAbi for Boolean {
        type Abi = u32;
        fn into_abi(self) -> Self::Abi {
            self.value.into_abi()
        }
    }
    impl IntoFuture for Boolean {
        type Output = Result<Any, JsValue>;
        type IntoFuture = Any;
        fn into_future(self) -> Self::IntoFuture {
            self.into()
        }
    }
    impl Boolean {
        pub fn new(
            value: wasm_bindgen::JsValue,
            context: wasm_bindgen::JsValue,
        ) -> Self {
            Self { value, context }
        }
        pub fn unwrap(&self) -> JsValue {
            self.value.clone()
        }
    }
    impl From<JsValue> for Boolean {
        fn from(val: JsValue) -> Boolean {
            Boolean::new(val, JsValue::undefined())
        }
    }
    impl Get<Boolean> for String {
        fn get(&self, key: &str) -> Result<Boolean, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "bool" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a boolean", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Boolean::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Boolean> for Object {
        fn get(&self, key: &str) -> Result<Boolean, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "bool" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a boolean", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Boolean::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Boolean> for Function {
        fn get(&self, key: &str) -> Result<Boolean, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "bool" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a boolean", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Boolean::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Boolean> for Boolean {
        fn get(&self, key: &str) -> Result<Boolean, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "bool" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a boolean", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Boolean::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Boolean> for Undefined {
        fn get(&self, key: &str) -> Result<Boolean, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "bool" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a boolean", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Boolean::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Boolean> for Symbol {
        fn get(&self, key: &str) -> Result<Boolean, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "bool" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a boolean", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Boolean::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Boolean> for BigInt {
        fn get(&self, key: &str) -> Result<Boolean, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "bool" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a boolean", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Boolean::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Boolean> for Number {
        fn get(&self, key: &str) -> Result<Boolean, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "bool" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a boolean", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Boolean::new(prop.into(), self.value.clone().into()))
        }
    }
    ///Simple wrapper for a JS
    ///undefined
    pub struct Undefined {
        pub(crate) value: wasm_bindgen::JsValue,
        pub(crate) context: wasm_bindgen::JsValue,
    }
    impl std::ops::Deref for Undefined {
        type Target = wasm_bindgen::JsValue;
        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }
    impl From<Undefined> for Any {
        fn from(value: Undefined) -> Any {
            Any::new(value.value.into(), value.context)
        }
    }
    impl wasm_bindgen::describe::WasmDescribe for Undefined {
        fn describe() {
            wasm_bindgen::describe::inform(wasm_bindgen::describe::EXTERNREF);
        }
    }
    impl wasm_bindgen::convert::FromWasmAbi for Undefined {
        type Abi = u32;
        unsafe fn from_abi(js: Self::Abi) -> Self {
            let any = unsafe { Any::from_abi(js) };
            let value: Result<Undefined, JsValue> = any.try_into();
            match value {
                Ok(v) => v,
                Err(v) => {
                    log_error(v);
                    {
                        #[cold]
                        #[track_caller]
                        #[inline(never)]
                        const fn panic_cold_explicit() -> ! {
                            ::core::panicking::panic_explicit()
                        }
                        panic_cold_explicit();
                    }
                }
            }
        }
    }
    impl wasm_bindgen::convert::IntoWasmAbi for Undefined {
        type Abi = u32;
        fn into_abi(self) -> Self::Abi {
            self.value.into_abi()
        }
    }
    impl IntoFuture for Undefined {
        type Output = Result<Any, JsValue>;
        type IntoFuture = Any;
        fn into_future(self) -> Self::IntoFuture {
            self.into()
        }
    }
    impl Undefined {
        pub fn new(
            value: wasm_bindgen::JsValue,
            context: wasm_bindgen::JsValue,
        ) -> Self {
            Self { value, context }
        }
        pub fn unwrap(&self) -> JsValue {
            self.value.clone()
        }
    }
    impl From<JsValue> for Undefined {
        fn from(val: JsValue) -> Undefined {
            Undefined::new(val, JsValue::undefined())
        }
    }
    impl Get<Undefined> for String {
        fn get(&self, key: &str) -> Result<Undefined, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "undefined" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not undefined", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Undefined::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Undefined> for Object {
        fn get(&self, key: &str) -> Result<Undefined, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "undefined" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not undefined", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Undefined::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Undefined> for Function {
        fn get(&self, key: &str) -> Result<Undefined, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "undefined" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not undefined", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Undefined::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Undefined> for Boolean {
        fn get(&self, key: &str) -> Result<Undefined, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "undefined" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not undefined", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Undefined::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Undefined> for Undefined {
        fn get(&self, key: &str) -> Result<Undefined, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "undefined" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not undefined", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Undefined::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Undefined> for Symbol {
        fn get(&self, key: &str) -> Result<Undefined, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "undefined" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not undefined", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Undefined::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Undefined> for BigInt {
        fn get(&self, key: &str) -> Result<Undefined, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "undefined" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not undefined", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Undefined::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Undefined> for Number {
        fn get(&self, key: &str) -> Result<Undefined, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "undefined" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not undefined", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Undefined::new(prop.into(), self.value.clone().into()))
        }
    }
    ///Simple wrapper for a JS
    ///symbol
    pub struct Symbol {
        pub(crate) value: wasm_bindgen::JsValue,
        pub(crate) context: wasm_bindgen::JsValue,
    }
    impl std::ops::Deref for Symbol {
        type Target = wasm_bindgen::JsValue;
        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }
    impl From<Symbol> for Any {
        fn from(value: Symbol) -> Any {
            Any::new(value.value.into(), value.context)
        }
    }
    impl wasm_bindgen::describe::WasmDescribe for Symbol {
        fn describe() {
            wasm_bindgen::describe::inform(wasm_bindgen::describe::EXTERNREF);
        }
    }
    impl wasm_bindgen::convert::FromWasmAbi for Symbol {
        type Abi = u32;
        unsafe fn from_abi(js: Self::Abi) -> Self {
            let any = unsafe { Any::from_abi(js) };
            let value: Result<Symbol, JsValue> = any.try_into();
            match value {
                Ok(v) => v,
                Err(v) => {
                    log_error(v);
                    {
                        #[cold]
                        #[track_caller]
                        #[inline(never)]
                        const fn panic_cold_explicit() -> ! {
                            ::core::panicking::panic_explicit()
                        }
                        panic_cold_explicit();
                    }
                }
            }
        }
    }
    impl wasm_bindgen::convert::IntoWasmAbi for Symbol {
        type Abi = u32;
        fn into_abi(self) -> Self::Abi {
            self.value.into_abi()
        }
    }
    impl IntoFuture for Symbol {
        type Output = Result<Any, JsValue>;
        type IntoFuture = Any;
        fn into_future(self) -> Self::IntoFuture {
            self.into()
        }
    }
    impl Symbol {
        pub fn new(
            value: wasm_bindgen::JsValue,
            context: wasm_bindgen::JsValue,
        ) -> Self {
            Self { value, context }
        }
        pub fn unwrap(&self) -> JsValue {
            self.value.clone()
        }
    }
    impl From<JsValue> for Symbol {
        fn from(val: JsValue) -> Symbol {
            Symbol::new(val, JsValue::undefined())
        }
    }
    impl Get<Symbol> for String {
        fn get(&self, key: &str) -> Result<Symbol, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "symbol" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a symbol", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Symbol::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Symbol> for Object {
        fn get(&self, key: &str) -> Result<Symbol, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "symbol" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a symbol", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Symbol::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Symbol> for Function {
        fn get(&self, key: &str) -> Result<Symbol, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "symbol" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a symbol", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Symbol::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Symbol> for Boolean {
        fn get(&self, key: &str) -> Result<Symbol, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "symbol" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a symbol", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Symbol::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Symbol> for Undefined {
        fn get(&self, key: &str) -> Result<Symbol, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "symbol" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a symbol", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Symbol::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Symbol> for Symbol {
        fn get(&self, key: &str) -> Result<Symbol, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "symbol" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a symbol", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Symbol::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Symbol> for BigInt {
        fn get(&self, key: &str) -> Result<Symbol, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "symbol" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a symbol", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Symbol::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Symbol> for Number {
        fn get(&self, key: &str) -> Result<Symbol, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "symbol" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a symbol", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Symbol::new(prop.into(), self.value.clone().into()))
        }
    }
    ///Simple wrapper for a JS
    ///bigint
    pub struct BigInt {
        pub(crate) value: wasm_bindgen::JsValue,
        pub(crate) context: wasm_bindgen::JsValue,
    }
    impl std::ops::Deref for BigInt {
        type Target = wasm_bindgen::JsValue;
        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }
    impl From<BigInt> for Any {
        fn from(value: BigInt) -> Any {
            Any::new(value.value.into(), value.context)
        }
    }
    impl wasm_bindgen::describe::WasmDescribe for BigInt {
        fn describe() {
            wasm_bindgen::describe::inform(wasm_bindgen::describe::EXTERNREF);
        }
    }
    impl wasm_bindgen::convert::FromWasmAbi for BigInt {
        type Abi = u32;
        unsafe fn from_abi(js: Self::Abi) -> Self {
            let any = unsafe { Any::from_abi(js) };
            let value: Result<BigInt, JsValue> = any.try_into();
            match value {
                Ok(v) => v,
                Err(v) => {
                    log_error(v);
                    {
                        #[cold]
                        #[track_caller]
                        #[inline(never)]
                        const fn panic_cold_explicit() -> ! {
                            ::core::panicking::panic_explicit()
                        }
                        panic_cold_explicit();
                    }
                }
            }
        }
    }
    impl wasm_bindgen::convert::IntoWasmAbi for BigInt {
        type Abi = u32;
        fn into_abi(self) -> Self::Abi {
            self.value.into_abi()
        }
    }
    impl IntoFuture for BigInt {
        type Output = Result<Any, JsValue>;
        type IntoFuture = Any;
        fn into_future(self) -> Self::IntoFuture {
            self.into()
        }
    }
    impl BigInt {
        pub fn new(
            value: wasm_bindgen::JsValue,
            context: wasm_bindgen::JsValue,
        ) -> Self {
            Self { value, context }
        }
        pub fn unwrap(&self) -> JsValue {
            self.value.clone()
        }
    }
    impl From<JsValue> for BigInt {
        fn from(val: JsValue) -> BigInt {
            BigInt::new(val, JsValue::undefined())
        }
    }
    impl Get<BigInt> for String {
        fn get(&self, key: &str) -> Result<BigInt, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "bigint" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a bigInt", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(BigInt::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<BigInt> for Object {
        fn get(&self, key: &str) -> Result<BigInt, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "bigint" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a bigInt", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(BigInt::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<BigInt> for Function {
        fn get(&self, key: &str) -> Result<BigInt, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "bigint" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a bigInt", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(BigInt::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<BigInt> for Boolean {
        fn get(&self, key: &str) -> Result<BigInt, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "bigint" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a bigInt", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(BigInt::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<BigInt> for Undefined {
        fn get(&self, key: &str) -> Result<BigInt, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "bigint" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a bigInt", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(BigInt::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<BigInt> for Symbol {
        fn get(&self, key: &str) -> Result<BigInt, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "bigint" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a bigInt", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(BigInt::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<BigInt> for BigInt {
        fn get(&self, key: &str) -> Result<BigInt, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "bigint" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a bigInt", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(BigInt::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<BigInt> for Number {
        fn get(&self, key: &str) -> Result<BigInt, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "bigint" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a bigInt", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(BigInt::new(prop.into(), self.value.clone().into()))
        }
    }
    ///Simple wrapper for a JS
    ///number
    pub struct Number {
        pub(crate) value: wasm_bindgen::JsValue,
        pub(crate) context: wasm_bindgen::JsValue,
    }
    impl std::ops::Deref for Number {
        type Target = wasm_bindgen::JsValue;
        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }
    impl From<Number> for Any {
        fn from(value: Number) -> Any {
            Any::new(value.value.into(), value.context)
        }
    }
    impl wasm_bindgen::describe::WasmDescribe for Number {
        fn describe() {
            wasm_bindgen::describe::inform(wasm_bindgen::describe::EXTERNREF);
        }
    }
    impl wasm_bindgen::convert::FromWasmAbi for Number {
        type Abi = u32;
        unsafe fn from_abi(js: Self::Abi) -> Self {
            let any = unsafe { Any::from_abi(js) };
            let value: Result<Number, JsValue> = any.try_into();
            match value {
                Ok(v) => v,
                Err(v) => {
                    log_error(v);
                    {
                        #[cold]
                        #[track_caller]
                        #[inline(never)]
                        const fn panic_cold_explicit() -> ! {
                            ::core::panicking::panic_explicit()
                        }
                        panic_cold_explicit();
                    }
                }
            }
        }
    }
    impl wasm_bindgen::convert::IntoWasmAbi for Number {
        type Abi = u32;
        fn into_abi(self) -> Self::Abi {
            self.value.into_abi()
        }
    }
    impl IntoFuture for Number {
        type Output = Result<Any, JsValue>;
        type IntoFuture = Any;
        fn into_future(self) -> Self::IntoFuture {
            self.into()
        }
    }
    impl Number {
        pub fn new(
            value: wasm_bindgen::JsValue,
            context: wasm_bindgen::JsValue,
        ) -> Self {
            Self { value, context }
        }
        pub fn unwrap(&self) -> JsValue {
            self.value.clone()
        }
    }
    impl From<JsValue> for Number {
        fn from(val: JsValue) -> Number {
            Number::new(val, JsValue::undefined())
        }
    }
    impl Get<Number> for String {
        fn get(&self, key: &str) -> Result<Number, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "number" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a number", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Number::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Number> for Object {
        fn get(&self, key: &str) -> Result<Number, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "number" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a number", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Number::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Number> for Function {
        fn get(&self, key: &str) -> Result<Number, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "number" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a number", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Number::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Number> for Boolean {
        fn get(&self, key: &str) -> Result<Number, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "number" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a number", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Number::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Number> for Undefined {
        fn get(&self, key: &str) -> Result<Number, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "number" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a number", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Number::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Number> for Symbol {
        fn get(&self, key: &str) -> Result<Number, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "number" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a number", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Number::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Number> for BigInt {
        fn get(&self, key: &str) -> Result<Number, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "number" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a number", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Number::new(prop.into(), self.value.clone().into()))
        }
    }
    impl Get<Number> for Number {
        fn get(&self, key: &str) -> Result<Number, wasm_bindgen::JsValue> {
            let prop = js_sys::Reflect::get(
                    &self.value,
                    &wasm_bindgen::JsValue::from(key),
                )
                .unwrap();
            if prop.js_typeof() != "number" {
                return Err(
                    wasm_bindgen::JsError::new(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Property {0} is not a number", key),
                                    );
                                    res
                                })
                                .as_str(),
                        )
                        .into(),
                );
            }
            Ok(Number::new(prop.into(), self.value.clone().into()))
        }
    }
    #[allow(nonstandard_style)]
    #[allow(clippy::all, clippy::nursery, clippy::pedantic, clippy::restriction)]
    pub fn log_error(s: JsValue) {
        unsafe fn __wbg_error_03e080b7a953296e(
            s_1: <<JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
            s_2: <<JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
            s_3: <<JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
            s_4: <<JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
        ) -> () {
            drop(s_1);
            drop(s_2);
            drop(s_3);
            drop(s_4);
            {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "cannot call wasm-bindgen imported functions on non-wasm targets",
                    ),
                );
            };
        }
        unsafe {
            let _ret = {
                let s = <JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(s);
                let (s_1, s_2, s_3, s_4) = <<JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::split(
                    s,
                );
                __wbg_error_03e080b7a953296e(s_1, s_2, s_3, s_4)
            };
            ()
        }
    }
    impl Function {
        pub fn from(value: Box<dyn Fn(js_sys::Array) -> JsValue>) -> Self {
            let js_wrapper = js_sys::Function::new_with_args(
                "fn",
                "return (...args) => fn(args)",
            );
            let closure = wasm_bindgen::closure::Closure::wrap(value).into_js_value();
            let closure = js_wrapper
                .call1(&JsValue::undefined(), &closure)
                .expect("If this throws black magic fuckery happened");
            Function::new(closure, JsValue::undefined())
        }
        pub fn arg(self, arg: Any) -> Self {
            let context = if self.is_bound() {
                wasm_bindgen::JsValue::undefined()
            } else {
                self.context.clone()
            };
            let this: js_sys::Function = self.value.into();
            Self {
                value: this.bind1(&context, &arg.value).into(),
                context,
            }
        }
        pub fn call(&self) -> Result<Any, wasm_bindgen::JsValue> {
            let this: js_sys::Function = self.value.clone().into();
            match this.call0(&self.context) {
                Ok(v) => Ok(Any::new(v, wasm_bindgen::JsValue::undefined())),
                Err(v) => Err(v),
            }
        }
        fn is_bound(&self) -> bool {
            let this: js_sys::Function = self.value.clone().into();
            return !this.has_own_property(&wasm_bindgen::JsValue::from("prototype"));
        }
    }
    impl From<f32> for Number {
        fn from(v: f32) -> Self {
            <Number>::new(JsValue::from(v), JsValue::undefined())
        }
    }
    impl From<f32> for Any {
        fn from(v: f32) -> Self {
            Any::new(JsValue::from(v), JsValue::undefined())
        }
    }
    impl From<f64> for Number {
        fn from(v: f64) -> Self {
            <Number>::new(JsValue::from(v), JsValue::undefined())
        }
    }
    impl From<f64> for Any {
        fn from(v: f64) -> Self {
            Any::new(JsValue::from(v), JsValue::undefined())
        }
    }
    impl From<i64> for Number {
        fn from(v: i64) -> Self {
            <Number>::new(JsValue::from(v), JsValue::undefined())
        }
    }
    impl From<i64> for Any {
        fn from(v: i64) -> Self {
            Any::new(JsValue::from(v), JsValue::undefined())
        }
    }
    impl From<i32> for Number {
        fn from(v: i32) -> Self {
            <Number>::new(JsValue::from(v), JsValue::undefined())
        }
    }
    impl From<i32> for Any {
        fn from(v: i32) -> Self {
            Any::new(JsValue::from(v), JsValue::undefined())
        }
    }
    impl From<i16> for Number {
        fn from(v: i16) -> Self {
            <Number>::new(JsValue::from(v), JsValue::undefined())
        }
    }
    impl From<i16> for Any {
        fn from(v: i16) -> Self {
            Any::new(JsValue::from(v), JsValue::undefined())
        }
    }
    impl From<i8> for Number {
        fn from(v: i8) -> Self {
            <Number>::new(JsValue::from(v), JsValue::undefined())
        }
    }
    impl From<i8> for Any {
        fn from(v: i8) -> Self {
            Any::new(JsValue::from(v), JsValue::undefined())
        }
    }
    impl From<u64> for Number {
        fn from(v: u64) -> Self {
            <Number>::new(JsValue::from(v), JsValue::undefined())
        }
    }
    impl From<u64> for Any {
        fn from(v: u64) -> Self {
            Any::new(JsValue::from(v), JsValue::undefined())
        }
    }
    impl From<u32> for Number {
        fn from(v: u32) -> Self {
            <Number>::new(JsValue::from(v), JsValue::undefined())
        }
    }
    impl From<u32> for Any {
        fn from(v: u32) -> Self {
            Any::new(JsValue::from(v), JsValue::undefined())
        }
    }
    impl From<u16> for Number {
        fn from(v: u16) -> Self {
            <Number>::new(JsValue::from(v), JsValue::undefined())
        }
    }
    impl From<u16> for Any {
        fn from(v: u16) -> Self {
            Any::new(JsValue::from(v), JsValue::undefined())
        }
    }
    impl From<u8> for Number {
        fn from(v: u8) -> Self {
            <Number>::new(JsValue::from(v), JsValue::undefined())
        }
    }
    impl From<u8> for Any {
        fn from(v: u8) -> Self {
            Any::new(JsValue::from(v), JsValue::undefined())
        }
    }
    impl From<usize> for Number {
        fn from(v: usize) -> Self {
            <Number>::new(JsValue::from(v), JsValue::undefined())
        }
    }
    impl From<usize> for Any {
        fn from(v: usize) -> Self {
            Any::new(JsValue::from(v), JsValue::undefined())
        }
    }
    impl From<&str> for String {
        fn from(v: &str) -> Self {
            <String>::new(JsValue::from(v), JsValue::undefined())
        }
    }
    impl From<&str> for Any {
        fn from(v: &str) -> Self {
            Any::new(JsValue::from(v), JsValue::undefined())
        }
    }
    impl From<std::string::String> for String {
        fn from(v: std::string::String) -> Self {
            <String>::new(JsValue::from(v), JsValue::undefined())
        }
    }
    impl From<std::string::String> for Any {
        fn from(v: std::string::String) -> Self {
            Any::new(JsValue::from(v), JsValue::undefined())
        }
    }
    impl From<()> for Undefined {
        fn from(_: ()) -> Self {
            Undefined::new(JsValue::undefined(), JsValue::undefined())
        }
    }
    impl From<()> for Any {
        fn from(_: ()) -> Self {
            Any::new(JsValue::undefined(), JsValue::undefined())
        }
    }
}
pub use types::*;
pub use bitburner_bindings_macros::{bb_bindgen, from_dts};
pub fn v4uuid() -> String {
    let my_uuid = uuid::Uuid::new_v4().as_hyphenated().to_string().as_str().to_owned();
    my_uuid.into()
}
pub use ns::NS;
pub mod ns {
    use super::*;
    pub struct HP {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for HP {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<HP, Self::Error> {
            Ok(HP {
                internal: Object::from(value),
            })
        }
    }
    impl HP {}
    pub struct Skills {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for Skills {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<Skills, Self::Error> {
            Ok(Skills {
                internal: Object::from(value),
            })
        }
    }
    impl Skills {}
    struct ScriptArg {}
    struct FilenameOrPID {}
    pub struct Person {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for Person {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<Person, Self::Error> {
            Ok(Person {
                internal: Object::from(value),
            })
        }
    }
    impl Person {}
    pub struct Player {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for Player {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<Player, Self::Error> {
            Ok(Player {
                internal: Object::from(value),
            })
        }
    }
    impl Player {}
    pub struct SleevePerson {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for SleevePerson {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<SleevePerson, Self::Error> {
            Ok(SleevePerson {
                internal: Object::from(value),
            })
        }
    }
    impl SleevePerson {}
    pub struct ResetInfo {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for ResetInfo {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<ResetInfo, Self::Error> {
            Ok(ResetInfo {
                internal: Object::from(value),
            })
        }
    }
    impl ResetInfo {}
    pub struct MoneySource {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for MoneySource {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<MoneySource, Self::Error> {
            Ok(MoneySource {
                internal: Object::from(value),
            })
        }
    }
    impl MoneySource {}
    pub struct MoneySources {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for MoneySources {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<MoneySources, Self::Error> {
            Ok(MoneySources {
                internal: Object::from(value),
            })
        }
    }
    impl MoneySources {}
    pub struct Multipliers {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for Multipliers {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<Multipliers, Self::Error> {
            Ok(Multipliers {
                internal: Object::from(value),
            })
        }
    }
    impl Multipliers {}
    pub struct TailProperties {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for TailProperties {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<TailProperties, Self::Error> {
            Ok(TailProperties {
                internal: Object::from(value),
            })
        }
    }
    impl TailProperties {}
    struct ReactNode {}
    pub struct ReactElement {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for ReactElement {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<ReactElement, Self::Error> {
            Ok(ReactElement {
                internal: Object::from(value),
            })
        }
    }
    impl ReactElement {}
    pub struct RunningScript {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for RunningScript {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<RunningScript, Self::Error> {
            Ok(RunningScript {
                internal: Object::from(value),
            })
        }
    }
    impl RunningScript {}
    pub struct RunOptions {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for RunOptions {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<RunOptions, Self::Error> {
            Ok(RunOptions {
                internal: Object::from(value),
            })
        }
    }
    impl RunOptions {}
    pub struct SpawnOptions {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for SpawnOptions {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<SpawnOptions, Self::Error> {
            Ok(SpawnOptions {
                internal: Object::from(value),
            })
        }
    }
    impl SpawnOptions {}
    pub struct RecentScript {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for RecentScript {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<RecentScript, Self::Error> {
            Ok(RecentScript {
                internal: Object::from(value),
            })
        }
    }
    impl RecentScript {}
    pub struct CrimeStats {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for CrimeStats {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<CrimeStats, Self::Error> {
            Ok(CrimeStats {
                internal: Object::from(value),
            })
        }
    }
    impl CrimeStats {}
    pub struct BasicHGWOptions {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for BasicHGWOptions {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<BasicHGWOptions, Self::Error> {
            Ok(BasicHGWOptions {
                internal: Object::from(value),
            })
        }
    }
    impl BasicHGWOptions {}
    pub struct AugmentPair {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for AugmentPair {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<AugmentPair, Self::Error> {
            Ok(AugmentPair {
                internal: Object::from(value),
            })
        }
    }
    impl AugmentPair {}
    pub enum PositionType {
        Long,
        Short,
    }
    impl PositionType {
        fn as_any(&self) -> crate::Any {
            match self {
                PositionType::Long => "L".into(),
                PositionType::Short => "S".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    impl From<PositionType> for crate::Any {
        fn from(value: PositionType) -> crate::Any {
            match value {
                PositionType::Long => "L".into(),
                PositionType::Short => "S".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    pub enum OrderType {
        LimitBuy,
        LimitSell,
        StopBuy,
        StopSell,
    }
    impl OrderType {
        fn as_any(&self) -> crate::Any {
            match self {
                OrderType::LimitBuy => "Limit Buy Order".into(),
                OrderType::LimitSell => "Limit Sell Order".into(),
                OrderType::StopBuy => "Stop Buy Order".into(),
                OrderType::StopSell => "Stop Sell Order".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    impl From<OrderType> for crate::Any {
        fn from(value: OrderType) -> crate::Any {
            match value {
                OrderType::LimitBuy => "Limit Buy Order".into(),
                OrderType::LimitSell => "Limit Sell Order".into(),
                OrderType::StopBuy => "Stop Buy Order".into(),
                OrderType::StopSell => "Stop Sell Order".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    pub struct StockOrderObject {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for StockOrderObject {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<StockOrderObject, Self::Error> {
            Ok(StockOrderObject {
                internal: Object::from(value),
            })
        }
    }
    impl StockOrderObject {}
    pub struct StockOrder {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for StockOrder {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<StockOrder, Self::Error> {
            Ok(StockOrder {
                internal: Object::from(value),
            })
        }
    }
    impl StockOrder {}
    pub struct StockMarketConstants {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for StockMarketConstants {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<StockMarketConstants, Self::Error> {
            Ok(StockMarketConstants {
                internal: Object::from(value),
            })
        }
    }
    impl StockMarketConstants {}
    pub struct ProcessInfo {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for ProcessInfo {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<ProcessInfo, Self::Error> {
            Ok(ProcessInfo {
                internal: Object::from(value),
            })
        }
    }
    impl ProcessInfo {}
    pub struct HackingMultipliers {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for HackingMultipliers {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<HackingMultipliers, Self::Error> {
            Ok(HackingMultipliers {
                internal: Object::from(value),
            })
        }
    }
    impl HackingMultipliers {}
    pub struct HacknetMultipliers {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for HacknetMultipliers {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<HacknetMultipliers, Self::Error> {
            Ok(HacknetMultipliers {
                internal: Object::from(value),
            })
        }
    }
    impl HacknetMultipliers {}
    pub struct HacknetNodeConstants {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for HacknetNodeConstants {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<HacknetNodeConstants, Self::Error> {
            Ok(HacknetNodeConstants {
                internal: Object::from(value),
            })
        }
    }
    impl HacknetNodeConstants {}
    pub struct HacknetServerConstants {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for HacknetServerConstants {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<HacknetServerConstants, Self::Error> {
            Ok(HacknetServerConstants {
                internal: Object::from(value),
            })
        }
    }
    impl HacknetServerConstants {}
    pub struct Server {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for Server {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<Server, Self::Error> {
            Ok(Server {
                internal: Object::from(value),
            })
        }
    }
    impl Server {}
    pub struct BitNodeMultipliers {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for BitNodeMultipliers {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<BitNodeMultipliers, Self::Error> {
            Ok(BitNodeMultipliers {
                internal: Object::from(value),
            })
        }
    }
    impl BitNodeMultipliers {}
    pub struct NodeStats {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for NodeStats {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<NodeStats, Self::Error> {
            Ok(NodeStats {
                internal: Object::from(value),
            })
        }
    }
    impl NodeStats {}
    pub struct SourceFileLvl {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for SourceFileLvl {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<SourceFileLvl, Self::Error> {
            Ok(SourceFileLvl {
                internal: Object::from(value),
            })
        }
    }
    impl SourceFileLvl {}
    pub struct BladeburnerCurAction {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for BladeburnerCurAction {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<BladeburnerCurAction, Self::Error> {
            Ok(BladeburnerCurAction {
                internal: Object::from(value),
            })
        }
    }
    impl BladeburnerCurAction {}
    pub struct GangGenInfo {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for GangGenInfo {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<GangGenInfo, Self::Error> {
            Ok(GangGenInfo {
                internal: Object::from(value),
            })
        }
    }
    impl GangGenInfo {}
    pub struct GangOtherInfoObject {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for GangOtherInfoObject {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<GangOtherInfoObject, Self::Error> {
            Ok(GangOtherInfoObject {
                internal: Object::from(value),
            })
        }
    }
    impl GangOtherInfoObject {}
    pub struct GangTaskStats {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for GangTaskStats {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<GangTaskStats, Self::Error> {
            Ok(GangTaskStats {
                internal: Object::from(value),
            })
        }
    }
    impl GangTaskStats {}
    pub struct EquipmentStats {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for EquipmentStats {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<EquipmentStats, Self::Error> {
            Ok(EquipmentStats {
                internal: Object::from(value),
            })
        }
    }
    impl EquipmentStats {}
    pub struct GangTerritory {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for GangTerritory {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<GangTerritory, Self::Error> {
            Ok(GangTerritory {
                internal: Object::from(value),
            })
        }
    }
    impl GangTerritory {}
    pub struct GangMemberExpGain {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for GangMemberExpGain {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<GangMemberExpGain, Self::Error> {
            Ok(GangMemberExpGain {
                internal: Object::from(value),
            })
        }
    }
    impl GangMemberExpGain {}
    pub struct GangMemberInfo {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for GangMemberInfo {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<GangMemberInfo, Self::Error> {
            Ok(GangMemberInfo {
                internal: Object::from(value),
            })
        }
    }
    impl GangMemberInfo {}
    pub struct GangMemberInstall {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for GangMemberInstall {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<GangMemberInstall, Self::Error> {
            Ok(GangMemberInstall {
                internal: Object::from(value),
            })
        }
    }
    impl GangMemberInstall {}
    pub struct GangMemberAscension {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for GangMemberAscension {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<GangMemberAscension, Self::Error> {
            Ok(GangMemberAscension {
                internal: Object::from(value),
            })
        }
    }
    impl GangMemberAscension {}
    struct SleeveBladeburnerTask {}
    struct SleeveClassTask {}
    struct SleeveCompanyTask {}
    struct SleeveCrimeTask {}
    struct SleeveFactionTask {}
    struct SleeveInfiltrateTask {}
    struct SleeveRecoveryTask {}
    struct SleeveSupportTask {}
    struct SleeveSynchroTask {}
    struct SleeveTask {}
    pub struct NetscriptPort {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for NetscriptPort {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<NetscriptPort, Self::Error> {
            Ok(NetscriptPort {
                internal: Object::from(value),
            })
        }
    }
    impl NetscriptPort {
        pub fn write<A: Into<crate::Any>>(
            &self,
            value: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let write: crate::Function = self.internal.get("write")?;
            let result = write.arg(value.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn tryWrite<A: Into<crate::Any>>(
            &self,
            value: A,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let tryWrite: crate::Function = self.internal.get("tryWrite")?;
            let result = tryWrite.arg(value.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn nextWrite(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let nextWrite: crate::Function = self.internal.get("nextWrite")?;
            let result = nextWrite.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn read(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let read: crate::Function = self.internal.get("read")?;
            let result = read.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn peek(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let peek: crate::Function = self.internal.get("peek")?;
            let result = peek.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn full(&self) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let full: crate::Function = self.internal.get("full")?;
            let result = full.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn empty(&self) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let empty: crate::Function = self.internal.get("empty")?;
            let result = empty.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn clear(&self) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let clear: crate::Function = self.internal.get("clear")?;
            let result = clear.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
    }
    pub struct TIX {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for TIX {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<TIX, Self::Error> {
            Ok(TIX {
                internal: Object::from(value),
            })
        }
    }
    impl TIX {
        pub fn getConstants(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getConstants: crate::Function = self.internal.get("getConstants")?;
            let result = getConstants.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn hasWSEAccount(&self) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let hasWSEAccount: crate::Function = self.internal.get("hasWSEAccount")?;
            let result = hasWSEAccount.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn hasTIXAPIAccess(&self) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let hasTIXAPIAccess: crate::Function = self.internal.get("hasTIXAPIAccess")?;
            let result = hasTIXAPIAccess.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn has4SData(&self) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let has4SData: crate::Function = self.internal.get("has4SData")?;
            let result = has4SData.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn has4SDataTIXAPI(&self) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let has4SDataTIXAPI: crate::Function = self.internal.get("has4SDataTIXAPI")?;
            let result = has4SDataTIXAPI.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getSymbols(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getSymbols: crate::Function = self.internal.get("getSymbols")?;
            let result = getSymbols.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getPrice<A: Into<crate::String>>(
            &self,
            sym: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getPrice: crate::Function = self.internal.get("getPrice")?;
            let result = getPrice.arg(sym.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getOrganization<A: Into<crate::String>>(
            &self,
            sym: A,
        ) -> Result<crate::String, wasm_bindgen::JsValue> {
            let getOrganization: crate::Function = self.internal.get("getOrganization")?;
            let result = getOrganization.arg(sym.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getAskPrice<A: Into<crate::String>>(
            &self,
            sym: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getAskPrice: crate::Function = self.internal.get("getAskPrice")?;
            let result = getAskPrice.arg(sym.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getBidPrice<A: Into<crate::String>>(
            &self,
            sym: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getBidPrice: crate::Function = self.internal.get("getBidPrice")?;
            let result = getBidPrice.arg(sym.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getPosition<A: Into<crate::String>>(
            &self,
            sym: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getPosition: crate::Function = self.internal.get("getPosition")?;
            let result = getPosition.arg(sym.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getMaxShares<A: Into<crate::String>>(
            &self,
            sym: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getMaxShares: crate::Function = self.internal.get("getMaxShares")?;
            let result = getMaxShares.arg(sym.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getPurchaseCost<
            A: Into<crate::String>,
            B: Into<crate::Number>,
            C: Into<crate::String>,
        >(
            &self,
            sym: A,
            shares: B,
            posType: C,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getPurchaseCost: crate::Function = self.internal.get("getPurchaseCost")?;
            let result = getPurchaseCost
                .arg(sym.into())
                .arg(shares.into())
                .arg(posType.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getSaleGain<
            A: Into<crate::String>,
            B: Into<crate::Number>,
            C: Into<crate::String>,
        >(
            &self,
            sym: A,
            shares: B,
            posType: C,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getSaleGain: crate::Function = self.internal.get("getSaleGain")?;
            let result = getSaleGain
                .arg(sym.into())
                .arg(shares.into())
                .arg(posType.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn buyStock<A: Into<crate::String>, B: Into<crate::Number>>(
            &self,
            sym: A,
            shares: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let buyStock: crate::Function = self.internal.get("buyStock")?;
            let result = buyStock.arg(sym.into()).arg(shares.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn sellStock<A: Into<crate::String>, B: Into<crate::Number>>(
            &self,
            sym: A,
            shares: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let sellStock: crate::Function = self.internal.get("sellStock")?;
            let result = sellStock.arg(sym.into()).arg(shares.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn buyShort<A: Into<crate::String>, B: Into<crate::Number>>(
            &self,
            sym: A,
            shares: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let buyShort: crate::Function = self.internal.get("buyShort")?;
            let result = buyShort.arg(sym.into()).arg(shares.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn sellShort<A: Into<crate::String>, B: Into<crate::Number>>(
            &self,
            sym: A,
            shares: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let sellShort: crate::Function = self.internal.get("sellShort")?;
            let result = sellShort.arg(sym.into()).arg(shares.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn placeOrder<
            A: Into<crate::String>,
            B: Into<crate::Number>,
            C: Into<crate::Number>,
            D: Into<crate::String>,
            E: Into<crate::String>,
        >(
            &self,
            sym: A,
            shares: B,
            price: C,
            _type: D,
            pos: E,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let placeOrder: crate::Function = self.internal.get("placeOrder")?;
            let result = placeOrder
                .arg(sym.into())
                .arg(shares.into())
                .arg(price.into())
                .arg(_type.into())
                .arg(pos.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn cancelOrder<
            A: Into<crate::String>,
            B: Into<crate::Number>,
            C: Into<crate::Number>,
            D: Into<crate::String>,
            E: Into<crate::String>,
        >(
            &self,
            sym: A,
            shares: B,
            price: C,
            _type: D,
            pos: E,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let cancelOrder: crate::Function = self.internal.get("cancelOrder")?;
            let result = cancelOrder
                .arg(sym.into())
                .arg(shares.into())
                .arg(price.into())
                .arg(_type.into())
                .arg(pos.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getOrders(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getOrders: crate::Function = self.internal.get("getOrders")?;
            let result = getOrders.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getVolatility<A: Into<crate::String>>(
            &self,
            sym: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getVolatility: crate::Function = self.internal.get("getVolatility")?;
            let result = getVolatility.arg(sym.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getForecast<A: Into<crate::String>>(
            &self,
            sym: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getForecast: crate::Function = self.internal.get("getForecast")?;
            let result = getForecast.arg(sym.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn purchase4SMarketData(
            &self,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let purchase4SMarketData: crate::Function = self
                .internal
                .get("purchase4SMarketData")?;
            let result = purchase4SMarketData.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn purchase4SMarketDataTixApi(
            &self,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let purchase4SMarketDataTixApi: crate::Function = self
                .internal
                .get("purchase4SMarketDataTixApi")?;
            let result = purchase4SMarketDataTixApi.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn purchaseWseAccount(
            &self,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let purchaseWseAccount: crate::Function = self
                .internal
                .get("purchaseWseAccount")?;
            let result = purchaseWseAccount.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn purchaseTixApi(&self) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let purchaseTixApi: crate::Function = self.internal.get("purchaseTixApi")?;
            let result = purchaseTixApi.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getBonusTime(&self) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getBonusTime: crate::Function = self.internal.get("getBonusTime")?;
            let result = getBonusTime.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn nextUpdate(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let nextUpdate: crate::Function = self.internal.get("nextUpdate")?;
            let result = nextUpdate.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
    }
    pub struct BaseTask {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for BaseTask {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<BaseTask, Self::Error> {
            Ok(BaseTask {
                internal: Object::from(value),
            })
        }
    }
    impl BaseTask {}
    pub struct StudyTask {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for StudyTask {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<StudyTask, Self::Error> {
            Ok(StudyTask {
                internal: Object::from(value),
            })
        }
    }
    impl StudyTask {}
    pub struct CompanyWorkTask {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for CompanyWorkTask {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<CompanyWorkTask, Self::Error> {
            Ok(CompanyWorkTask {
                internal: Object::from(value),
            })
        }
    }
    impl CompanyWorkTask {}
    pub struct CreateProgramWorkTask {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for CreateProgramWorkTask {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<CreateProgramWorkTask, Self::Error> {
            Ok(CreateProgramWorkTask {
                internal: Object::from(value),
            })
        }
    }
    impl CreateProgramWorkTask {}
    pub struct CrimeTask {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for CrimeTask {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<CrimeTask, Self::Error> {
            Ok(CrimeTask {
                internal: Object::from(value),
            })
        }
    }
    impl CrimeTask {}
    pub struct FactionWorkTask {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for FactionWorkTask {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<FactionWorkTask, Self::Error> {
            Ok(FactionWorkTask {
                internal: Object::from(value),
            })
        }
    }
    impl FactionWorkTask {}
    pub struct GraftingTask {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for GraftingTask {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<GraftingTask, Self::Error> {
            Ok(GraftingTask {
                internal: Object::from(value),
            })
        }
    }
    impl GraftingTask {}
    struct Task {}
    pub struct BitNodeOptions {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for BitNodeOptions {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<BitNodeOptions, Self::Error> {
            Ok(BitNodeOptions {
                internal: Object::from(value),
            })
        }
    }
    impl BitNodeOptions {}
    pub struct BitNodeBooleanOptions {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for BitNodeBooleanOptions {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<BitNodeBooleanOptions, Self::Error> {
            Ok(BitNodeBooleanOptions {
                internal: Object::from(value),
            })
        }
    }
    impl BitNodeBooleanOptions {}
    pub struct Singularity {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for Singularity {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<Singularity, Self::Error> {
            Ok(Singularity {
                internal: Object::from(value),
            })
        }
    }
    impl Singularity {
        pub fn getSaveData(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getSaveData: crate::Function = self.internal.get("getSaveData")?;
            let result = getSaveData.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn exportGame(&self) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let exportGame: crate::Function = self.internal.get("exportGame")?;
            let result = exportGame.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn exportGameBonus(&self) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let exportGameBonus: crate::Function = self.internal.get("exportGameBonus")?;
            let result = exportGameBonus.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn universityCourse<
            A: Into<crate::Any>,
            B: Into<crate::Any>,
            C: Into<crate::Boolean>,
        >(
            &self,
            universityName: A,
            courseName: B,
            focus: C,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let universityCourse: crate::Function = self
                .internal
                .get("universityCourse")?;
            let result = universityCourse
                .arg(universityName.into())
                .arg(courseName.into())
                .arg(focus.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn gymWorkout<
            A: Into<crate::Any>,
            B: Into<crate::Any>,
            C: Into<crate::Boolean>,
        >(
            &self,
            gymName: A,
            stat: B,
            focus: C,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let gymWorkout: crate::Function = self.internal.get("gymWorkout")?;
            let result = gymWorkout
                .arg(gymName.into())
                .arg(stat.into())
                .arg(focus.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn travelToCity<A: Into<crate::Any>>(
            &self,
            city: A,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let travelToCity: crate::Function = self.internal.get("travelToCity")?;
            let result = travelToCity.arg(city.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn purchaseTor(&self) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let purchaseTor: crate::Function = self.internal.get("purchaseTor")?;
            let result = purchaseTor.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn purchaseProgram<A: Into<crate::String>>(
            &self,
            programName: A,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let purchaseProgram: crate::Function = self.internal.get("purchaseProgram")?;
            let result = purchaseProgram.arg(programName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn isBusy(&self) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let isBusy: crate::Function = self.internal.get("isBusy")?;
            let result = isBusy.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn stopAction(&self) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let stopAction: crate::Function = self.internal.get("stopAction")?;
            let result = stopAction.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn upgradeHomeRam(&self) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let upgradeHomeRam: crate::Function = self.internal.get("upgradeHomeRam")?;
            let result = upgradeHomeRam.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn upgradeHomeCores(&self) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let upgradeHomeCores: crate::Function = self
                .internal
                .get("upgradeHomeCores")?;
            let result = upgradeHomeCores.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getUpgradeHomeRamCost(
            &self,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getUpgradeHomeRamCost: crate::Function = self
                .internal
                .get("getUpgradeHomeRamCost")?;
            let result = getUpgradeHomeRamCost.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getUpgradeHomeCoresCost(
            &self,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getUpgradeHomeCoresCost: crate::Function = self
                .internal
                .get("getUpgradeHomeCoresCost")?;
            let result = getUpgradeHomeCoresCost.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getCompanyPositionInfo<A: Into<crate::Any>, B: Into<crate::Any>>(
            &self,
            companyName: A,
            positionName: B,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getCompanyPositionInfo: crate::Function = self
                .internal
                .get("getCompanyPositionInfo")?;
            let result = getCompanyPositionInfo
                .arg(companyName.into())
                .arg(positionName.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getCompanyPositions<A: Into<crate::Any>>(
            &self,
            companyName: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getCompanyPositions: crate::Function = self
                .internal
                .get("getCompanyPositions")?;
            let result = getCompanyPositions.arg(companyName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn workForCompany<A: Into<crate::Any>, B: Into<crate::Boolean>>(
            &self,
            companyName: A,
            focus: B,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let workForCompany: crate::Function = self.internal.get("workForCompany")?;
            let result = workForCompany
                .arg(companyName.into())
                .arg(focus.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn quitJob<A: Into<crate::Any>>(
            &self,
            companyName: A,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let quitJob: crate::Function = self.internal.get("quitJob")?;
            let result = quitJob.arg(companyName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn applyToCompany<A: Into<crate::Any>, B: Into<crate::Any>>(
            &self,
            companyName: A,
            field: B,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let applyToCompany: crate::Function = self.internal.get("applyToCompany")?;
            let result = applyToCompany
                .arg(companyName.into())
                .arg(field.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getCompanyRep<A: Into<crate::Any>>(
            &self,
            companyName: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getCompanyRep: crate::Function = self.internal.get("getCompanyRep")?;
            let result = getCompanyRep.arg(companyName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getCompanyFavor<A: Into<crate::Any>>(
            &self,
            companyName: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getCompanyFavor: crate::Function = self.internal.get("getCompanyFavor")?;
            let result = getCompanyFavor.arg(companyName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getCompanyFavorGain<A: Into<crate::Any>>(
            &self,
            companyName: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getCompanyFavorGain: crate::Function = self
                .internal
                .get("getCompanyFavorGain")?;
            let result = getCompanyFavorGain.arg(companyName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getFactionInviteRequirements<A: Into<crate::String>>(
            &self,
            faction: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getFactionInviteRequirements: crate::Function = self
                .internal
                .get("getFactionInviteRequirements")?;
            let result = getFactionInviteRequirements.arg(faction.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getFactionEnemies<A: Into<crate::String>>(
            &self,
            faction: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getFactionEnemies: crate::Function = self
                .internal
                .get("getFactionEnemies")?;
            let result = getFactionEnemies.arg(faction.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn checkFactionInvitations(
            &self,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let checkFactionInvitations: crate::Function = self
                .internal
                .get("checkFactionInvitations")?;
            let result = checkFactionInvitations.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn joinFaction<A: Into<crate::String>>(
            &self,
            faction: A,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let joinFaction: crate::Function = self.internal.get("joinFaction")?;
            let result = joinFaction.arg(faction.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn workForFaction<
            A: Into<crate::String>,
            B: Into<crate::Any>,
            C: Into<crate::Boolean>,
        >(
            &self,
            faction: A,
            workType: B,
            focus: C,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let workForFaction: crate::Function = self.internal.get("workForFaction")?;
            let result = workForFaction
                .arg(faction.into())
                .arg(workType.into())
                .arg(focus.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getFactionWorkTypes<A: Into<crate::String>>(
            &self,
            faction: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getFactionWorkTypes: crate::Function = self
                .internal
                .get("getFactionWorkTypes")?;
            let result = getFactionWorkTypes.arg(faction.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getFactionRep<A: Into<crate::String>>(
            &self,
            faction: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getFactionRep: crate::Function = self.internal.get("getFactionRep")?;
            let result = getFactionRep.arg(faction.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getFactionFavor<A: Into<crate::String>>(
            &self,
            faction: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getFactionFavor: crate::Function = self.internal.get("getFactionFavor")?;
            let result = getFactionFavor.arg(faction.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getFactionFavorGain<A: Into<crate::String>>(
            &self,
            faction: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getFactionFavorGain: crate::Function = self
                .internal
                .get("getFactionFavorGain")?;
            let result = getFactionFavorGain.arg(faction.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn donateToFaction<A: Into<crate::String>, B: Into<crate::Number>>(
            &self,
            faction: A,
            amount: B,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let donateToFaction: crate::Function = self.internal.get("donateToFaction")?;
            let result = donateToFaction.arg(faction.into()).arg(amount.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn createProgram<A: Into<crate::String>, B: Into<crate::Boolean>>(
            &self,
            program: A,
            focus: B,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let createProgram: crate::Function = self.internal.get("createProgram")?;
            let result = createProgram.arg(program.into()).arg(focus.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn commitCrime<A: Into<crate::Any>, B: Into<crate::Boolean>>(
            &self,
            crime: A,
            focus: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let commitCrime: crate::Function = self.internal.get("commitCrime")?;
            let result = commitCrime.arg(crime.into()).arg(focus.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getCrimeChance<A: Into<crate::Any>>(
            &self,
            crime: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getCrimeChance: crate::Function = self.internal.get("getCrimeChance")?;
            let result = getCrimeChance.arg(crime.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getCrimeStats<A: Into<crate::Any>>(
            &self,
            crime: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getCrimeStats: crate::Function = self.internal.get("getCrimeStats")?;
            let result = getCrimeStats.arg(crime.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getOwnedAugmentations<A: Into<crate::Boolean>>(
            &self,
            purchased: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getOwnedAugmentations: crate::Function = self
                .internal
                .get("getOwnedAugmentations")?;
            let result = getOwnedAugmentations.arg(purchased.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getOwnedSourceFiles(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getOwnedSourceFiles: crate::Function = self
                .internal
                .get("getOwnedSourceFiles")?;
            let result = getOwnedSourceFiles.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getAugmentationFactions<A: Into<crate::String>>(
            &self,
            augName: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getAugmentationFactions: crate::Function = self
                .internal
                .get("getAugmentationFactions")?;
            let result = getAugmentationFactions.arg(augName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getAugmentationsFromFaction<A: Into<crate::String>>(
            &self,
            faction: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getAugmentationsFromFaction: crate::Function = self
                .internal
                .get("getAugmentationsFromFaction")?;
            let result = getAugmentationsFromFaction.arg(faction.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getAugmentationPrereq<A: Into<crate::String>>(
            &self,
            augName: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getAugmentationPrereq: crate::Function = self
                .internal
                .get("getAugmentationPrereq")?;
            let result = getAugmentationPrereq.arg(augName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getAugmentationPrice<A: Into<crate::String>>(
            &self,
            augName: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getAugmentationPrice: crate::Function = self
                .internal
                .get("getAugmentationPrice")?;
            let result = getAugmentationPrice.arg(augName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getAugmentationBasePrice<A: Into<crate::String>>(
            &self,
            augName: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getAugmentationBasePrice: crate::Function = self
                .internal
                .get("getAugmentationBasePrice")?;
            let result = getAugmentationBasePrice.arg(augName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getAugmentationRepReq<A: Into<crate::String>>(
            &self,
            augName: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getAugmentationRepReq: crate::Function = self
                .internal
                .get("getAugmentationRepReq")?;
            let result = getAugmentationRepReq.arg(augName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn purchaseAugmentation<A: Into<crate::String>, B: Into<crate::String>>(
            &self,
            faction: A,
            augmentation: B,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let purchaseAugmentation: crate::Function = self
                .internal
                .get("purchaseAugmentation")?;
            let result = purchaseAugmentation
                .arg(faction.into())
                .arg(augmentation.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getAugmentationStats<A: Into<crate::String>>(
            &self,
            name: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getAugmentationStats: crate::Function = self
                .internal
                .get("getAugmentationStats")?;
            let result = getAugmentationStats.arg(name.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn installAugmentations<A: Into<crate::String>>(
            &self,
            cbScript: A,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let installAugmentations: crate::Function = self
                .internal
                .get("installAugmentations")?;
            let result = installAugmentations.arg(cbScript.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn hospitalize(&self) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let hospitalize: crate::Function = self.internal.get("hospitalize")?;
            let result = hospitalize.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn softReset<A: Into<crate::String>>(
            &self,
            cbScript: A,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let softReset: crate::Function = self.internal.get("softReset")?;
            let result = softReset.arg(cbScript.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn goToLocation<A: Into<crate::Any>>(
            &self,
            locationName: A,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let goToLocation: crate::Function = self.internal.get("goToLocation")?;
            let result = goToLocation.arg(locationName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getCurrentServer(&self) -> Result<crate::String, wasm_bindgen::JsValue> {
            let getCurrentServer: crate::Function = self
                .internal
                .get("getCurrentServer")?;
            let result = getCurrentServer.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn connect<A: Into<crate::String>>(
            &self,
            hostname: A,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let connect: crate::Function = self.internal.get("connect")?;
            let result = connect.arg(hostname.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn manualHack(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let manualHack: crate::Function = self.internal.get("manualHack")?;
            let result = manualHack.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn installBackdoor(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let installBackdoor: crate::Function = self.internal.get("installBackdoor")?;
            let result = installBackdoor.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn isFocused(&self) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let isFocused: crate::Function = self.internal.get("isFocused")?;
            let result = isFocused.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn setFocus<A: Into<crate::Boolean>>(
            &self,
            focus: A,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let setFocus: crate::Function = self.internal.get("setFocus")?;
            let result = setFocus.arg(focus.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getDarkwebPrograms(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getDarkwebPrograms: crate::Function = self
                .internal
                .get("getDarkwebPrograms")?;
            let result = getDarkwebPrograms.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getDarkwebProgramCost<A: Into<crate::String>>(
            &self,
            programName: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getDarkwebProgramCost: crate::Function = self
                .internal
                .get("getDarkwebProgramCost")?;
            let result = getDarkwebProgramCost.arg(programName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn b1tflum3<
            A: Into<crate::Number>,
            B: Into<crate::String>,
            C: Into<crate::Any>,
        >(
            &self,
            nextBN: A,
            callbackScript: B,
            bitNodeOptions: C,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let b1tflum3: crate::Function = self.internal.get("b1tflum3")?;
            let result = b1tflum3
                .arg(nextBN.into())
                .arg(callbackScript.into())
                .arg(bitNodeOptions.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn destroyW0r1dD43m0n<
            A: Into<crate::Number>,
            B: Into<crate::String>,
            C: Into<crate::Any>,
        >(
            &self,
            nextBN: A,
            callbackScript: B,
            bitNodeOptions: C,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let destroyW0r1dD43m0n: crate::Function = self
                .internal
                .get("destroyW0r1dD43m0n")?;
            let result = destroyW0r1dD43m0n
                .arg(nextBN.into())
                .arg(callbackScript.into())
                .arg(bitNodeOptions.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getCurrentWork(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getCurrentWork: crate::Function = self.internal.get("getCurrentWork")?;
            let result = getCurrentWork.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
    }
    pub struct CompanyPositionInfo {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for CompanyPositionInfo {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<CompanyPositionInfo, Self::Error> {
            Ok(CompanyPositionInfo {
                internal: Object::from(value),
            })
        }
    }
    impl CompanyPositionInfo {}
    pub struct Hacknet {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for Hacknet {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<Hacknet, Self::Error> {
            Ok(Hacknet {
                internal: Object::from(value),
            })
        }
    }
    impl Hacknet {
        pub fn numNodes(&self) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let numNodes: crate::Function = self.internal.get("numNodes")?;
            let result = numNodes.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn maxNumNodes(&self) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let maxNumNodes: crate::Function = self.internal.get("maxNumNodes")?;
            let result = maxNumNodes.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn purchaseNode(&self) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let purchaseNode: crate::Function = self.internal.get("purchaseNode")?;
            let result = purchaseNode.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getPurchaseNodeCost(
            &self,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getPurchaseNodeCost: crate::Function = self
                .internal
                .get("getPurchaseNodeCost")?;
            let result = getPurchaseNodeCost.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getNodeStats<A: Into<crate::Number>>(
            &self,
            index: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getNodeStats: crate::Function = self.internal.get("getNodeStats")?;
            let result = getNodeStats.arg(index.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn upgradeLevel<A: Into<crate::Number>, B: Into<crate::Number>>(
            &self,
            index: A,
            n: B,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let upgradeLevel: crate::Function = self.internal.get("upgradeLevel")?;
            let result = upgradeLevel.arg(index.into()).arg(n.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn upgradeRam<A: Into<crate::Number>, B: Into<crate::Number>>(
            &self,
            index: A,
            n: B,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let upgradeRam: crate::Function = self.internal.get("upgradeRam")?;
            let result = upgradeRam.arg(index.into()).arg(n.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn upgradeCore<A: Into<crate::Number>, B: Into<crate::Number>>(
            &self,
            index: A,
            n: B,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let upgradeCore: crate::Function = self.internal.get("upgradeCore")?;
            let result = upgradeCore.arg(index.into()).arg(n.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn upgradeCache<A: Into<crate::Number>, B: Into<crate::Number>>(
            &self,
            index: A,
            n: B,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let upgradeCache: crate::Function = self.internal.get("upgradeCache")?;
            let result = upgradeCache.arg(index.into()).arg(n.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getLevelUpgradeCost<A: Into<crate::Number>, B: Into<crate::Number>>(
            &self,
            index: A,
            n: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getLevelUpgradeCost: crate::Function = self
                .internal
                .get("getLevelUpgradeCost")?;
            let result = getLevelUpgradeCost.arg(index.into()).arg(n.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getRamUpgradeCost<A: Into<crate::Number>, B: Into<crate::Number>>(
            &self,
            index: A,
            n: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getRamUpgradeCost: crate::Function = self
                .internal
                .get("getRamUpgradeCost")?;
            let result = getRamUpgradeCost.arg(index.into()).arg(n.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getCoreUpgradeCost<A: Into<crate::Number>, B: Into<crate::Number>>(
            &self,
            index: A,
            n: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getCoreUpgradeCost: crate::Function = self
                .internal
                .get("getCoreUpgradeCost")?;
            let result = getCoreUpgradeCost.arg(index.into()).arg(n.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getCacheUpgradeCost<A: Into<crate::Number>, B: Into<crate::Number>>(
            &self,
            index: A,
            n: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getCacheUpgradeCost: crate::Function = self
                .internal
                .get("getCacheUpgradeCost")?;
            let result = getCacheUpgradeCost.arg(index.into()).arg(n.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn numHashes(&self) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let numHashes: crate::Function = self.internal.get("numHashes")?;
            let result = numHashes.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn hashCapacity(&self) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let hashCapacity: crate::Function = self.internal.get("hashCapacity")?;
            let result = hashCapacity.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn hashCost<A: Into<crate::String>, B: Into<crate::Number>>(
            &self,
            upgName: A,
            count: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let hashCost: crate::Function = self.internal.get("hashCost")?;
            let result = hashCost.arg(upgName.into()).arg(count.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn spendHashes<
            A: Into<crate::String>,
            B: Into<crate::String>,
            C: Into<crate::Number>,
        >(
            &self,
            upgName: A,
            upgTarget: B,
            count: C,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let spendHashes: crate::Function = self.internal.get("spendHashes")?;
            let result = spendHashes
                .arg(upgName.into())
                .arg(upgTarget.into())
                .arg(count.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getHashUpgrades(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getHashUpgrades: crate::Function = self.internal.get("getHashUpgrades")?;
            let result = getHashUpgrades.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getHashUpgradeLevel<A: Into<crate::String>>(
            &self,
            upgName: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getHashUpgradeLevel: crate::Function = self
                .internal
                .get("getHashUpgradeLevel")?;
            let result = getHashUpgradeLevel.arg(upgName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getStudyMult(&self) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getStudyMult: crate::Function = self.internal.get("getStudyMult")?;
            let result = getStudyMult.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getTrainingMult(&self) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getTrainingMult: crate::Function = self.internal.get("getTrainingMult")?;
            let result = getTrainingMult.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
    }
    pub enum BladeburnerActionType {
        General,
        Contract,
        Operation,
        BlackOp,
    }
    impl BladeburnerActionType {
        fn as_any(&self) -> crate::Any {
            match self {
                BladeburnerActionType::General => "General".into(),
                BladeburnerActionType::Contract => "Contracts".into(),
                BladeburnerActionType::Operation => "Operations".into(),
                BladeburnerActionType::BlackOp => "Black Operations".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    impl From<BladeburnerActionType> for crate::Any {
        fn from(value: BladeburnerActionType) -> crate::Any {
            match value {
                BladeburnerActionType::General => "General".into(),
                BladeburnerActionType::Contract => "Contracts".into(),
                BladeburnerActionType::Operation => "Operations".into(),
                BladeburnerActionType::BlackOp => "Black Operations".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    pub enum BladeburnerGeneralActionName {
        Training,
        FieldAnalysis,
        Recruitment,
        Diplomacy,
        HyperbolicRegen,
        InciteViolence,
    }
    impl BladeburnerGeneralActionName {
        fn as_any(&self) -> crate::Any {
            match self {
                BladeburnerGeneralActionName::Training => "Training".into(),
                BladeburnerGeneralActionName::FieldAnalysis => "Field Analysis".into(),
                BladeburnerGeneralActionName::Recruitment => "Recruitment".into(),
                BladeburnerGeneralActionName::Diplomacy => "Diplomacy".into(),
                BladeburnerGeneralActionName::HyperbolicRegen => {
                    "Hyperbolic Regeneration Chamber".into()
                }
                BladeburnerGeneralActionName::InciteViolence => "Incite Violence".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    impl From<BladeburnerGeneralActionName> for crate::Any {
        fn from(value: BladeburnerGeneralActionName) -> crate::Any {
            match value {
                BladeburnerGeneralActionName::Training => "Training".into(),
                BladeburnerGeneralActionName::FieldAnalysis => "Field Analysis".into(),
                BladeburnerGeneralActionName::Recruitment => "Recruitment".into(),
                BladeburnerGeneralActionName::Diplomacy => "Diplomacy".into(),
                BladeburnerGeneralActionName::HyperbolicRegen => {
                    "Hyperbolic Regeneration Chamber".into()
                }
                BladeburnerGeneralActionName::InciteViolence => "Incite Violence".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    pub enum BladeburnerContractName {
        Tracking,
        BountyHunter,
        Retirement,
    }
    impl BladeburnerContractName {
        fn as_any(&self) -> crate::Any {
            match self {
                BladeburnerContractName::Tracking => "Tracking".into(),
                BladeburnerContractName::BountyHunter => "Bounty Hunter".into(),
                BladeburnerContractName::Retirement => "Retirement".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    impl From<BladeburnerContractName> for crate::Any {
        fn from(value: BladeburnerContractName) -> crate::Any {
            match value {
                BladeburnerContractName::Tracking => "Tracking".into(),
                BladeburnerContractName::BountyHunter => "Bounty Hunter".into(),
                BladeburnerContractName::Retirement => "Retirement".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    pub enum BladeburnerOperationName {
        Investigation,
        Undercover,
        Sting,
        Raid,
        StealthRetirement,
        Assassination,
    }
    impl BladeburnerOperationName {
        fn as_any(&self) -> crate::Any {
            match self {
                BladeburnerOperationName::Investigation => "Investigation".into(),
                BladeburnerOperationName::Undercover => "Undercover Operation".into(),
                BladeburnerOperationName::Sting => "Sting Operation".into(),
                BladeburnerOperationName::Raid => "Raid".into(),
                BladeburnerOperationName::StealthRetirement => {
                    "Stealth Retirement Operation".into()
                }
                BladeburnerOperationName::Assassination => "Assassination".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    impl From<BladeburnerOperationName> for crate::Any {
        fn from(value: BladeburnerOperationName) -> crate::Any {
            match value {
                BladeburnerOperationName::Investigation => "Investigation".into(),
                BladeburnerOperationName::Undercover => "Undercover Operation".into(),
                BladeburnerOperationName::Sting => "Sting Operation".into(),
                BladeburnerOperationName::Raid => "Raid".into(),
                BladeburnerOperationName::StealthRetirement => {
                    "Stealth Retirement Operation".into()
                }
                BladeburnerOperationName::Assassination => "Assassination".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    pub enum BladeburnerBlackOpName {
        OperationTyphoon,
        OperationZero,
        OperationX,
        OperationTitan,
        OperationAres,
        OperationArchangel,
        OperationJuggernaut,
        OperationRedDragon,
        OperationK,
        OperationDeckard,
        OperationTyrell,
        OperationWallace,
        OperationShoulderOfOrion,
        OperationHyron,
        OperationMorpheus,
        OperationIonStorm,
        OperationAnnihilus,
        OperationUltron,
        OperationCenturion,
        OperationVindictus,
        OperationDaedalus,
    }
    impl BladeburnerBlackOpName {
        fn as_any(&self) -> crate::Any {
            match self {
                BladeburnerBlackOpName::OperationTyphoon => "Operation Typhoon".into(),
                BladeburnerBlackOpName::OperationZero => "Operation Zero".into(),
                BladeburnerBlackOpName::OperationX => "Operation X".into(),
                BladeburnerBlackOpName::OperationTitan => "Operation Titan".into(),
                BladeburnerBlackOpName::OperationAres => "Operation Ares".into(),
                BladeburnerBlackOpName::OperationArchangel => {
                    "Operation Archangel".into()
                }
                BladeburnerBlackOpName::OperationJuggernaut => {
                    "Operation Juggernaut".into()
                }
                BladeburnerBlackOpName::OperationRedDragon => {
                    "Operation Red Dragon".into()
                }
                BladeburnerBlackOpName::OperationK => "Operation K".into(),
                BladeburnerBlackOpName::OperationDeckard => "Operation Deckard".into(),
                BladeburnerBlackOpName::OperationTyrell => "Operation Tyrell".into(),
                BladeburnerBlackOpName::OperationWallace => "Operation Wallace".into(),
                BladeburnerBlackOpName::OperationShoulderOfOrion => {
                    "Operation Shoulder of Orion".into()
                }
                BladeburnerBlackOpName::OperationHyron => "Operation Hyron".into(),
                BladeburnerBlackOpName::OperationMorpheus => "Operation Morpheus".into(),
                BladeburnerBlackOpName::OperationIonStorm => "Operation Ion Storm".into(),
                BladeburnerBlackOpName::OperationAnnihilus => {
                    "Operation Annihilus".into()
                }
                BladeburnerBlackOpName::OperationUltron => "Operation Ultron".into(),
                BladeburnerBlackOpName::OperationCenturion => {
                    "Operation Centurion".into()
                }
                BladeburnerBlackOpName::OperationVindictus => {
                    "Operation Vindictus".into()
                }
                BladeburnerBlackOpName::OperationDaedalus => "Operation Daedalus".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    impl From<BladeburnerBlackOpName> for crate::Any {
        fn from(value: BladeburnerBlackOpName) -> crate::Any {
            match value {
                BladeburnerBlackOpName::OperationTyphoon => "Operation Typhoon".into(),
                BladeburnerBlackOpName::OperationZero => "Operation Zero".into(),
                BladeburnerBlackOpName::OperationX => "Operation X".into(),
                BladeburnerBlackOpName::OperationTitan => "Operation Titan".into(),
                BladeburnerBlackOpName::OperationAres => "Operation Ares".into(),
                BladeburnerBlackOpName::OperationArchangel => {
                    "Operation Archangel".into()
                }
                BladeburnerBlackOpName::OperationJuggernaut => {
                    "Operation Juggernaut".into()
                }
                BladeburnerBlackOpName::OperationRedDragon => {
                    "Operation Red Dragon".into()
                }
                BladeburnerBlackOpName::OperationK => "Operation K".into(),
                BladeburnerBlackOpName::OperationDeckard => "Operation Deckard".into(),
                BladeburnerBlackOpName::OperationTyrell => "Operation Tyrell".into(),
                BladeburnerBlackOpName::OperationWallace => "Operation Wallace".into(),
                BladeburnerBlackOpName::OperationShoulderOfOrion => {
                    "Operation Shoulder of Orion".into()
                }
                BladeburnerBlackOpName::OperationHyron => "Operation Hyron".into(),
                BladeburnerBlackOpName::OperationMorpheus => "Operation Morpheus".into(),
                BladeburnerBlackOpName::OperationIonStorm => "Operation Ion Storm".into(),
                BladeburnerBlackOpName::OperationAnnihilus => {
                    "Operation Annihilus".into()
                }
                BladeburnerBlackOpName::OperationUltron => "Operation Ultron".into(),
                BladeburnerBlackOpName::OperationCenturion => {
                    "Operation Centurion".into()
                }
                BladeburnerBlackOpName::OperationVindictus => {
                    "Operation Vindictus".into()
                }
                BladeburnerBlackOpName::OperationDaedalus => "Operation Daedalus".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    pub enum BladeburnerSkillName {
        BladesIntuition,
        Cloak,
        ShortCircuit,
        DigitalObserver,
        Tracer,
        Overclock,
        Reaper,
        EvasiveSystem,
        Datamancer,
        CybersEdge,
        HandsOfMidas,
        Hyperdrive,
    }
    impl BladeburnerSkillName {
        fn as_any(&self) -> crate::Any {
            match self {
                BladeburnerSkillName::BladesIntuition => "Blade's Intuition".into(),
                BladeburnerSkillName::Cloak => "Cloak".into(),
                BladeburnerSkillName::ShortCircuit => "Short-Circuit".into(),
                BladeburnerSkillName::DigitalObserver => "Digital Observer".into(),
                BladeburnerSkillName::Tracer => "Tracer".into(),
                BladeburnerSkillName::Overclock => "Overclock".into(),
                BladeburnerSkillName::Reaper => "Reaper".into(),
                BladeburnerSkillName::EvasiveSystem => "Evasive System".into(),
                BladeburnerSkillName::Datamancer => "Datamancer".into(),
                BladeburnerSkillName::CybersEdge => "Cyber's Edge".into(),
                BladeburnerSkillName::HandsOfMidas => "Hands of Midas".into(),
                BladeburnerSkillName::Hyperdrive => "Hyperdrive".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    impl From<BladeburnerSkillName> for crate::Any {
        fn from(value: BladeburnerSkillName) -> crate::Any {
            match value {
                BladeburnerSkillName::BladesIntuition => "Blade's Intuition".into(),
                BladeburnerSkillName::Cloak => "Cloak".into(),
                BladeburnerSkillName::ShortCircuit => "Short-Circuit".into(),
                BladeburnerSkillName::DigitalObserver => "Digital Observer".into(),
                BladeburnerSkillName::Tracer => "Tracer".into(),
                BladeburnerSkillName::Overclock => "Overclock".into(),
                BladeburnerSkillName::Reaper => "Reaper".into(),
                BladeburnerSkillName::EvasiveSystem => "Evasive System".into(),
                BladeburnerSkillName::Datamancer => "Datamancer".into(),
                BladeburnerSkillName::CybersEdge => "Cyber's Edge".into(),
                BladeburnerSkillName::HandsOfMidas => "Hands of Midas".into(),
                BladeburnerSkillName::Hyperdrive => "Hyperdrive".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    struct BladeburnerActionName {}
    pub enum SpecialBladeburnerActionTypeForSleeve {
        InfiltrateSynthoids,
        SupportMainSleeve,
        TakeOnContracts,
    }
    impl SpecialBladeburnerActionTypeForSleeve {
        fn as_any(&self) -> crate::Any {
            match self {
                SpecialBladeburnerActionTypeForSleeve::InfiltrateSynthoids => {
                    "Infiltrate Synthoids".into()
                }
                SpecialBladeburnerActionTypeForSleeve::SupportMainSleeve => {
                    "Support main sleeve".into()
                }
                SpecialBladeburnerActionTypeForSleeve::TakeOnContracts => {
                    "Take on contracts".into()
                }
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    impl From<SpecialBladeburnerActionTypeForSleeve> for crate::Any {
        fn from(value: SpecialBladeburnerActionTypeForSleeve) -> crate::Any {
            match value {
                SpecialBladeburnerActionTypeForSleeve::InfiltrateSynthoids => {
                    "Infiltrate Synthoids".into()
                }
                SpecialBladeburnerActionTypeForSleeve::SupportMainSleeve => {
                    "Support main sleeve".into()
                }
                SpecialBladeburnerActionTypeForSleeve::TakeOnContracts => {
                    "Take on contracts".into()
                }
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    struct BladeburnerActionTypeForSleeve {}
    pub struct Bladeburner {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for Bladeburner {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<Bladeburner, Self::Error> {
            Ok(Bladeburner {
                internal: Object::from(value),
            })
        }
    }
    impl Bladeburner {
        pub fn getContractNames(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getContractNames: crate::Function = self
                .internal
                .get("getContractNames")?;
            let result = getContractNames.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getOperationNames(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getOperationNames: crate::Function = self
                .internal
                .get("getOperationNames")?;
            let result = getOperationNames.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getBlackOpNames(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getBlackOpNames: crate::Function = self.internal.get("getBlackOpNames")?;
            let result = getBlackOpNames.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getNextBlackOp(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getNextBlackOp: crate::Function = self.internal.get("getNextBlackOp")?;
            let result = getNextBlackOp.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getGeneralActionNames(
            &self,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getGeneralActionNames: crate::Function = self
                .internal
                .get("getGeneralActionNames")?;
            let result = getGeneralActionNames.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getSkillNames(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getSkillNames: crate::Function = self.internal.get("getSkillNames")?;
            let result = getSkillNames.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn startAction<A: Into<crate::Any>, B: Into<crate::Any>>(
            &self,
            _type: A,
            name: B,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let startAction: crate::Function = self.internal.get("startAction")?;
            let result = startAction.arg(_type.into()).arg(name.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn stopBladeburnerAction(
            &self,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let stopBladeburnerAction: crate::Function = self
                .internal
                .get("stopBladeburnerAction")?;
            let result = stopBladeburnerAction.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getCurrentAction(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getCurrentAction: crate::Function = self
                .internal
                .get("getCurrentAction")?;
            let result = getCurrentAction.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getActionTime<A: Into<crate::Any>, B: Into<crate::Any>>(
            &self,
            _type: A,
            name: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getActionTime: crate::Function = self.internal.get("getActionTime")?;
            let result = getActionTime.arg(_type.into()).arg(name.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getActionCurrentTime(
            &self,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getActionCurrentTime: crate::Function = self
                .internal
                .get("getActionCurrentTime")?;
            let result = getActionCurrentTime.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getActionEstimatedSuccessChance<
            A: Into<crate::Any>,
            B: Into<crate::Any>,
            C: Into<crate::Number>,
        >(
            &self,
            _type: A,
            name: B,
            sleeveNumber: C,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getActionEstimatedSuccessChance: crate::Function = self
                .internal
                .get("getActionEstimatedSuccessChance")?;
            let result = getActionEstimatedSuccessChance
                .arg(_type.into())
                .arg(name.into())
                .arg(sleeveNumber.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getActionRepGain<
            A: Into<crate::Any>,
            B: Into<crate::Any>,
            C: Into<crate::Number>,
        >(
            &self,
            _type: A,
            name: B,
            level: C,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getActionRepGain: crate::Function = self
                .internal
                .get("getActionRepGain")?;
            let result = getActionRepGain
                .arg(_type.into())
                .arg(name.into())
                .arg(level.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getActionCountRemaining<A: Into<crate::Any>, B: Into<crate::Any>>(
            &self,
            _type: A,
            name: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getActionCountRemaining: crate::Function = self
                .internal
                .get("getActionCountRemaining")?;
            let result = getActionCountRemaining
                .arg(_type.into())
                .arg(name.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getActionMaxLevel<A: Into<crate::Any>, B: Into<crate::Any>>(
            &self,
            _type: A,
            name: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getActionMaxLevel: crate::Function = self
                .internal
                .get("getActionMaxLevel")?;
            let result = getActionMaxLevel.arg(_type.into()).arg(name.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getActionCurrentLevel<A: Into<crate::Any>, B: Into<crate::Any>>(
            &self,
            _type: A,
            name: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getActionCurrentLevel: crate::Function = self
                .internal
                .get("getActionCurrentLevel")?;
            let result = getActionCurrentLevel
                .arg(_type.into())
                .arg(name.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getActionAutolevel<A: Into<crate::Any>, B: Into<crate::Any>>(
            &self,
            _type: A,
            name: B,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let getActionAutolevel: crate::Function = self
                .internal
                .get("getActionAutolevel")?;
            let result = getActionAutolevel.arg(_type.into()).arg(name.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getActionSuccesses<A: Into<crate::Any>, B: Into<crate::Any>>(
            &self,
            _type: A,
            name: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getActionSuccesses: crate::Function = self
                .internal
                .get("getActionSuccesses")?;
            let result = getActionSuccesses.arg(_type.into()).arg(name.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn setActionAutolevel<
            A: Into<crate::Any>,
            B: Into<crate::Any>,
            C: Into<crate::Boolean>,
        >(
            &self,
            _type: A,
            name: B,
            autoLevel: C,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let setActionAutolevel: crate::Function = self
                .internal
                .get("setActionAutolevel")?;
            let result = setActionAutolevel
                .arg(_type.into())
                .arg(name.into())
                .arg(autoLevel.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn setActionLevel<
            A: Into<crate::Any>,
            B: Into<crate::Any>,
            C: Into<crate::Number>,
        >(
            &self,
            _type: A,
            name: B,
            level: C,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let setActionLevel: crate::Function = self.internal.get("setActionLevel")?;
            let result = setActionLevel
                .arg(_type.into())
                .arg(name.into())
                .arg(level.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getRank(&self) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getRank: crate::Function = self.internal.get("getRank")?;
            let result = getRank.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getBlackOpRank<A: Into<crate::Any>>(
            &self,
            name: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getBlackOpRank: crate::Function = self.internal.get("getBlackOpRank")?;
            let result = getBlackOpRank.arg(name.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getSkillPoints(&self) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getSkillPoints: crate::Function = self.internal.get("getSkillPoints")?;
            let result = getSkillPoints.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getSkillLevel<A: Into<crate::Any>>(
            &self,
            skillName: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getSkillLevel: crate::Function = self.internal.get("getSkillLevel")?;
            let result = getSkillLevel.arg(skillName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getSkillUpgradeCost<A: Into<crate::Any>, B: Into<crate::Number>>(
            &self,
            skillName: A,
            count: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getSkillUpgradeCost: crate::Function = self
                .internal
                .get("getSkillUpgradeCost")?;
            let result = getSkillUpgradeCost
                .arg(skillName.into())
                .arg(count.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn upgradeSkill<A: Into<crate::Any>, B: Into<crate::Number>>(
            &self,
            skillName: A,
            count: B,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let upgradeSkill: crate::Function = self.internal.get("upgradeSkill")?;
            let result = upgradeSkill.arg(skillName.into()).arg(count.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getTeamSize<A: Into<crate::Any>, B: Into<crate::Any>>(
            &self,
            _type: A,
            name: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getTeamSize: crate::Function = self.internal.get("getTeamSize")?;
            let result = getTeamSize.arg(_type.into()).arg(name.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn setTeamSize<
            A: Into<crate::Any>,
            B: Into<crate::Any>,
            C: Into<crate::Number>,
        >(
            &self,
            _type: A,
            name: B,
            size: C,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let setTeamSize: crate::Function = self.internal.get("setTeamSize")?;
            let result = setTeamSize
                .arg(_type.into())
                .arg(name.into())
                .arg(size.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getCityEstimatedPopulation<A: Into<crate::Any>>(
            &self,
            city: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getCityEstimatedPopulation: crate::Function = self
                .internal
                .get("getCityEstimatedPopulation")?;
            let result = getCityEstimatedPopulation.arg(city.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getCityCommunities<A: Into<crate::Any>>(
            &self,
            city: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getCityCommunities: crate::Function = self
                .internal
                .get("getCityCommunities")?;
            let result = getCityCommunities.arg(city.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getCityChaos<A: Into<crate::Any>>(
            &self,
            city: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getCityChaos: crate::Function = self.internal.get("getCityChaos")?;
            let result = getCityChaos.arg(city.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getCity(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getCity: crate::Function = self.internal.get("getCity")?;
            let result = getCity.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn switchCity<A: Into<crate::Any>>(
            &self,
            city: A,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let switchCity: crate::Function = self.internal.get("switchCity")?;
            let result = switchCity.arg(city.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getStamina(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getStamina: crate::Function = self.internal.get("getStamina")?;
            let result = getStamina.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn joinBladeburnerFaction(
            &self,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let joinBladeburnerFaction: crate::Function = self
                .internal
                .get("joinBladeburnerFaction")?;
            let result = joinBladeburnerFaction.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn joinBladeburnerDivision(
            &self,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let joinBladeburnerDivision: crate::Function = self
                .internal
                .get("joinBladeburnerDivision")?;
            let result = joinBladeburnerDivision.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getBonusTime(&self) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getBonusTime: crate::Function = self.internal.get("getBonusTime")?;
            let result = getBonusTime.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn nextUpdate(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let nextUpdate: crate::Function = self.internal.get("nextUpdate")?;
            let result = nextUpdate.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn inBladeburner(&self) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let inBladeburner: crate::Function = self.internal.get("inBladeburner")?;
            let result = inBladeburner.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
    }
    pub struct CodingContract {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for CodingContract {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<CodingContract, Self::Error> {
            Ok(CodingContract {
                internal: Object::from(value),
            })
        }
    }
    impl CodingContract {
        pub fn attempt<
            A: Into<crate::Any>,
            B: Into<crate::String>,
            C: Into<crate::String>,
        >(
            &self,
            answer: A,
            filename: B,
            host: C,
        ) -> Result<crate::String, wasm_bindgen::JsValue> {
            let attempt: crate::Function = self.internal.get("attempt")?;
            let result = attempt
                .arg(answer.into())
                .arg(filename.into())
                .arg(host.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getContractType<A: Into<crate::String>, B: Into<crate::String>>(
            &self,
            filename: A,
            host: B,
        ) -> Result<crate::String, wasm_bindgen::JsValue> {
            let getContractType: crate::Function = self.internal.get("getContractType")?;
            let result = getContractType.arg(filename.into()).arg(host.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getDescription<A: Into<crate::String>, B: Into<crate::String>>(
            &self,
            filename: A,
            host: B,
        ) -> Result<crate::String, wasm_bindgen::JsValue> {
            let getDescription: crate::Function = self.internal.get("getDescription")?;
            let result = getDescription.arg(filename.into()).arg(host.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getData<A: Into<crate::String>, B: Into<crate::String>>(
            &self,
            filename: A,
            host: B,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getData: crate::Function = self.internal.get("getData")?;
            let result = getData.arg(filename.into()).arg(host.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getContract<A: Into<crate::String>, B: Into<crate::String>>(
            &self,
            filename: A,
            host: B,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getContract: crate::Function = self.internal.get("getContract")?;
            let result = getContract.arg(filename.into()).arg(host.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getNumTriesRemaining<A: Into<crate::String>, B: Into<crate::String>>(
            &self,
            filename: A,
            host: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getNumTriesRemaining: crate::Function = self
                .internal
                .get("getNumTriesRemaining")?;
            let result = getNumTriesRemaining
                .arg(filename.into())
                .arg(host.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn createDummyContract<A: Into<crate::String>>(
            &self,
            _type: A,
        ) -> Result<crate::String, wasm_bindgen::JsValue> {
            let createDummyContract: crate::Function = self
                .internal
                .get("createDummyContract")?;
            let result = createDummyContract.arg(_type.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getContractTypes(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getContractTypes: crate::Function = self
                .internal
                .get("getContractTypes")?;
            let result = getContractTypes.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
    }
    pub struct Gang {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for Gang {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<Gang, Self::Error> {
            Ok(Gang {
                internal: Object::from(value),
            })
        }
    }
    impl Gang {
        pub fn createGang<A: Into<crate::String>>(
            &self,
            faction: A,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let createGang: crate::Function = self.internal.get("createGang")?;
            let result = createGang.arg(faction.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn inGang(&self) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let inGang: crate::Function = self.internal.get("inGang")?;
            let result = inGang.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getMemberNames(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getMemberNames: crate::Function = self.internal.get("getMemberNames")?;
            let result = getMemberNames.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn renameMember<A: Into<crate::String>, B: Into<crate::String>>(
            &self,
            memberName: A,
            newName: B,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let renameMember: crate::Function = self.internal.get("renameMember")?;
            let result = renameMember.arg(memberName.into()).arg(newName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getGangInformation(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getGangInformation: crate::Function = self
                .internal
                .get("getGangInformation")?;
            let result = getGangInformation.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getOtherGangInformation(
            &self,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getOtherGangInformation: crate::Function = self
                .internal
                .get("getOtherGangInformation")?;
            let result = getOtherGangInformation.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getMemberInformation<A: Into<crate::String>>(
            &self,
            name: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getMemberInformation: crate::Function = self
                .internal
                .get("getMemberInformation")?;
            let result = getMemberInformation.arg(name.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn canRecruitMember(&self) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let canRecruitMember: crate::Function = self
                .internal
                .get("canRecruitMember")?;
            let result = canRecruitMember.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getRecruitsAvailable(
            &self,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getRecruitsAvailable: crate::Function = self
                .internal
                .get("getRecruitsAvailable")?;
            let result = getRecruitsAvailable.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn respectForNextRecruit(
            &self,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let respectForNextRecruit: crate::Function = self
                .internal
                .get("respectForNextRecruit")?;
            let result = respectForNextRecruit.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn recruitMember<A: Into<crate::String>>(
            &self,
            name: A,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let recruitMember: crate::Function = self.internal.get("recruitMember")?;
            let result = recruitMember.arg(name.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getTaskNames(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getTaskNames: crate::Function = self.internal.get("getTaskNames")?;
            let result = getTaskNames.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn setMemberTask<A: Into<crate::String>, B: Into<crate::String>>(
            &self,
            memberName: A,
            taskName: B,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let setMemberTask: crate::Function = self.internal.get("setMemberTask")?;
            let result = setMemberTask
                .arg(memberName.into())
                .arg(taskName.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getTaskStats<A: Into<crate::String>>(
            &self,
            name: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getTaskStats: crate::Function = self.internal.get("getTaskStats")?;
            let result = getTaskStats.arg(name.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getEquipmentNames(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getEquipmentNames: crate::Function = self
                .internal
                .get("getEquipmentNames")?;
            let result = getEquipmentNames.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getEquipmentCost<A: Into<crate::String>>(
            &self,
            equipName: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getEquipmentCost: crate::Function = self
                .internal
                .get("getEquipmentCost")?;
            let result = getEquipmentCost.arg(equipName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getEquipmentType<A: Into<crate::String>>(
            &self,
            equipName: A,
        ) -> Result<crate::String, wasm_bindgen::JsValue> {
            let getEquipmentType: crate::Function = self
                .internal
                .get("getEquipmentType")?;
            let result = getEquipmentType.arg(equipName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getEquipmentStats<A: Into<crate::String>>(
            &self,
            equipName: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getEquipmentStats: crate::Function = self
                .internal
                .get("getEquipmentStats")?;
            let result = getEquipmentStats.arg(equipName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn purchaseEquipment<A: Into<crate::String>, B: Into<crate::String>>(
            &self,
            memberName: A,
            equipName: B,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let purchaseEquipment: crate::Function = self
                .internal
                .get("purchaseEquipment")?;
            let result = purchaseEquipment
                .arg(memberName.into())
                .arg(equipName.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn ascendMember<A: Into<crate::String>>(
            &self,
            memberName: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let ascendMember: crate::Function = self.internal.get("ascendMember")?;
            let result = ascendMember.arg(memberName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getAscensionResult<A: Into<crate::String>>(
            &self,
            memberName: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getAscensionResult: crate::Function = self
                .internal
                .get("getAscensionResult")?;
            let result = getAscensionResult.arg(memberName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getInstallResult<A: Into<crate::String>>(
            &self,
            memberName: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getInstallResult: crate::Function = self
                .internal
                .get("getInstallResult")?;
            let result = getInstallResult.arg(memberName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn setTerritoryWarfare<A: Into<crate::Boolean>>(
            &self,
            engage: A,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let setTerritoryWarfare: crate::Function = self
                .internal
                .get("setTerritoryWarfare")?;
            let result = setTerritoryWarfare.arg(engage.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getChanceToWinClash<A: Into<crate::String>>(
            &self,
            gangName: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getChanceToWinClash: crate::Function = self
                .internal
                .get("getChanceToWinClash")?;
            let result = getChanceToWinClash.arg(gangName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getBonusTime(&self) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getBonusTime: crate::Function = self.internal.get("getBonusTime")?;
            let result = getBonusTime.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn nextUpdate(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let nextUpdate: crate::Function = self.internal.get("nextUpdate")?;
            let result = nextUpdate.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
    }
    struct GoOpponent {}
    struct SimpleOpponentStats {}
    pub struct GoAnalysis {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for GoAnalysis {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<GoAnalysis, Self::Error> {
            Ok(GoAnalysis {
                internal: Object::from(value),
            })
        }
    }
    impl GoAnalysis {
        pub fn getValidMoves<
            A: Into<crate::Any>,
            B: Into<crate::Any>,
            C: Into<crate::Boolean>,
        >(
            &self,
            boardState: A,
            priorBoardState: B,
            playAsWhite: C,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getValidMoves: crate::Function = self.internal.get("getValidMoves")?;
            let result = getValidMoves
                .arg(boardState.into())
                .arg(priorBoardState.into())
                .arg(playAsWhite.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getChains<A: Into<crate::Any>>(
            &self,
            boardState: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getChains: crate::Function = self.internal.get("getChains")?;
            let result = getChains.arg(boardState.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getLiberties<A: Into<crate::Any>>(
            &self,
            boardState: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getLiberties: crate::Function = self.internal.get("getLiberties")?;
            let result = getLiberties.arg(boardState.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getControlledEmptyNodes<A: Into<crate::Any>>(
            &self,
            boardState: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getControlledEmptyNodes: crate::Function = self
                .internal
                .get("getControlledEmptyNodes")?;
            let result = getControlledEmptyNodes.arg(boardState.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getStats(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getStats: crate::Function = self.internal.get("getStats")?;
            let result = getStats.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn resetStats<A: Into<crate::Boolean>>(
            &self,
            resetAll: A,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let resetStats: crate::Function = self.internal.get("resetStats")?;
            let result = resetStats.arg(resetAll.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
    }
    pub struct GoCheat {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for GoCheat {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<GoCheat, Self::Error> {
            Ok(GoCheat {
                internal: Object::from(value),
            })
        }
    }
    impl GoCheat {
        pub fn getCheatSuccessChance<A: Into<crate::Number>, B: Into<crate::Boolean>>(
            &self,
            cheatCount: A,
            playAsWhite: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getCheatSuccessChance: crate::Function = self
                .internal
                .get("getCheatSuccessChance")?;
            let result = getCheatSuccessChance
                .arg(cheatCount.into())
                .arg(playAsWhite.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getCheatCount<A: Into<crate::Boolean>>(
            &self,
            playAsWhite: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getCheatCount: crate::Function = self.internal.get("getCheatCount")?;
            let result = getCheatCount.arg(playAsWhite.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn removeRouter<
            A: Into<crate::Number>,
            B: Into<crate::Number>,
            C: Into<crate::Boolean>,
        >(
            &self,
            x: A,
            y: B,
            playAsWhite: C,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let removeRouter: crate::Function = self.internal.get("removeRouter")?;
            let result = removeRouter
                .arg(x.into())
                .arg(y.into())
                .arg(playAsWhite.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn playTwoMoves<
            A: Into<crate::Number>,
            B: Into<crate::Number>,
            C: Into<crate::Number>,
            D: Into<crate::Number>,
            E: Into<crate::Boolean>,
        >(
            &self,
            x1: A,
            y1: B,
            x2: C,
            y2: D,
            playAsWhite: E,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let playTwoMoves: crate::Function = self.internal.get("playTwoMoves")?;
            let result = playTwoMoves
                .arg(x1.into())
                .arg(y1.into())
                .arg(x2.into())
                .arg(y2.into())
                .arg(playAsWhite.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn repairOfflineNode<
            A: Into<crate::Number>,
            B: Into<crate::Number>,
            C: Into<crate::Boolean>,
        >(
            &self,
            x: A,
            y: B,
            playAsWhite: C,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let repairOfflineNode: crate::Function = self
                .internal
                .get("repairOfflineNode")?;
            let result = repairOfflineNode
                .arg(x.into())
                .arg(y.into())
                .arg(playAsWhite.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn destroyNode<
            A: Into<crate::Number>,
            B: Into<crate::Number>,
            C: Into<crate::Boolean>,
        >(
            &self,
            x: A,
            y: B,
            playAsWhite: C,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let destroyNode: crate::Function = self.internal.get("destroyNode")?;
            let result = destroyNode
                .arg(x.into())
                .arg(y.into())
                .arg(playAsWhite.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
    }
    pub struct Go {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for Go {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<Go, Self::Error> {
            Ok(Go {
                internal: Object::from(value),
            })
        }
    }
    impl Go {
        pub fn makeMove<
            A: Into<crate::Number>,
            B: Into<crate::Number>,
            C: Into<crate::Boolean>,
        >(
            &self,
            x: A,
            y: B,
            playAsWhite: C,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let makeMove: crate::Function = self.internal.get("makeMove")?;
            let result = makeMove
                .arg(x.into())
                .arg(y.into())
                .arg(playAsWhite.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn passTurn<A: Into<crate::Boolean>>(
            &self,
            passAsWhite: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let passTurn: crate::Function = self.internal.get("passTurn")?;
            let result = passTurn.arg(passAsWhite.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn opponentNextTurn<A: Into<crate::Boolean>, B: Into<crate::Boolean>>(
            &self,
            logOpponentMove: A,
            playAsWhite: B,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let opponentNextTurn: crate::Function = self
                .internal
                .get("opponentNextTurn")?;
            let result = opponentNextTurn
                .arg(logOpponentMove.into())
                .arg(playAsWhite.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getBoardState(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getBoardState: crate::Function = self.internal.get("getBoardState")?;
            let result = getBoardState.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getMoveHistory(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getMoveHistory: crate::Function = self.internal.get("getMoveHistory")?;
            let result = getMoveHistory.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getCurrentPlayer(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getCurrentPlayer: crate::Function = self
                .internal
                .get("getCurrentPlayer")?;
            let result = getCurrentPlayer.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getGameState(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getGameState: crate::Function = self.internal.get("getGameState")?;
            let result = getGameState.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getOpponent(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getOpponent: crate::Function = self.internal.get("getOpponent")?;
            let result = getOpponent.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn resetBoardState<A: Into<crate::Any>, B: Into<crate::Any>>(
            &self,
            opponent: A,
            boardSize: B,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let resetBoardState: crate::Function = self.internal.get("resetBoardState")?;
            let result = resetBoardState
                .arg(opponent.into())
                .arg(boardSize.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
    }
    pub struct Sleeve {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for Sleeve {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<Sleeve, Self::Error> {
            Ok(Sleeve {
                internal: Object::from(value),
            })
        }
    }
    impl Sleeve {
        pub fn getNumSleeves(&self) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getNumSleeves: crate::Function = self.internal.get("getNumSleeves")?;
            let result = getNumSleeves.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getSleeve<A: Into<crate::Number>>(
            &self,
            sleeveNumber: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getSleeve: crate::Function = self.internal.get("getSleeve")?;
            let result = getSleeve.arg(sleeveNumber.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getTask<A: Into<crate::Number>>(
            &self,
            sleeveNumber: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getTask: crate::Function = self.internal.get("getTask")?;
            let result = getTask.arg(sleeveNumber.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn setToIdle<A: Into<crate::Number>>(
            &self,
            sleeveNumber: A,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let setToIdle: crate::Function = self.internal.get("setToIdle")?;
            let result = setToIdle.arg(sleeveNumber.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn setToShockRecovery<A: Into<crate::Number>>(
            &self,
            sleeveNumber: A,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let setToShockRecovery: crate::Function = self
                .internal
                .get("setToShockRecovery")?;
            let result = setToShockRecovery.arg(sleeveNumber.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn setToSynchronize<A: Into<crate::Number>>(
            &self,
            sleeveNumber: A,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let setToSynchronize: crate::Function = self
                .internal
                .get("setToSynchronize")?;
            let result = setToSynchronize.arg(sleeveNumber.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn setToCommitCrime<A: Into<crate::Number>, B: Into<crate::Any>>(
            &self,
            sleeveNumber: A,
            crimeType: B,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let setToCommitCrime: crate::Function = self
                .internal
                .get("setToCommitCrime")?;
            let result = setToCommitCrime
                .arg(sleeveNumber.into())
                .arg(crimeType.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn setToFactionWork<
            A: Into<crate::Number>,
            B: Into<crate::String>,
            C: Into<crate::Any>,
        >(
            &self,
            sleeveNumber: A,
            factionName: B,
            factionWorkType: C,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let setToFactionWork: crate::Function = self
                .internal
                .get("setToFactionWork")?;
            let result = setToFactionWork
                .arg(sleeveNumber.into())
                .arg(factionName.into())
                .arg(factionWorkType.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn setToCompanyWork<A: Into<crate::Number>, B: Into<crate::Any>>(
            &self,
            sleeveNumber: A,
            companyName: B,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let setToCompanyWork: crate::Function = self
                .internal
                .get("setToCompanyWork")?;
            let result = setToCompanyWork
                .arg(sleeveNumber.into())
                .arg(companyName.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn setToUniversityCourse<
            A: Into<crate::Number>,
            B: Into<crate::Any>,
            C: Into<crate::Any>,
        >(
            &self,
            sleeveNumber: A,
            universityName: B,
            courseName: C,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let setToUniversityCourse: crate::Function = self
                .internal
                .get("setToUniversityCourse")?;
            let result = setToUniversityCourse
                .arg(sleeveNumber.into())
                .arg(universityName.into())
                .arg(courseName.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn setToGymWorkout<
            A: Into<crate::Number>,
            B: Into<crate::Any>,
            C: Into<crate::Any>,
        >(
            &self,
            sleeveNumber: A,
            gymName: B,
            stat: C,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let setToGymWorkout: crate::Function = self.internal.get("setToGymWorkout")?;
            let result = setToGymWorkout
                .arg(sleeveNumber.into())
                .arg(gymName.into())
                .arg(stat.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn travel<A: Into<crate::Number>, B: Into<crate::Any>>(
            &self,
            sleeveNumber: A,
            city: B,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let travel: crate::Function = self.internal.get("travel")?;
            let result = travel.arg(sleeveNumber.into()).arg(city.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getSleeveAugmentations<A: Into<crate::Number>>(
            &self,
            sleeveNumber: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getSleeveAugmentations: crate::Function = self
                .internal
                .get("getSleeveAugmentations")?;
            let result = getSleeveAugmentations.arg(sleeveNumber.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getSleeveAugmentationPrice<A: Into<crate::String>>(
            &self,
            augName: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getSleeveAugmentationPrice: crate::Function = self
                .internal
                .get("getSleeveAugmentationPrice")?;
            let result = getSleeveAugmentationPrice.arg(augName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getSleeveAugmentationRepReq<A: Into<crate::String>>(
            &self,
            augName: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getSleeveAugmentationRepReq: crate::Function = self
                .internal
                .get("getSleeveAugmentationRepReq")?;
            let result = getSleeveAugmentationRepReq.arg(augName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getSleevePurchasableAugs<A: Into<crate::Number>>(
            &self,
            sleeveNumber: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getSleevePurchasableAugs: crate::Function = self
                .internal
                .get("getSleevePurchasableAugs")?;
            let result = getSleevePurchasableAugs.arg(sleeveNumber.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn purchaseSleeveAug<A: Into<crate::Number>, B: Into<crate::String>>(
            &self,
            sleeveNumber: A,
            augName: B,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let purchaseSleeveAug: crate::Function = self
                .internal
                .get("purchaseSleeveAug")?;
            let result = purchaseSleeveAug
                .arg(sleeveNumber.into())
                .arg(augName.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn setToBladeburnerAction<
            A: Into<crate::Number>,
            B: Into<crate::Any>,
            C: Into<crate::Any>,
        >(
            &self,
            sleeveNumber: A,
            action: B,
            contract: C,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let setToBladeburnerAction: crate::Function = self
                .internal
                .get("setToBladeburnerAction")?;
            let result = setToBladeburnerAction
                .arg(sleeveNumber.into())
                .arg(action.into())
                .arg(contract.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
    }
    pub struct Grafting {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for Grafting {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<Grafting, Self::Error> {
            Ok(Grafting {
                internal: Object::from(value),
            })
        }
    }
    impl Grafting {
        pub fn getAugmentationGraftPrice<A: Into<crate::String>>(
            &self,
            augName: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getAugmentationGraftPrice: crate::Function = self
                .internal
                .get("getAugmentationGraftPrice")?;
            let result = getAugmentationGraftPrice.arg(augName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getAugmentationGraftTime<A: Into<crate::String>>(
            &self,
            augName: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getAugmentationGraftTime: crate::Function = self
                .internal
                .get("getAugmentationGraftTime")?;
            let result = getAugmentationGraftTime.arg(augName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getGraftableAugmentations(
            &self,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getGraftableAugmentations: crate::Function = self
                .internal
                .get("getGraftableAugmentations")?;
            let result = getGraftableAugmentations.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn graftAugmentation<A: Into<crate::String>, B: Into<crate::Boolean>>(
            &self,
            augName: A,
            focus: B,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let graftAugmentation: crate::Function = self
                .internal
                .get("graftAugmentation")?;
            let result = graftAugmentation.arg(augName.into()).arg(focus.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn waitForOngoingGrafting(
            &self,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let waitForOngoingGrafting: crate::Function = self
                .internal
                .get("waitForOngoingGrafting")?;
            let result = waitForOngoingGrafting.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
    }
    pub struct SkillsFormulas {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for SkillsFormulas {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<SkillsFormulas, Self::Error> {
            Ok(SkillsFormulas {
                internal: Object::from(value),
            })
        }
    }
    impl SkillsFormulas {
        pub fn calculateSkill<A: Into<crate::Number>, B: Into<crate::Number>>(
            &self,
            exp: A,
            skillMult: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let calculateSkill: crate::Function = self.internal.get("calculateSkill")?;
            let result = calculateSkill.arg(exp.into()).arg(skillMult.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn calculateExp<A: Into<crate::Number>, B: Into<crate::Number>>(
            &self,
            skill: A,
            skillMult: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let calculateExp: crate::Function = self.internal.get("calculateExp")?;
            let result = calculateExp.arg(skill.into()).arg(skillMult.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
    }
    pub struct WorkStats {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for WorkStats {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<WorkStats, Self::Error> {
            Ok(WorkStats {
                internal: Object::from(value),
            })
        }
    }
    impl WorkStats {}
    pub struct WorkFormulas {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for WorkFormulas {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<WorkFormulas, Self::Error> {
            Ok(WorkFormulas {
                internal: Object::from(value),
            })
        }
    }
    impl WorkFormulas {
        pub fn crimeSuccessChance<A: Into<crate::Any>, B: Into<crate::Any>>(
            &self,
            person: A,
            crimeType: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let crimeSuccessChance: crate::Function = self
                .internal
                .get("crimeSuccessChance")?;
            let result = crimeSuccessChance
                .arg(person.into())
                .arg(crimeType.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn crimeGains<A: Into<crate::Any>, B: Into<crate::Any>>(
            &self,
            person: A,
            crimeType: B,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let crimeGains: crate::Function = self.internal.get("crimeGains")?;
            let result = crimeGains.arg(person.into()).arg(crimeType.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn gymGains<A: Into<crate::Any>, B: Into<crate::Any>, C: Into<crate::Any>>(
            &self,
            person: A,
            gymType: B,
            locationName: C,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let gymGains: crate::Function = self.internal.get("gymGains")?;
            let result = gymGains
                .arg(person.into())
                .arg(gymType.into())
                .arg(locationName.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn universityGains<
            A: Into<crate::Any>,
            B: Into<crate::Any>,
            C: Into<crate::Any>,
        >(
            &self,
            person: A,
            classType: B,
            locationName: C,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let universityGains: crate::Function = self.internal.get("universityGains")?;
            let result = universityGains
                .arg(person.into())
                .arg(classType.into())
                .arg(locationName.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn factionGains<
            A: Into<crate::Any>,
            B: Into<crate::Any>,
            C: Into<crate::Number>,
        >(
            &self,
            person: A,
            workType: B,
            favor: C,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let factionGains: crate::Function = self.internal.get("factionGains")?;
            let result = factionGains
                .arg(person.into())
                .arg(workType.into())
                .arg(favor.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn companyGains<
            A: Into<crate::Any>,
            B: Into<crate::Any>,
            C: Into<crate::Any>,
            D: Into<crate::Number>,
        >(
            &self,
            person: A,
            companyName: B,
            workType: C,
            favor: D,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let companyGains: crate::Function = self.internal.get("companyGains")?;
            let result = companyGains
                .arg(person.into())
                .arg(companyName.into())
                .arg(workType.into())
                .arg(favor.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
    }
    pub struct ReputationFormulas {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for ReputationFormulas {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<ReputationFormulas, Self::Error> {
            Ok(ReputationFormulas {
                internal: Object::from(value),
            })
        }
    }
    impl ReputationFormulas {
        pub fn calculateFavorToRep<A: Into<crate::Number>>(
            &self,
            favor: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let calculateFavorToRep: crate::Function = self
                .internal
                .get("calculateFavorToRep")?;
            let result = calculateFavorToRep.arg(favor.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn calculateRepToFavor<A: Into<crate::Number>>(
            &self,
            rep: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let calculateRepToFavor: crate::Function = self
                .internal
                .get("calculateRepToFavor")?;
            let result = calculateRepToFavor.arg(rep.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn repFromDonation<A: Into<crate::Number>, B: Into<crate::Any>>(
            &self,
            amount: A,
            player: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let repFromDonation: crate::Function = self.internal.get("repFromDonation")?;
            let result = repFromDonation.arg(amount.into()).arg(player.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn donationForRep<A: Into<crate::Number>, B: Into<crate::Any>>(
            &self,
            reputation: A,
            player: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let donationForRep: crate::Function = self.internal.get("donationForRep")?;
            let result = donationForRep
                .arg(reputation.into())
                .arg(player.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
    }
    pub struct HackingFormulas {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for HackingFormulas {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<HackingFormulas, Self::Error> {
            Ok(HackingFormulas {
                internal: Object::from(value),
            })
        }
    }
    impl HackingFormulas {
        pub fn hackChance<A: Into<crate::Any>, B: Into<crate::Any>>(
            &self,
            server: A,
            player: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let hackChance: crate::Function = self.internal.get("hackChance")?;
            let result = hackChance.arg(server.into()).arg(player.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn hackExp<A: Into<crate::Any>, B: Into<crate::Any>>(
            &self,
            server: A,
            player: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let hackExp: crate::Function = self.internal.get("hackExp")?;
            let result = hackExp.arg(server.into()).arg(player.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn hackPercent<A: Into<crate::Any>, B: Into<crate::Any>>(
            &self,
            server: A,
            player: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let hackPercent: crate::Function = self.internal.get("hackPercent")?;
            let result = hackPercent.arg(server.into()).arg(player.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn growPercent<
            A: Into<crate::Any>,
            B: Into<crate::Number>,
            C: Into<crate::Any>,
            D: Into<crate::Number>,
        >(
            &self,
            server: A,
            threads: B,
            player: C,
            cores: D,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let growPercent: crate::Function = self.internal.get("growPercent")?;
            let result = growPercent
                .arg(server.into())
                .arg(threads.into())
                .arg(player.into())
                .arg(cores.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn growThreads<
            A: Into<crate::Any>,
            B: Into<crate::Any>,
            C: Into<crate::Number>,
            D: Into<crate::Number>,
        >(
            &self,
            server: A,
            player: B,
            targetMoney: C,
            cores: D,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let growThreads: crate::Function = self.internal.get("growThreads")?;
            let result = growThreads
                .arg(server.into())
                .arg(player.into())
                .arg(targetMoney.into())
                .arg(cores.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn growAmount<
            A: Into<crate::Any>,
            B: Into<crate::Any>,
            C: Into<crate::Number>,
            D: Into<crate::Number>,
        >(
            &self,
            server: A,
            player: B,
            threads: C,
            cores: D,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let growAmount: crate::Function = self.internal.get("growAmount")?;
            let result = growAmount
                .arg(server.into())
                .arg(player.into())
                .arg(threads.into())
                .arg(cores.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn hackTime<A: Into<crate::Any>, B: Into<crate::Any>>(
            &self,
            server: A,
            player: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let hackTime: crate::Function = self.internal.get("hackTime")?;
            let result = hackTime.arg(server.into()).arg(player.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn growTime<A: Into<crate::Any>, B: Into<crate::Any>>(
            &self,
            server: A,
            player: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let growTime: crate::Function = self.internal.get("growTime")?;
            let result = growTime.arg(server.into()).arg(player.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn weakenTime<A: Into<crate::Any>, B: Into<crate::Any>>(
            &self,
            server: A,
            player: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let weakenTime: crate::Function = self.internal.get("weakenTime")?;
            let result = weakenTime.arg(server.into()).arg(player.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
    }
    pub struct HacknetNodesFormulas {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for HacknetNodesFormulas {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<HacknetNodesFormulas, Self::Error> {
            Ok(HacknetNodesFormulas {
                internal: Object::from(value),
            })
        }
    }
    impl HacknetNodesFormulas {
        pub fn moneyGainRate<
            A: Into<crate::Number>,
            B: Into<crate::Number>,
            C: Into<crate::Number>,
            D: Into<crate::Number>,
        >(
            &self,
            level: A,
            ram: B,
            cores: C,
            mult: D,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let moneyGainRate: crate::Function = self.internal.get("moneyGainRate")?;
            let result = moneyGainRate
                .arg(level.into())
                .arg(ram.into())
                .arg(cores.into())
                .arg(mult.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn levelUpgradeCost<
            A: Into<crate::Number>,
            B: Into<crate::Number>,
            C: Into<crate::Number>,
        >(
            &self,
            startingLevel: A,
            extraLevels: B,
            costMult: C,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let levelUpgradeCost: crate::Function = self
                .internal
                .get("levelUpgradeCost")?;
            let result = levelUpgradeCost
                .arg(startingLevel.into())
                .arg(extraLevels.into())
                .arg(costMult.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn ramUpgradeCost<
            A: Into<crate::Number>,
            B: Into<crate::Number>,
            C: Into<crate::Number>,
        >(
            &self,
            startingRam: A,
            extraLevels: B,
            costMult: C,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let ramUpgradeCost: crate::Function = self.internal.get("ramUpgradeCost")?;
            let result = ramUpgradeCost
                .arg(startingRam.into())
                .arg(extraLevels.into())
                .arg(costMult.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn coreUpgradeCost<
            A: Into<crate::Number>,
            B: Into<crate::Number>,
            C: Into<crate::Number>,
        >(
            &self,
            startingCore: A,
            extraCores: B,
            costMult: C,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let coreUpgradeCost: crate::Function = self.internal.get("coreUpgradeCost")?;
            let result = coreUpgradeCost
                .arg(startingCore.into())
                .arg(extraCores.into())
                .arg(costMult.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn hacknetNodeCost<A: Into<crate::Number>, B: Into<crate::Number>>(
            &self,
            n: A,
            mult: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let hacknetNodeCost: crate::Function = self.internal.get("hacknetNodeCost")?;
            let result = hacknetNodeCost.arg(n.into()).arg(mult.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn constants(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let constants: crate::Function = self.internal.get("constants")?;
            let result = constants.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
    }
    pub struct HacknetServersFormulas {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for HacknetServersFormulas {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<HacknetServersFormulas, Self::Error> {
            Ok(HacknetServersFormulas {
                internal: Object::from(value),
            })
        }
    }
    impl HacknetServersFormulas {
        pub fn hashGainRate<
            A: Into<crate::Number>,
            B: Into<crate::Number>,
            C: Into<crate::Number>,
            D: Into<crate::Number>,
            E: Into<crate::Number>,
        >(
            &self,
            level: A,
            ramUsed: B,
            maxRam: C,
            cores: D,
            mult: E,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let hashGainRate: crate::Function = self.internal.get("hashGainRate")?;
            let result = hashGainRate
                .arg(level.into())
                .arg(ramUsed.into())
                .arg(maxRam.into())
                .arg(cores.into())
                .arg(mult.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn levelUpgradeCost<
            A: Into<crate::Number>,
            B: Into<crate::Number>,
            C: Into<crate::Number>,
        >(
            &self,
            startingLevel: A,
            extraLevels: B,
            costMult: C,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let levelUpgradeCost: crate::Function = self
                .internal
                .get("levelUpgradeCost")?;
            let result = levelUpgradeCost
                .arg(startingLevel.into())
                .arg(extraLevels.into())
                .arg(costMult.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn ramUpgradeCost<
            A: Into<crate::Number>,
            B: Into<crate::Number>,
            C: Into<crate::Number>,
        >(
            &self,
            startingRam: A,
            extraLevels: B,
            costMult: C,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let ramUpgradeCost: crate::Function = self.internal.get("ramUpgradeCost")?;
            let result = ramUpgradeCost
                .arg(startingRam.into())
                .arg(extraLevels.into())
                .arg(costMult.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn coreUpgradeCost<
            A: Into<crate::Number>,
            B: Into<crate::Number>,
            C: Into<crate::Number>,
        >(
            &self,
            startingCore: A,
            extraCores: B,
            costMult: C,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let coreUpgradeCost: crate::Function = self.internal.get("coreUpgradeCost")?;
            let result = coreUpgradeCost
                .arg(startingCore.into())
                .arg(extraCores.into())
                .arg(costMult.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn cacheUpgradeCost<A: Into<crate::Number>, B: Into<crate::Number>>(
            &self,
            startingCache: A,
            extraCache: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let cacheUpgradeCost: crate::Function = self
                .internal
                .get("cacheUpgradeCost")?;
            let result = cacheUpgradeCost
                .arg(startingCache.into())
                .arg(extraCache.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn hashUpgradeCost<A: Into<crate::String>, B: Into<crate::Number>>(
            &self,
            upgName: A,
            level: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let hashUpgradeCost: crate::Function = self.internal.get("hashUpgradeCost")?;
            let result = hashUpgradeCost.arg(upgName.into()).arg(level.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn hacknetServerCost<A: Into<crate::Number>, B: Into<crate::Number>>(
            &self,
            n: A,
            mult: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let hacknetServerCost: crate::Function = self
                .internal
                .get("hacknetServerCost")?;
            let result = hacknetServerCost.arg(n.into()).arg(mult.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn constants(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let constants: crate::Function = self.internal.get("constants")?;
            let result = constants.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
    }
    pub struct GangFormulas {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for GangFormulas {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<GangFormulas, Self::Error> {
            Ok(GangFormulas {
                internal: Object::from(value),
            })
        }
    }
    impl GangFormulas {
        pub fn wantedPenalty<A: Into<crate::Any>>(
            &self,
            gang: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let wantedPenalty: crate::Function = self.internal.get("wantedPenalty")?;
            let result = wantedPenalty.arg(gang.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn respectGain<
            A: Into<crate::Any>,
            B: Into<crate::Any>,
            C: Into<crate::Any>,
        >(
            &self,
            gang: A,
            member: B,
            task: C,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let respectGain: crate::Function = self.internal.get("respectGain")?;
            let result = respectGain
                .arg(gang.into())
                .arg(member.into())
                .arg(task.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn wantedLevelGain<
            A: Into<crate::Any>,
            B: Into<crate::Any>,
            C: Into<crate::Any>,
        >(
            &self,
            gang: A,
            member: B,
            task: C,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let wantedLevelGain: crate::Function = self.internal.get("wantedLevelGain")?;
            let result = wantedLevelGain
                .arg(gang.into())
                .arg(member.into())
                .arg(task.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn moneyGain<A: Into<crate::Any>, B: Into<crate::Any>, C: Into<crate::Any>>(
            &self,
            gang: A,
            member: B,
            task: C,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let moneyGain: crate::Function = self.internal.get("moneyGain")?;
            let result = moneyGain
                .arg(gang.into())
                .arg(member.into())
                .arg(task.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn ascensionPointsGain<A: Into<crate::Number>>(
            &self,
            exp: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let ascensionPointsGain: crate::Function = self
                .internal
                .get("ascensionPointsGain")?;
            let result = ascensionPointsGain.arg(exp.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn ascensionMultiplier<A: Into<crate::Number>>(
            &self,
            points: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let ascensionMultiplier: crate::Function = self
                .internal
                .get("ascensionMultiplier")?;
            let result = ascensionMultiplier.arg(points.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
    }
    pub struct BladeburnerFormulas {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for BladeburnerFormulas {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<BladeburnerFormulas, Self::Error> {
            Ok(BladeburnerFormulas {
                internal: Object::from(value),
            })
        }
    }
    impl BladeburnerFormulas {
        pub fn skillMaxUpgradeCount<
            A: Into<crate::Any>,
            B: Into<crate::Number>,
            C: Into<crate::Number>,
        >(
            &self,
            name: A,
            level: B,
            skillPoints: C,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let skillMaxUpgradeCount: crate::Function = self
                .internal
                .get("skillMaxUpgradeCount")?;
            let result = skillMaxUpgradeCount
                .arg(name.into())
                .arg(level.into())
                .arg(skillPoints.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
    }
    pub struct Formulas {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for Formulas {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<Formulas, Self::Error> {
            Ok(Formulas {
                internal: Object::from(value),
            })
        }
    }
    impl Formulas {
        pub fn mockServer(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let mockServer: crate::Function = self.internal.get("mockServer")?;
            let result = mockServer.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn mockPlayer(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let mockPlayer: crate::Function = self.internal.get("mockPlayer")?;
            let result = mockPlayer.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn mockPerson(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let mockPerson: crate::Function = self.internal.get("mockPerson")?;
            let result = mockPerson.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
    }
    pub struct Fragment {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for Fragment {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<Fragment, Self::Error> {
            Ok(Fragment {
                internal: Object::from(value),
            })
        }
    }
    impl Fragment {}
    pub struct ActiveFragment {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for ActiveFragment {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<ActiveFragment, Self::Error> {
            Ok(ActiveFragment {
                internal: Object::from(value),
            })
        }
    }
    impl ActiveFragment {}
    pub struct Stanek {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for Stanek {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<Stanek, Self::Error> {
            Ok(Stanek {
                internal: Object::from(value),
            })
        }
    }
    impl Stanek {
        pub fn giftWidth(&self) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let giftWidth: crate::Function = self.internal.get("giftWidth")?;
            let result = giftWidth.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn giftHeight(&self) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let giftHeight: crate::Function = self.internal.get("giftHeight")?;
            let result = giftHeight.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn chargeFragment<A: Into<crate::Number>, B: Into<crate::Number>>(
            &self,
            rootX: A,
            rootY: B,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let chargeFragment: crate::Function = self.internal.get("chargeFragment")?;
            let result = chargeFragment.arg(rootX.into()).arg(rootY.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn fragmentDefinitions(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let fragmentDefinitions: crate::Function = self
                .internal
                .get("fragmentDefinitions")?;
            let result = fragmentDefinitions.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn activeFragments(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let activeFragments: crate::Function = self.internal.get("activeFragments")?;
            let result = activeFragments.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn clearGift(&self) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let clearGift: crate::Function = self.internal.get("clearGift")?;
            let result = clearGift.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn canPlaceFragment<
            A: Into<crate::Number>,
            B: Into<crate::Number>,
            C: Into<crate::Number>,
            D: Into<crate::Number>,
        >(
            &self,
            rootX: A,
            rootY: B,
            rotation: C,
            fragmentId: D,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let canPlaceFragment: crate::Function = self
                .internal
                .get("canPlaceFragment")?;
            let result = canPlaceFragment
                .arg(rootX.into())
                .arg(rootY.into())
                .arg(rotation.into())
                .arg(fragmentId.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn placeFragment<
            A: Into<crate::Number>,
            B: Into<crate::Number>,
            C: Into<crate::Number>,
            D: Into<crate::Number>,
        >(
            &self,
            rootX: A,
            rootY: B,
            rotation: C,
            fragmentId: D,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let placeFragment: crate::Function = self.internal.get("placeFragment")?;
            let result = placeFragment
                .arg(rootX.into())
                .arg(rootY.into())
                .arg(rotation.into())
                .arg(fragmentId.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getFragment<A: Into<crate::Number>, B: Into<crate::Number>>(
            &self,
            rootX: A,
            rootY: B,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getFragment: crate::Function = self.internal.get("getFragment")?;
            let result = getFragment.arg(rootX.into()).arg(rootY.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn removeFragment<A: Into<crate::Number>, B: Into<crate::Number>>(
            &self,
            rootX: A,
            rootY: B,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let removeFragment: crate::Function = self.internal.get("removeFragment")?;
            let result = removeFragment.arg(rootX.into()).arg(rootY.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn acceptGift(&self) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let acceptGift: crate::Function = self.internal.get("acceptGift")?;
            let result = acceptGift.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
    }
    pub struct InfiltrationReward {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for InfiltrationReward {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<InfiltrationReward, Self::Error> {
            Ok(InfiltrationReward {
                internal: Object::from(value),
            })
        }
    }
    impl InfiltrationReward {}
    pub struct ILocation {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for ILocation {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<ILocation, Self::Error> {
            Ok(ILocation {
                internal: Object::from(value),
            })
        }
    }
    impl ILocation {}
    pub struct InfiltrationLocation {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for InfiltrationLocation {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<InfiltrationLocation, Self::Error> {
            Ok(InfiltrationLocation {
                internal: Object::from(value),
            })
        }
    }
    impl InfiltrationLocation {}
    pub struct Infiltration {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for Infiltration {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<Infiltration, Self::Error> {
            Ok(Infiltration {
                internal: Object::from(value),
            })
        }
    }
    impl Infiltration {
        pub fn getPossibleLocations(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getPossibleLocations: crate::Function = self
                .internal
                .get("getPossibleLocations")?;
            let result = getPossibleLocations.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getInfiltration<A: Into<crate::Any>>(
            &self,
            location: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getInfiltration: crate::Function = self.internal.get("getInfiltration")?;
            let result = getInfiltration.arg(location.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
    }
    pub struct UserInterface {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for UserInterface {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<UserInterface, Self::Error> {
            Ok(UserInterface {
                internal: Object::from(value),
            })
        }
    }
    impl UserInterface {
        pub fn openTail<
            A: Into<crate::Any>,
            B: Into<crate::String>,
            C: Into<crate::Any>,
        >(
            &self,
            _fn: A,
            host: B,
            args: C,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let openTail: crate::Function = self.internal.get("openTail")?;
            let result = openTail
                .arg(_fn.into())
                .arg(host.into())
                .arg(args.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn renderTail<A: Into<crate::Number>>(
            &self,
            pid: A,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let renderTail: crate::Function = self.internal.get("renderTail")?;
            let result = renderTail.arg(pid.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn moveTail<
            A: Into<crate::Number>,
            B: Into<crate::Number>,
            C: Into<crate::Number>,
        >(&self, x: A, y: B, pid: C) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let moveTail: crate::Function = self.internal.get("moveTail")?;
            let result = moveTail.arg(x.into()).arg(y.into()).arg(pid.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn resizeTail<
            A: Into<crate::Number>,
            B: Into<crate::Number>,
            C: Into<crate::Number>,
        >(
            &self,
            width: A,
            height: B,
            pid: C,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let resizeTail: crate::Function = self.internal.get("resizeTail")?;
            let result = resizeTail
                .arg(width.into())
                .arg(height.into())
                .arg(pid.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn closeTail<A: Into<crate::Number>>(
            &self,
            pid: A,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let closeTail: crate::Function = self.internal.get("closeTail")?;
            let result = closeTail.arg(pid.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn setTailTitle<A: Into<crate::Any>, B: Into<crate::Number>>(
            &self,
            title: A,
            pid: B,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let setTailTitle: crate::Function = self.internal.get("setTailTitle")?;
            let result = setTailTitle.arg(title.into()).arg(pid.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn setTailFontSize<
            A: Into<crate::Number>,
            B: Into<crate::Any>,
            C: Into<crate::String>,
            D: Into<crate::Any>,
        >(
            &self,
            pixel: A,
            _fn: B,
            host: C,
            args: D,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let setTailFontSize: crate::Function = self.internal.get("setTailFontSize")?;
            let result = setTailFontSize
                .arg(pixel.into())
                .arg(_fn.into())
                .arg(host.into())
                .arg(args.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn windowSize(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let windowSize: crate::Function = self.internal.get("windowSize")?;
            let result = windowSize.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getTheme(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getTheme: crate::Function = self.internal.get("getTheme")?;
            let result = getTheme.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn setTheme<A: Into<crate::Any>>(
            &self,
            newTheme: A,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let setTheme: crate::Function = self.internal.get("setTheme")?;
            let result = setTheme.arg(newTheme.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn resetTheme(&self) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let resetTheme: crate::Function = self.internal.get("resetTheme")?;
            let result = resetTheme.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getStyles(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getStyles: crate::Function = self.internal.get("getStyles")?;
            let result = getStyles.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn setStyles<A: Into<crate::Any>>(
            &self,
            newStyles: A,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let setStyles: crate::Function = self.internal.get("setStyles")?;
            let result = setStyles.arg(newStyles.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn resetStyles(&self) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let resetStyles: crate::Function = self.internal.get("resetStyles")?;
            let result = resetStyles.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getGameInfo(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getGameInfo: crate::Function = self.internal.get("getGameInfo")?;
            let result = getGameInfo.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn clearTerminal(&self) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let clearTerminal: crate::Function = self.internal.get("clearTerminal")?;
            let result = clearTerminal.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
    }
    pub struct NS {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for NS {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<NS, Self::Error> {
            Ok(NS {
                internal: Object::from(value),
            })
        }
    }
    impl NS {
        pub fn hack<A: Into<crate::String>, B: Into<crate::Any>>(
            &self,
            host: A,
            opts: B,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let hack: crate::Function = self.internal.get("hack")?;
            let result = hack.arg(host.into()).arg(opts.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn grow<A: Into<crate::String>, B: Into<crate::Any>>(
            &self,
            host: A,
            opts: B,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let grow: crate::Function = self.internal.get("grow")?;
            let result = grow.arg(host.into()).arg(opts.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn weaken<A: Into<crate::String>, B: Into<crate::Any>>(
            &self,
            host: A,
            opts: B,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let weaken: crate::Function = self.internal.get("weaken")?;
            let result = weaken.arg(host.into()).arg(opts.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn weakenAnalyze<A: Into<crate::Number>, B: Into<crate::Number>>(
            &self,
            threads: A,
            cores: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let weakenAnalyze: crate::Function = self.internal.get("weakenAnalyze")?;
            let result = weakenAnalyze.arg(threads.into()).arg(cores.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn hackAnalyzeThreads<A: Into<crate::String>, B: Into<crate::Number>>(
            &self,
            host: A,
            hackAmount: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let hackAnalyzeThreads: crate::Function = self
                .internal
                .get("hackAnalyzeThreads")?;
            let result = hackAnalyzeThreads
                .arg(host.into())
                .arg(hackAmount.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn hackAnalyze<A: Into<crate::String>>(
            &self,
            host: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let hackAnalyze: crate::Function = self.internal.get("hackAnalyze")?;
            let result = hackAnalyze.arg(host.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn hackAnalyzeSecurity<A: Into<crate::Number>, B: Into<crate::String>>(
            &self,
            threads: A,
            hostname: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let hackAnalyzeSecurity: crate::Function = self
                .internal
                .get("hackAnalyzeSecurity")?;
            let result = hackAnalyzeSecurity
                .arg(threads.into())
                .arg(hostname.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn hackAnalyzeChance<A: Into<crate::String>>(
            &self,
            host: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let hackAnalyzeChance: crate::Function = self
                .internal
                .get("hackAnalyzeChance")?;
            let result = hackAnalyzeChance.arg(host.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn growthAnalyze<
            A: Into<crate::String>,
            B: Into<crate::Number>,
            C: Into<crate::Number>,
        >(
            &self,
            host: A,
            multiplier: B,
            cores: C,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let growthAnalyze: crate::Function = self.internal.get("growthAnalyze")?;
            let result = growthAnalyze
                .arg(host.into())
                .arg(multiplier.into())
                .arg(cores.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn growthAnalyzeSecurity<
            A: Into<crate::Number>,
            B: Into<crate::String>,
            C: Into<crate::Number>,
        >(
            &self,
            threads: A,
            hostname: B,
            cores: C,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let growthAnalyzeSecurity: crate::Function = self
                .internal
                .get("growthAnalyzeSecurity")?;
            let result = growthAnalyzeSecurity
                .arg(threads.into())
                .arg(hostname.into())
                .arg(cores.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn sleep<A: Into<crate::Number>>(
            &self,
            millis: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let sleep: crate::Function = self.internal.get("sleep")?;
            let result = sleep.arg(millis.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn asleep<A: Into<crate::Number>>(
            &self,
            millis: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let asleep: crate::Function = self.internal.get("asleep")?;
            let result = asleep.arg(millis.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn print<A: Into<crate::Any>>(
            &self,
            args: A,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let print: crate::Function = self.internal.get("print")?;
            let result = print.arg(args.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn printRaw<A: Into<crate::Any>>(
            &self,
            node: A,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let printRaw: crate::Function = self.internal.get("printRaw")?;
            let result = printRaw.arg(node.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn printf<A: Into<crate::String>, B: Into<crate::Any>>(
            &self,
            format: A,
            args: B,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let printf: crate::Function = self.internal.get("printf")?;
            let result = printf.arg(format.into()).arg(args.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn tprint<A: Into<crate::Any>>(
            &self,
            args: A,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let tprint: crate::Function = self.internal.get("tprint")?;
            let result = tprint.arg(args.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn tprintRaw<A: Into<crate::Any>>(
            &self,
            node: A,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let tprintRaw: crate::Function = self.internal.get("tprintRaw")?;
            let result = tprintRaw.arg(node.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn tprintf<A: Into<crate::String>, B: Into<crate::Any>>(
            &self,
            format: A,
            values: B,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let tprintf: crate::Function = self.internal.get("tprintf")?;
            let result = tprintf.arg(format.into()).arg(values.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn clearLog(&self) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let clearLog: crate::Function = self.internal.get("clearLog")?;
            let result = clearLog.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn disableLog<A: Into<crate::String>>(
            &self,
            _fn: A,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let disableLog: crate::Function = self.internal.get("disableLog")?;
            let result = disableLog.arg(_fn.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn enableLog<A: Into<crate::String>>(
            &self,
            _fn: A,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let enableLog: crate::Function = self.internal.get("enableLog")?;
            let result = enableLog.arg(_fn.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn isLogEnabled<A: Into<crate::String>>(
            &self,
            _fn: A,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let isLogEnabled: crate::Function = self.internal.get("isLogEnabled")?;
            let result = isLogEnabled.arg(_fn.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getScriptLogs<
            A: Into<crate::Any>,
            B: Into<crate::String>,
            C: Into<crate::Any>,
        >(&self, _fn: A, host: B, args: C) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getScriptLogs: crate::Function = self.internal.get("getScriptLogs")?;
            let result = getScriptLogs
                .arg(_fn.into())
                .arg(host.into())
                .arg(args.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getRecentScripts(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getRecentScripts: crate::Function = self
                .internal
                .get("getRecentScripts")?;
            let result = getRecentScripts.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn tail<A: Into<crate::Any>, B: Into<crate::String>, C: Into<crate::Any>>(
            &self,
            _fn: A,
            host: B,
            args: C,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let tail: crate::Function = self.internal.get("tail")?;
            let result = tail.arg(_fn.into()).arg(host.into()).arg(args.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn moveTail<
            A: Into<crate::Number>,
            B: Into<crate::Number>,
            C: Into<crate::Number>,
        >(&self, x: A, y: B, pid: C) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let moveTail: crate::Function = self.internal.get("moveTail")?;
            let result = moveTail.arg(x.into()).arg(y.into()).arg(pid.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn resizeTail<
            A: Into<crate::Number>,
            B: Into<crate::Number>,
            C: Into<crate::Number>,
        >(
            &self,
            width: A,
            height: B,
            pid: C,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let resizeTail: crate::Function = self.internal.get("resizeTail")?;
            let result = resizeTail
                .arg(width.into())
                .arg(height.into())
                .arg(pid.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn closeTail<A: Into<crate::Number>>(
            &self,
            pid: A,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let closeTail: crate::Function = self.internal.get("closeTail")?;
            let result = closeTail.arg(pid.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn setTitle<A: Into<crate::Any>, B: Into<crate::Number>>(
            &self,
            title: A,
            pid: B,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let setTitle: crate::Function = self.internal.get("setTitle")?;
            let result = setTitle.arg(title.into()).arg(pid.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn scan<A: Into<crate::String>>(
            &self,
            host: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let scan: crate::Function = self.internal.get("scan")?;
            let result = scan.arg(host.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn hasTorRouter(&self) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let hasTorRouter: crate::Function = self.internal.get("hasTorRouter")?;
            let result = hasTorRouter.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn nuke<A: Into<crate::String>>(
            &self,
            host: A,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let nuke: crate::Function = self.internal.get("nuke")?;
            let result = nuke.arg(host.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn brutessh<A: Into<crate::String>>(
            &self,
            host: A,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let brutessh: crate::Function = self.internal.get("brutessh")?;
            let result = brutessh.arg(host.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn ftpcrack<A: Into<crate::String>>(
            &self,
            host: A,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let ftpcrack: crate::Function = self.internal.get("ftpcrack")?;
            let result = ftpcrack.arg(host.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn relaysmtp<A: Into<crate::String>>(
            &self,
            host: A,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let relaysmtp: crate::Function = self.internal.get("relaysmtp")?;
            let result = relaysmtp.arg(host.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn httpworm<A: Into<crate::String>>(
            &self,
            host: A,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let httpworm: crate::Function = self.internal.get("httpworm")?;
            let result = httpworm.arg(host.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn sqlinject<A: Into<crate::String>>(
            &self,
            host: A,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let sqlinject: crate::Function = self.internal.get("sqlinject")?;
            let result = sqlinject.arg(host.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn run<A: Into<crate::String>, B: Into<crate::Any>, C: Into<crate::Any>>(
            &self,
            script: A,
            threadOrOptions: B,
            args: C,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let run: crate::Function = self.internal.get("run")?;
            let result = run
                .arg(script.into())
                .arg(threadOrOptions.into())
                .arg(args.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn exec<
            A: Into<crate::String>,
            B: Into<crate::String>,
            C: Into<crate::Any>,
            D: Into<crate::Any>,
        >(
            &self,
            script: A,
            hostname: B,
            threadOrOptions: C,
            args: D,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let exec: crate::Function = self.internal.get("exec")?;
            let result = exec
                .arg(script.into())
                .arg(hostname.into())
                .arg(threadOrOptions.into())
                .arg(args.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn spawn<A: Into<crate::String>, B: Into<crate::Any>, C: Into<crate::Any>>(
            &self,
            script: A,
            threadOrOptions: B,
            args: C,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let spawn: crate::Function = self.internal.get("spawn")?;
            let result = spawn
                .arg(script.into())
                .arg(threadOrOptions.into())
                .arg(args.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn _self(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let _self: crate::Function = self.internal.get("_self")?;
            let result = _self.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn kill0<A: Into<crate::Number>>(
            &self,
            pid: A,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let kill: crate::Function = self.internal.get("kill")?;
            let result = kill.arg(pid.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn kill1<
            A: Into<crate::String>,
            B: Into<crate::String>,
            C: Into<crate::Any>,
        >(
            &self,
            filename: A,
            hostname: B,
            args: C,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let kill: crate::Function = self.internal.get("kill")?;
            let result = kill
                .arg(filename.into())
                .arg(hostname.into())
                .arg(args.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn killall<A: Into<crate::String>, B: Into<crate::Boolean>>(
            &self,
            host: A,
            safetyGuard: B,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let killall: crate::Function = self.internal.get("killall")?;
            let result = killall.arg(host.into()).arg(safetyGuard.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn exit(&self) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let exit: crate::Function = self.internal.get("exit")?;
            let result = exit.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn scp<A: Into<crate::Any>, B: Into<crate::String>, C: Into<crate::String>>(
            &self,
            files: A,
            destination: B,
            source: C,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let scp: crate::Function = self.internal.get("scp")?;
            let result = scp
                .arg(files.into())
                .arg(destination.into())
                .arg(source.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn ls<A: Into<crate::String>, B: Into<crate::String>>(
            &self,
            host: A,
            substring: B,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let ls: crate::Function = self.internal.get("ls")?;
            let result = ls.arg(host.into()).arg(substring.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn ps<A: Into<crate::String>>(
            &self,
            host: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let ps: crate::Function = self.internal.get("ps")?;
            let result = ps.arg(host.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn hasRootAccess<A: Into<crate::String>>(
            &self,
            host: A,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let hasRootAccess: crate::Function = self.internal.get("hasRootAccess")?;
            let result = hasRootAccess.arg(host.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getHostname(&self) -> Result<crate::String, wasm_bindgen::JsValue> {
            let getHostname: crate::Function = self.internal.get("getHostname")?;
            let result = getHostname.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getHackingLevel(&self) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getHackingLevel: crate::Function = self.internal.get("getHackingLevel")?;
            let result = getHackingLevel.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getHackingMultipliers(
            &self,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getHackingMultipliers: crate::Function = self
                .internal
                .get("getHackingMultipliers")?;
            let result = getHackingMultipliers.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getHacknetMultipliers(
            &self,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getHacknetMultipliers: crate::Function = self
                .internal
                .get("getHacknetMultipliers")?;
            let result = getHacknetMultipliers.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getServer<A: Into<crate::String>>(
            &self,
            host: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getServer: crate::Function = self.internal.get("getServer")?;
            let result = getServer.arg(host.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getServerMoneyAvailable<A: Into<crate::String>>(
            &self,
            host: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getServerMoneyAvailable: crate::Function = self
                .internal
                .get("getServerMoneyAvailable")?;
            let result = getServerMoneyAvailable.arg(host.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getServerMaxMoney<A: Into<crate::String>>(
            &self,
            host: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getServerMaxMoney: crate::Function = self
                .internal
                .get("getServerMaxMoney")?;
            let result = getServerMaxMoney.arg(host.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getServerGrowth<A: Into<crate::String>>(
            &self,
            host: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getServerGrowth: crate::Function = self.internal.get("getServerGrowth")?;
            let result = getServerGrowth.arg(host.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getServerSecurityLevel<A: Into<crate::String>>(
            &self,
            host: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getServerSecurityLevel: crate::Function = self
                .internal
                .get("getServerSecurityLevel")?;
            let result = getServerSecurityLevel.arg(host.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getServerMinSecurityLevel<A: Into<crate::String>>(
            &self,
            host: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getServerMinSecurityLevel: crate::Function = self
                .internal
                .get("getServerMinSecurityLevel")?;
            let result = getServerMinSecurityLevel.arg(host.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getServerBaseSecurityLevel<A: Into<crate::String>>(
            &self,
            host: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getServerBaseSecurityLevel: crate::Function = self
                .internal
                .get("getServerBaseSecurityLevel")?;
            let result = getServerBaseSecurityLevel.arg(host.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getServerMaxRam<A: Into<crate::String>>(
            &self,
            host: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getServerMaxRam: crate::Function = self.internal.get("getServerMaxRam")?;
            let result = getServerMaxRam.arg(host.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getServerUsedRam<A: Into<crate::String>>(
            &self,
            host: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getServerUsedRam: crate::Function = self
                .internal
                .get("getServerUsedRam")?;
            let result = getServerUsedRam.arg(host.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getServerRequiredHackingLevel<A: Into<crate::String>>(
            &self,
            host: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getServerRequiredHackingLevel: crate::Function = self
                .internal
                .get("getServerRequiredHackingLevel")?;
            let result = getServerRequiredHackingLevel.arg(host.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getServerNumPortsRequired<A: Into<crate::String>>(
            &self,
            host: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getServerNumPortsRequired: crate::Function = self
                .internal
                .get("getServerNumPortsRequired")?;
            let result = getServerNumPortsRequired.arg(host.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn serverExists<A: Into<crate::String>>(
            &self,
            host: A,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let serverExists: crate::Function = self.internal.get("serverExists")?;
            let result = serverExists.arg(host.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn fileExists<A: Into<crate::String>, B: Into<crate::String>>(
            &self,
            filename: A,
            host: B,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let fileExists: crate::Function = self.internal.get("fileExists")?;
            let result = fileExists.arg(filename.into()).arg(host.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn isRunning<
            A: Into<crate::Any>,
            B: Into<crate::String>,
            C: Into<crate::Any>,
        >(
            &self,
            script: A,
            host: B,
            args: C,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let isRunning: crate::Function = self.internal.get("isRunning")?;
            let result = isRunning
                .arg(script.into())
                .arg(host.into())
                .arg(args.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getRunningScript<
            A: Into<crate::Any>,
            B: Into<crate::String>,
            C: Into<crate::Any>,
        >(
            &self,
            filename: A,
            hostname: B,
            args: C,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getRunningScript: crate::Function = self
                .internal
                .get("getRunningScript")?;
            let result = getRunningScript
                .arg(filename.into())
                .arg(hostname.into())
                .arg(args.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn ramOverride<A: Into<crate::Number>>(
            &self,
            ram: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let ramOverride: crate::Function = self.internal.get("ramOverride")?;
            let result = ramOverride.arg(ram.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getPurchasedServerCost<A: Into<crate::Number>>(
            &self,
            ram: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getPurchasedServerCost: crate::Function = self
                .internal
                .get("getPurchasedServerCost")?;
            let result = getPurchasedServerCost.arg(ram.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn purchaseServer<A: Into<crate::String>, B: Into<crate::Number>>(
            &self,
            hostname: A,
            ram: B,
        ) -> Result<crate::String, wasm_bindgen::JsValue> {
            let purchaseServer: crate::Function = self.internal.get("purchaseServer")?;
            let result = purchaseServer.arg(hostname.into()).arg(ram.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getPurchasedServerUpgradeCost<
            A: Into<crate::String>,
            B: Into<crate::Number>,
        >(&self, hostname: A, ram: B) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getPurchasedServerUpgradeCost: crate::Function = self
                .internal
                .get("getPurchasedServerUpgradeCost")?;
            let result = getPurchasedServerUpgradeCost
                .arg(hostname.into())
                .arg(ram.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn upgradePurchasedServer<A: Into<crate::String>, B: Into<crate::Number>>(
            &self,
            hostname: A,
            ram: B,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let upgradePurchasedServer: crate::Function = self
                .internal
                .get("upgradePurchasedServer")?;
            let result = upgradePurchasedServer
                .arg(hostname.into())
                .arg(ram.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn renamePurchasedServer<A: Into<crate::String>, B: Into<crate::String>>(
            &self,
            hostname: A,
            newName: B,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let renamePurchasedServer: crate::Function = self
                .internal
                .get("renamePurchasedServer")?;
            let result = renamePurchasedServer
                .arg(hostname.into())
                .arg(newName.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn deleteServer<A: Into<crate::String>>(
            &self,
            host: A,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let deleteServer: crate::Function = self.internal.get("deleteServer")?;
            let result = deleteServer.arg(host.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getPurchasedServers(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getPurchasedServers: crate::Function = self
                .internal
                .get("getPurchasedServers")?;
            let result = getPurchasedServers.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getPurchasedServerLimit(
            &self,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getPurchasedServerLimit: crate::Function = self
                .internal
                .get("getPurchasedServerLimit")?;
            let result = getPurchasedServerLimit.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getPurchasedServerMaxRam(
            &self,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getPurchasedServerMaxRam: crate::Function = self
                .internal
                .get("getPurchasedServerMaxRam")?;
            let result = getPurchasedServerMaxRam.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn write<
            A: Into<crate::String>,
            B: Into<crate::String>,
            C: Into<crate::Any>,
        >(
            &self,
            filename: A,
            data: B,
            mode: C,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let write: crate::Function = self.internal.get("write")?;
            let result = write
                .arg(filename.into())
                .arg(data.into())
                .arg(mode.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn tryWritePort<A: Into<crate::Number>, B: Into<crate::Any>>(
            &self,
            portNumber: A,
            data: B,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let tryWritePort: crate::Function = self.internal.get("tryWritePort")?;
            let result = tryWritePort.arg(portNumber.into()).arg(data.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn nextPortWrite<A: Into<crate::Number>>(
            &self,
            port: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let nextPortWrite: crate::Function = self.internal.get("nextPortWrite")?;
            let result = nextPortWrite.arg(port.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn read<A: Into<crate::String>>(
            &self,
            filename: A,
        ) -> Result<crate::String, wasm_bindgen::JsValue> {
            let read: crate::Function = self.internal.get("read")?;
            let result = read.arg(filename.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn peek<A: Into<crate::Number>>(
            &self,
            portNumber: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let peek: crate::Function = self.internal.get("peek")?;
            let result = peek.arg(portNumber.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn clear<A: Into<crate::String>>(
            &self,
            handle: A,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let clear: crate::Function = self.internal.get("clear")?;
            let result = clear.arg(handle.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn clearPort<A: Into<crate::Number>>(
            &self,
            portNumber: A,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let clearPort: crate::Function = self.internal.get("clearPort")?;
            let result = clearPort.arg(portNumber.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn writePort<A: Into<crate::Number>, B: Into<crate::Any>>(
            &self,
            portNumber: A,
            data: B,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let writePort: crate::Function = self.internal.get("writePort")?;
            let result = writePort.arg(portNumber.into()).arg(data.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn readPort<A: Into<crate::Number>>(
            &self,
            portNumber: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let readPort: crate::Function = self.internal.get("readPort")?;
            let result = readPort.arg(portNumber.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getPortHandle<A: Into<crate::Number>>(
            &self,
            portNumber: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getPortHandle: crate::Function = self.internal.get("getPortHandle")?;
            let result = getPortHandle.arg(portNumber.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn rm<A: Into<crate::String>, B: Into<crate::String>>(
            &self,
            name: A,
            host: B,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let rm: crate::Function = self.internal.get("rm")?;
            let result = rm.arg(name.into()).arg(host.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn scriptRunning<A: Into<crate::String>, B: Into<crate::String>>(
            &self,
            script: A,
            host: B,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let scriptRunning: crate::Function = self.internal.get("scriptRunning")?;
            let result = scriptRunning.arg(script.into()).arg(host.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn scriptKill<A: Into<crate::String>, B: Into<crate::String>>(
            &self,
            script: A,
            host: B,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let scriptKill: crate::Function = self.internal.get("scriptKill")?;
            let result = scriptKill.arg(script.into()).arg(host.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getScriptName(&self) -> Result<crate::String, wasm_bindgen::JsValue> {
            let getScriptName: crate::Function = self.internal.get("getScriptName")?;
            let result = getScriptName.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getScriptRam<A: Into<crate::String>, B: Into<crate::String>>(
            &self,
            script: A,
            host: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getScriptRam: crate::Function = self.internal.get("getScriptRam")?;
            let result = getScriptRam.arg(script.into()).arg(host.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getHackTime<A: Into<crate::String>>(
            &self,
            host: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getHackTime: crate::Function = self.internal.get("getHackTime")?;
            let result = getHackTime.arg(host.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getGrowTime<A: Into<crate::String>>(
            &self,
            host: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getGrowTime: crate::Function = self.internal.get("getGrowTime")?;
            let result = getGrowTime.arg(host.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getWeakenTime<A: Into<crate::String>>(
            &self,
            host: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getWeakenTime: crate::Function = self.internal.get("getWeakenTime")?;
            let result = getWeakenTime.arg(host.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getTotalScriptIncome(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getTotalScriptIncome: crate::Function = self
                .internal
                .get("getTotalScriptIncome")?;
            let result = getTotalScriptIncome.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getScriptIncome<
            A: Into<crate::String>,
            B: Into<crate::String>,
            C: Into<crate::Any>,
        >(
            &self,
            script: A,
            host: B,
            args: C,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getScriptIncome: crate::Function = self.internal.get("getScriptIncome")?;
            let result = getScriptIncome
                .arg(script.into())
                .arg(host.into())
                .arg(args.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getTotalScriptExpGain(
            &self,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getTotalScriptExpGain: crate::Function = self
                .internal
                .get("getTotalScriptExpGain")?;
            let result = getTotalScriptExpGain.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getScriptExpGain<
            A: Into<crate::String>,
            B: Into<crate::String>,
            C: Into<crate::Any>,
        >(
            &self,
            script: A,
            host: B,
            args: C,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getScriptExpGain: crate::Function = self
                .internal
                .get("getScriptExpGain")?;
            let result = getScriptExpGain
                .arg(script.into())
                .arg(host.into())
                .arg(args.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getTimeSinceLastAug(
            &self,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getTimeSinceLastAug: crate::Function = self
                .internal
                .get("getTimeSinceLastAug")?;
            let result = getTimeSinceLastAug.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn sprintf<A: Into<crate::String>, B: Into<crate::Any>>(
            &self,
            format: A,
            args: B,
        ) -> Result<crate::String, wasm_bindgen::JsValue> {
            let sprintf: crate::Function = self.internal.get("sprintf")?;
            let result = sprintf.arg(format.into()).arg(args.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn vsprintf<A: Into<crate::String>, B: Into<crate::Any>>(
            &self,
            format: A,
            args: B,
        ) -> Result<crate::String, wasm_bindgen::JsValue> {
            let vsprintf: crate::Function = self.internal.get("vsprintf")?;
            let result = vsprintf.arg(format.into()).arg(args.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn formatNumber<
            A: Into<crate::Number>,
            B: Into<crate::Number>,
            C: Into<crate::Number>,
            D: Into<crate::Boolean>,
        >(
            &self,
            n: A,
            fractionalDigits: B,
            suffixStart: C,
            isInteger: D,
        ) -> Result<crate::String, wasm_bindgen::JsValue> {
            let formatNumber: crate::Function = self.internal.get("formatNumber")?;
            let result = formatNumber
                .arg(n.into())
                .arg(fractionalDigits.into())
                .arg(suffixStart.into())
                .arg(isInteger.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn formatRam<A: Into<crate::Number>, B: Into<crate::Number>>(
            &self,
            n: A,
            fractionalDigits: B,
        ) -> Result<crate::String, wasm_bindgen::JsValue> {
            let formatRam: crate::Function = self.internal.get("formatRam")?;
            let result = formatRam.arg(n.into()).arg(fractionalDigits.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn formatPercent<
            A: Into<crate::Number>,
            B: Into<crate::Number>,
            C: Into<crate::Number>,
        >(
            &self,
            n: A,
            fractionalDigits: B,
            suffixStart: C,
        ) -> Result<crate::String, wasm_bindgen::JsValue> {
            let formatPercent: crate::Function = self.internal.get("formatPercent")?;
            let result = formatPercent
                .arg(n.into())
                .arg(fractionalDigits.into())
                .arg(suffixStart.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn nFormat<A: Into<crate::Number>, B: Into<crate::String>>(
            &self,
            n: A,
            format: B,
        ) -> Result<crate::String, wasm_bindgen::JsValue> {
            let nFormat: crate::Function = self.internal.get("nFormat")?;
            let result = nFormat.arg(n.into()).arg(format.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn tFormat<A: Into<crate::Number>, B: Into<crate::Boolean>>(
            &self,
            milliseconds: A,
            milliPrecision: B,
        ) -> Result<crate::String, wasm_bindgen::JsValue> {
            let tFormat: crate::Function = self.internal.get("tFormat")?;
            let result = tFormat
                .arg(milliseconds.into())
                .arg(milliPrecision.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn prompt<A: Into<crate::String>, B: Into<crate::Any>>(
            &self,
            txt: A,
            options: B,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let prompt: crate::Function = self.internal.get("prompt")?;
            let result = prompt.arg(txt.into()).arg(options.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn alert<A: Into<crate::String>>(
            &self,
            msg: A,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let alert: crate::Function = self.internal.get("alert")?;
            let result = alert.arg(msg.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn toast<A: Into<crate::String>, B: Into<crate::Any>, C: Into<crate::Any>>(
            &self,
            msg: A,
            variant: B,
            duration: C,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let toast: crate::Function = self.internal.get("toast")?;
            let result = toast
                .arg(msg.into())
                .arg(variant.into())
                .arg(duration.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn wget<
            A: Into<crate::String>,
            B: Into<crate::String>,
            C: Into<crate::String>,
        >(
            &self,
            url: A,
            target: B,
            host: C,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let wget: crate::Function = self.internal.get("wget")?;
            let result = wget
                .arg(url.into())
                .arg(target.into())
                .arg(host.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getFavorToDonate(&self) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getFavorToDonate: crate::Function = self
                .internal
                .get("getFavorToDonate")?;
            let result = getFavorToDonate.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getBitNodeMultipliers<A: Into<crate::Number>, B: Into<crate::Number>>(
            &self,
            n: A,
            lvl: B,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getBitNodeMultipliers: crate::Function = self
                .internal
                .get("getBitNodeMultipliers")?;
            let result = getBitNodeMultipliers.arg(n.into()).arg(lvl.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getPlayer(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getPlayer: crate::Function = self.internal.get("getPlayer")?;
            let result = getPlayer.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getMoneySources(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getMoneySources: crate::Function = self.internal.get("getMoneySources")?;
            let result = getMoneySources.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn atExit<A: Into<crate::Any>, B: Into<crate::String>>(
            &self,
            f: A,
            id: B,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let atExit: crate::Function = self.internal.get("atExit")?;
            let result = atExit.arg(f.into()).arg(id.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn mv<
            A: Into<crate::String>,
            B: Into<crate::String>,
            C: Into<crate::String>,
        >(
            &self,
            host: A,
            source: B,
            destination: C,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let mv: crate::Function = self.internal.get("mv")?;
            let result = mv
                .arg(host.into())
                .arg(source.into())
                .arg(destination.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getResetInfo(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getResetInfo: crate::Function = self.internal.get("getResetInfo")?;
            let result = getResetInfo.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getFunctionRamCost<A: Into<crate::String>>(
            &self,
            name: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getFunctionRamCost: crate::Function = self
                .internal
                .get("getFunctionRamCost")?;
            let result = getFunctionRamCost.arg(name.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn flags<A: Into<crate::Any>>(
            &self,
            schema: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let flags: crate::Function = self.internal.get("flags")?;
            let result = flags.arg(schema.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn share(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let share: crate::Function = self.internal.get("share")?;
            let result = share.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getSharePower(&self) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getSharePower: crate::Function = self.internal.get("getSharePower")?;
            let result = getSharePower.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
    }
    pub enum ToastVariant {
        SUCCESS,
        WARNING,
        ERROR,
        INFO,
    }
    impl ToastVariant {
        fn as_any(&self) -> crate::Any {
            match self {
                ToastVariant::SUCCESS => "success".into(),
                ToastVariant::WARNING => "warning".into(),
                ToastVariant::ERROR => "error".into(),
                ToastVariant::INFO => "info".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    impl From<ToastVariant> for crate::Any {
        fn from(value: ToastVariant) -> crate::Any {
            match value {
                ToastVariant::SUCCESS => "success".into(),
                ToastVariant::WARNING => "warning".into(),
                ToastVariant::ERROR => "error".into(),
                ToastVariant::INFO => "info".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    pub enum CrimeType {
        shoplift,
        robStore,
        mug,
        larceny,
        dealDrugs,
        bondForgery,
        traffickArms,
        homicide,
        grandTheftAuto,
        kidnap,
        assassination,
        heist,
    }
    impl CrimeType {
        fn as_any(&self) -> crate::Any {
            match self {
                CrimeType::shoplift => "Shoplift".into(),
                CrimeType::robStore => "Rob Store".into(),
                CrimeType::mug => "Mug".into(),
                CrimeType::larceny => "Larceny".into(),
                CrimeType::dealDrugs => "Deal Drugs".into(),
                CrimeType::bondForgery => "Bond Forgery".into(),
                CrimeType::traffickArms => "Traffick Arms".into(),
                CrimeType::homicide => "Homicide".into(),
                CrimeType::grandTheftAuto => "Grand Theft Auto".into(),
                CrimeType::kidnap => "Kidnap".into(),
                CrimeType::assassination => "Assassination".into(),
                CrimeType::heist => "Heist".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    impl From<CrimeType> for crate::Any {
        fn from(value: CrimeType) -> crate::Any {
            match value {
                CrimeType::shoplift => "Shoplift".into(),
                CrimeType::robStore => "Rob Store".into(),
                CrimeType::mug => "Mug".into(),
                CrimeType::larceny => "Larceny".into(),
                CrimeType::dealDrugs => "Deal Drugs".into(),
                CrimeType::bondForgery => "Bond Forgery".into(),
                CrimeType::traffickArms => "Traffick Arms".into(),
                CrimeType::homicide => "Homicide".into(),
                CrimeType::grandTheftAuto => "Grand Theft Auto".into(),
                CrimeType::kidnap => "Kidnap".into(),
                CrimeType::assassination => "Assassination".into(),
                CrimeType::heist => "Heist".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    pub enum FactionWorkType {
        hacking,
        field,
        security,
    }
    impl FactionWorkType {
        fn as_any(&self) -> crate::Any {
            match self {
                FactionWorkType::hacking => "hacking".into(),
                FactionWorkType::field => "field".into(),
                FactionWorkType::security => "security".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    impl From<FactionWorkType> for crate::Any {
        fn from(value: FactionWorkType) -> crate::Any {
            match value {
                FactionWorkType::hacking => "hacking".into(),
                FactionWorkType::field => "field".into(),
                FactionWorkType::security => "security".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    pub enum UniversityClassType {
        computerScience,
        dataStructures,
        networks,
        algorithms,
        management,
        leadership,
    }
    impl UniversityClassType {
        fn as_any(&self) -> crate::Any {
            match self {
                UniversityClassType::computerScience => "Computer Science".into(),
                UniversityClassType::dataStructures => "Data Structures".into(),
                UniversityClassType::networks => "Networks".into(),
                UniversityClassType::algorithms => "Algorithms".into(),
                UniversityClassType::management => "Management".into(),
                UniversityClassType::leadership => "Leadership".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    impl From<UniversityClassType> for crate::Any {
        fn from(value: UniversityClassType) -> crate::Any {
            match value {
                UniversityClassType::computerScience => "Computer Science".into(),
                UniversityClassType::dataStructures => "Data Structures".into(),
                UniversityClassType::networks => "Networks".into(),
                UniversityClassType::algorithms => "Algorithms".into(),
                UniversityClassType::management => "Management".into(),
                UniversityClassType::leadership => "Leadership".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    pub enum GymType {
        strength,
        defense,
        dexterity,
        agility,
    }
    impl GymType {
        fn as_any(&self) -> crate::Any {
            match self {
                GymType::strength => "str".into(),
                GymType::defense => "def".into(),
                GymType::dexterity => "dex".into(),
                GymType::agility => "agi".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    impl From<GymType> for crate::Any {
        fn from(value: GymType) -> crate::Any {
            match value {
                GymType::strength => "str".into(),
                GymType::defense => "def".into(),
                GymType::dexterity => "dex".into(),
                GymType::agility => "agi".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    pub enum JobName {
        software0,
        software1,
        software2,
        software3,
        software4,
        software5,
        software6,
        software7,
        IT0,
        IT1,
        IT2,
        IT3,
        securityEng,
        networkEng0,
        networkEng1,
        business0,
        business1,
        business2,
        business3,
        business4,
        business5,
        security0,
        security1,
        security2,
        security3,
        agent0,
        agent1,
        agent2,
        waiter,
        employee,
        softwareConsult0,
        softwareConsult1,
        businessConsult0,
        businessConsult1,
        waiterPT,
        employeePT,
    }
    impl JobName {
        fn as_any(&self) -> crate::Any {
            match self {
                JobName::software0 => "Software Engineering Intern".into(),
                JobName::software1 => "Junior Software Engineer".into(),
                JobName::software2 => "Senior Software Engineer".into(),
                JobName::software3 => "Lead Software Developer".into(),
                JobName::software4 => "Head of Software".into(),
                JobName::software5 => "Head of Engineering".into(),
                JobName::software6 => "Vice President of Technology".into(),
                JobName::software7 => "Chief Technology Officer".into(),
                JobName::IT0 => "IT Intern".into(),
                JobName::IT1 => "IT Analyst".into(),
                JobName::IT2 => "IT Manager".into(),
                JobName::IT3 => "Systems Administrator".into(),
                JobName::securityEng => "Security Engineer".into(),
                JobName::networkEng0 => "Network Engineer".into(),
                JobName::networkEng1 => "Network Administrator".into(),
                JobName::business0 => "Business Intern".into(),
                JobName::business1 => "Business Analyst".into(),
                JobName::business2 => "Business Manager".into(),
                JobName::business3 => "Operations Manager".into(),
                JobName::business4 => "Chief Financial Officer".into(),
                JobName::business5 => "Chief Executive Officer".into(),
                JobName::security0 => "Security Guard".into(),
                JobName::security1 => "Security Officer".into(),
                JobName::security2 => "Security Supervisor".into(),
                JobName::security3 => "Head of Security".into(),
                JobName::agent0 => "Field Agent".into(),
                JobName::agent1 => "Secret Agent".into(),
                JobName::agent2 => "Special Operative".into(),
                JobName::waiter => "Waiter".into(),
                JobName::employee => "Employee".into(),
                JobName::softwareConsult0 => "Software Consultant".into(),
                JobName::softwareConsult1 => "Senior Software Consultant".into(),
                JobName::businessConsult0 => "Business Consultant".into(),
                JobName::businessConsult1 => "Senior Business Consultant".into(),
                JobName::waiterPT => "Part-time Waiter".into(),
                JobName::employeePT => "Part-time Employee".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    impl From<JobName> for crate::Any {
        fn from(value: JobName) -> crate::Any {
            match value {
                JobName::software0 => "Software Engineering Intern".into(),
                JobName::software1 => "Junior Software Engineer".into(),
                JobName::software2 => "Senior Software Engineer".into(),
                JobName::software3 => "Lead Software Developer".into(),
                JobName::software4 => "Head of Software".into(),
                JobName::software5 => "Head of Engineering".into(),
                JobName::software6 => "Vice President of Technology".into(),
                JobName::software7 => "Chief Technology Officer".into(),
                JobName::IT0 => "IT Intern".into(),
                JobName::IT1 => "IT Analyst".into(),
                JobName::IT2 => "IT Manager".into(),
                JobName::IT3 => "Systems Administrator".into(),
                JobName::securityEng => "Security Engineer".into(),
                JobName::networkEng0 => "Network Engineer".into(),
                JobName::networkEng1 => "Network Administrator".into(),
                JobName::business0 => "Business Intern".into(),
                JobName::business1 => "Business Analyst".into(),
                JobName::business2 => "Business Manager".into(),
                JobName::business3 => "Operations Manager".into(),
                JobName::business4 => "Chief Financial Officer".into(),
                JobName::business5 => "Chief Executive Officer".into(),
                JobName::security0 => "Security Guard".into(),
                JobName::security1 => "Security Officer".into(),
                JobName::security2 => "Security Supervisor".into(),
                JobName::security3 => "Head of Security".into(),
                JobName::agent0 => "Field Agent".into(),
                JobName::agent1 => "Secret Agent".into(),
                JobName::agent2 => "Special Operative".into(),
                JobName::waiter => "Waiter".into(),
                JobName::employee => "Employee".into(),
                JobName::softwareConsult0 => "Software Consultant".into(),
                JobName::softwareConsult1 => "Senior Software Consultant".into(),
                JobName::businessConsult0 => "Business Consultant".into(),
                JobName::businessConsult1 => "Senior Business Consultant".into(),
                JobName::waiterPT => "Part-time Waiter".into(),
                JobName::employeePT => "Part-time Employee".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    pub enum JobField {
        software,
        softwareConsultant,
        it,
        securityEngineer,
        networkEngineer,
        business,
        businessConsultant,
        security,
        agent,
        employee,
        partTimeEmployee,
        waiter,
        partTimeWaiter,
    }
    impl JobField {
        fn as_any(&self) -> crate::Any {
            match self {
                JobField::software => "Software".into(),
                JobField::softwareConsultant => "Software Consultant".into(),
                JobField::it => "IT".into(),
                JobField::securityEngineer => "Security Engineer".into(),
                JobField::networkEngineer => "Network Engineer".into(),
                JobField::business => "Business".into(),
                JobField::businessConsultant => "Business Consultant".into(),
                JobField::security => "Security".into(),
                JobField::agent => "Agent".into(),
                JobField::employee => "Employee".into(),
                JobField::partTimeEmployee => "Part-time Employee".into(),
                JobField::waiter => "Waiter".into(),
                JobField::partTimeWaiter => "Part-time Waiter".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    impl From<JobField> for crate::Any {
        fn from(value: JobField) -> crate::Any {
            match value {
                JobField::software => "Software".into(),
                JobField::softwareConsultant => "Software Consultant".into(),
                JobField::it => "IT".into(),
                JobField::securityEngineer => "Security Engineer".into(),
                JobField::networkEngineer => "Network Engineer".into(),
                JobField::business => "Business".into(),
                JobField::businessConsultant => "Business Consultant".into(),
                JobField::security => "Security".into(),
                JobField::agent => "Agent".into(),
                JobField::employee => "Employee".into(),
                JobField::partTimeEmployee => "Part-time Employee".into(),
                JobField::waiter => "Waiter".into(),
                JobField::partTimeWaiter => "Part-time Waiter".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    struct CorpEmployeePosition {}
    struct CorpIndustryName {}
    struct CorpSmartSupplyOption {}
    pub enum CityName {
        Aevum,
        Chongqing,
        Sector12,
        NewTokyo,
        Ishima,
        Volhaven,
    }
    impl CityName {
        fn as_any(&self) -> crate::Any {
            match self {
                CityName::Aevum => "Aevum".into(),
                CityName::Chongqing => "Chongqing".into(),
                CityName::Sector12 => "Sector-12".into(),
                CityName::NewTokyo => "New Tokyo".into(),
                CityName::Ishima => "Ishima".into(),
                CityName::Volhaven => "Volhaven".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    impl From<CityName> for crate::Any {
        fn from(value: CityName) -> crate::Any {
            match value {
                CityName::Aevum => "Aevum".into(),
                CityName::Chongqing => "Chongqing".into(),
                CityName::Sector12 => "Sector-12".into(),
                CityName::NewTokyo => "New Tokyo".into(),
                CityName::Ishima => "Ishima".into(),
                CityName::Volhaven => "Volhaven".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    pub enum LocationName {
        AevumAeroCorp,
        AevumBachmanAndAssociates,
        AevumClarkeIncorporated,
        AevumCrushFitnessGym,
        AevumECorp,
        AevumFulcrumTechnologies,
        AevumGalacticCybersystems,
        AevumNetLinkTechnologies,
        AevumPolice,
        AevumRhoConstruction,
        AevumSnapFitnessGym,
        AevumSummitUniversity,
        AevumWatchdogSecurity,
        AevumCasino,
        ChongqingKuaiGongInternational,
        ChongqingSolarisSpaceSystems,
        ChongqingChurchOfTheMachineGod,
        Sector12AlphaEnterprises,
        Sector12BladeIndustries,
        Sector12CIA,
        Sector12CarmichaelSecurity,
        Sector12CityHall,
        Sector12DeltaOne,
        Sector12FoodNStuff,
        Sector12FourSigma,
        Sector12IcarusMicrosystems,
        Sector12IronGym,
        Sector12JoesGuns,
        Sector12MegaCorp,
        Sector12NSA,
        Sector12PowerhouseGym,
        Sector12RothmanUniversity,
        Sector12UniversalEnergy,
        NewTokyoDefComm,
        NewTokyoGlobalPharmaceuticals,
        NewTokyoNoodleBar,
        NewTokyoVitaLife,
        NewTokyoArcade,
        IshimaNovaMedical,
        IshimaOmegaSoftware,
        IshimaStormTechnologies,
        IshimaGlitch,
        VolhavenCompuTek,
        VolhavenHeliosLabs,
        VolhavenLexoCorp,
        VolhavenMilleniumFitnessGym,
        VolhavenNWO,
        VolhavenOmniTekIncorporated,
        VolhavenOmniaCybersystems,
        VolhavenSysCoreSecurities,
        VolhavenZBInstituteOfTechnology,
        Hospital,
        Slums,
        TravelAgency,
        WorldStockExchange,
        Void,
    }
    impl LocationName {
        fn as_any(&self) -> crate::Any {
            match self {
                LocationName::AevumAeroCorp => "AeroCorp".into(),
                LocationName::AevumBachmanAndAssociates => "Bachman & Associates".into(),
                LocationName::AevumClarkeIncorporated => "Clarke Incorporated".into(),
                LocationName::AevumCrushFitnessGym => "Crush Fitness Gym".into(),
                LocationName::AevumECorp => "ECorp".into(),
                LocationName::AevumFulcrumTechnologies => "Fulcrum Technologies".into(),
                LocationName::AevumGalacticCybersystems => "Galactic Cybersystems".into(),
                LocationName::AevumNetLinkTechnologies => "NetLink Technologies".into(),
                LocationName::AevumPolice => "Aevum Police Headquarters".into(),
                LocationName::AevumRhoConstruction => "Rho Construction".into(),
                LocationName::AevumSnapFitnessGym => "Snap Fitness Gym".into(),
                LocationName::AevumSummitUniversity => "Summit University".into(),
                LocationName::AevumWatchdogSecurity => "Watchdog Security".into(),
                LocationName::AevumCasino => "Iker Molina Casino".into(),
                LocationName::ChongqingKuaiGongInternational => {
                    "KuaiGong International".into()
                }
                LocationName::ChongqingSolarisSpaceSystems => {
                    "Solaris Space Systems".into()
                }
                LocationName::ChongqingChurchOfTheMachineGod => {
                    "Church of the Machine God".into()
                }
                LocationName::Sector12AlphaEnterprises => "Alpha Enterprises".into(),
                LocationName::Sector12BladeIndustries => "Blade Industries".into(),
                LocationName::Sector12CIA => "Central Intelligence Agency".into(),
                LocationName::Sector12CarmichaelSecurity => "Carmichael Security".into(),
                LocationName::Sector12CityHall => "Sector-12 City Hall".into(),
                LocationName::Sector12DeltaOne => "DeltaOne".into(),
                LocationName::Sector12FoodNStuff => "FoodNStuff".into(),
                LocationName::Sector12FourSigma => "Four Sigma".into(),
                LocationName::Sector12IcarusMicrosystems => "Icarus Microsystems".into(),
                LocationName::Sector12IronGym => "Iron Gym".into(),
                LocationName::Sector12JoesGuns => "Joe's Guns".into(),
                LocationName::Sector12MegaCorp => "MegaCorp".into(),
                LocationName::Sector12NSA => "National Security Agency".into(),
                LocationName::Sector12PowerhouseGym => "Powerhouse Gym".into(),
                LocationName::Sector12RothmanUniversity => "Rothman University".into(),
                LocationName::Sector12UniversalEnergy => "Universal Energy".into(),
                LocationName::NewTokyoDefComm => "DefComm".into(),
                LocationName::NewTokyoGlobalPharmaceuticals => {
                    "Global Pharmaceuticals".into()
                }
                LocationName::NewTokyoNoodleBar => "Noodle Bar".into(),
                LocationName::NewTokyoVitaLife => "VitaLife".into(),
                LocationName::NewTokyoArcade => "Arcade".into(),
                LocationName::IshimaNovaMedical => "Nova Medical".into(),
                LocationName::IshimaOmegaSoftware => "Omega Software".into(),
                LocationName::IshimaStormTechnologies => "Storm Technologies".into(),
                LocationName::IshimaGlitch => "0x6C1".into(),
                LocationName::VolhavenCompuTek => "CompuTek".into(),
                LocationName::VolhavenHeliosLabs => "Helios Labs".into(),
                LocationName::VolhavenLexoCorp => "LexoCorp".into(),
                LocationName::VolhavenMilleniumFitnessGym => {
                    "Millenium Fitness Gym".into()
                }
                LocationName::VolhavenNWO => "NWO".into(),
                LocationName::VolhavenOmniTekIncorporated => {
                    "OmniTek Incorporated".into()
                }
                LocationName::VolhavenOmniaCybersystems => "Omnia Cybersystems".into(),
                LocationName::VolhavenSysCoreSecurities => "SysCore Securities".into(),
                LocationName::VolhavenZBInstituteOfTechnology => {
                    "ZB Institute of Technology".into()
                }
                LocationName::Hospital => "Hospital".into(),
                LocationName::Slums => "The Slums".into(),
                LocationName::TravelAgency => "Travel Agency".into(),
                LocationName::WorldStockExchange => "World Stock Exchange".into(),
                LocationName::Void => "The Void".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    impl From<LocationName> for crate::Any {
        fn from(value: LocationName) -> crate::Any {
            match value {
                LocationName::AevumAeroCorp => "AeroCorp".into(),
                LocationName::AevumBachmanAndAssociates => "Bachman & Associates".into(),
                LocationName::AevumClarkeIncorporated => "Clarke Incorporated".into(),
                LocationName::AevumCrushFitnessGym => "Crush Fitness Gym".into(),
                LocationName::AevumECorp => "ECorp".into(),
                LocationName::AevumFulcrumTechnologies => "Fulcrum Technologies".into(),
                LocationName::AevumGalacticCybersystems => "Galactic Cybersystems".into(),
                LocationName::AevumNetLinkTechnologies => "NetLink Technologies".into(),
                LocationName::AevumPolice => "Aevum Police Headquarters".into(),
                LocationName::AevumRhoConstruction => "Rho Construction".into(),
                LocationName::AevumSnapFitnessGym => "Snap Fitness Gym".into(),
                LocationName::AevumSummitUniversity => "Summit University".into(),
                LocationName::AevumWatchdogSecurity => "Watchdog Security".into(),
                LocationName::AevumCasino => "Iker Molina Casino".into(),
                LocationName::ChongqingKuaiGongInternational => {
                    "KuaiGong International".into()
                }
                LocationName::ChongqingSolarisSpaceSystems => {
                    "Solaris Space Systems".into()
                }
                LocationName::ChongqingChurchOfTheMachineGod => {
                    "Church of the Machine God".into()
                }
                LocationName::Sector12AlphaEnterprises => "Alpha Enterprises".into(),
                LocationName::Sector12BladeIndustries => "Blade Industries".into(),
                LocationName::Sector12CIA => "Central Intelligence Agency".into(),
                LocationName::Sector12CarmichaelSecurity => "Carmichael Security".into(),
                LocationName::Sector12CityHall => "Sector-12 City Hall".into(),
                LocationName::Sector12DeltaOne => "DeltaOne".into(),
                LocationName::Sector12FoodNStuff => "FoodNStuff".into(),
                LocationName::Sector12FourSigma => "Four Sigma".into(),
                LocationName::Sector12IcarusMicrosystems => "Icarus Microsystems".into(),
                LocationName::Sector12IronGym => "Iron Gym".into(),
                LocationName::Sector12JoesGuns => "Joe's Guns".into(),
                LocationName::Sector12MegaCorp => "MegaCorp".into(),
                LocationName::Sector12NSA => "National Security Agency".into(),
                LocationName::Sector12PowerhouseGym => "Powerhouse Gym".into(),
                LocationName::Sector12RothmanUniversity => "Rothman University".into(),
                LocationName::Sector12UniversalEnergy => "Universal Energy".into(),
                LocationName::NewTokyoDefComm => "DefComm".into(),
                LocationName::NewTokyoGlobalPharmaceuticals => {
                    "Global Pharmaceuticals".into()
                }
                LocationName::NewTokyoNoodleBar => "Noodle Bar".into(),
                LocationName::NewTokyoVitaLife => "VitaLife".into(),
                LocationName::NewTokyoArcade => "Arcade".into(),
                LocationName::IshimaNovaMedical => "Nova Medical".into(),
                LocationName::IshimaOmegaSoftware => "Omega Software".into(),
                LocationName::IshimaStormTechnologies => "Storm Technologies".into(),
                LocationName::IshimaGlitch => "0x6C1".into(),
                LocationName::VolhavenCompuTek => "CompuTek".into(),
                LocationName::VolhavenHeliosLabs => "Helios Labs".into(),
                LocationName::VolhavenLexoCorp => "LexoCorp".into(),
                LocationName::VolhavenMilleniumFitnessGym => {
                    "Millenium Fitness Gym".into()
                }
                LocationName::VolhavenNWO => "NWO".into(),
                LocationName::VolhavenOmniTekIncorporated => {
                    "OmniTek Incorporated".into()
                }
                LocationName::VolhavenOmniaCybersystems => "Omnia Cybersystems".into(),
                LocationName::VolhavenSysCoreSecurities => "SysCore Securities".into(),
                LocationName::VolhavenZBInstituteOfTechnology => {
                    "ZB Institute of Technology".into()
                }
                LocationName::Hospital => "Hospital".into(),
                LocationName::Slums => "The Slums".into(),
                LocationName::TravelAgency => "Travel Agency".into(),
                LocationName::WorldStockExchange => "World Stock Exchange".into(),
                LocationName::Void => "The Void".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    pub enum UniversityLocationName {
        AevumSummitUniversity,
        Sector12RothmanUniversity,
        VolhavenZBInstituteOfTechnology,
    }
    impl UniversityLocationName {
        fn as_any(&self) -> crate::Any {
            match self {
                UniversityLocationName::AevumSummitUniversity => {
                    LocationName::AevumSummitUniversity.into()
                }
                UniversityLocationName::Sector12RothmanUniversity => {
                    LocationName::Sector12RothmanUniversity.into()
                }
                UniversityLocationName::VolhavenZBInstituteOfTechnology => {
                    LocationName::VolhavenZBInstituteOfTechnology.into()
                }
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    impl From<UniversityLocationName> for crate::Any {
        fn from(value: UniversityLocationName) -> crate::Any {
            match value {
                UniversityLocationName::AevumSummitUniversity => {
                    LocationName::AevumSummitUniversity.into()
                }
                UniversityLocationName::Sector12RothmanUniversity => {
                    LocationName::Sector12RothmanUniversity.into()
                }
                UniversityLocationName::VolhavenZBInstituteOfTechnology => {
                    LocationName::VolhavenZBInstituteOfTechnology.into()
                }
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    pub enum GymLocationName {
        AevumCrushFitnessGym,
        AevumSnapFitnessGym,
        Sector12IronGym,
        Sector12PowerhouseGym,
        VolhavenMilleniumFitnessGym,
    }
    impl GymLocationName {
        fn as_any(&self) -> crate::Any {
            match self {
                GymLocationName::AevumCrushFitnessGym => {
                    LocationName::AevumCrushFitnessGym.into()
                }
                GymLocationName::AevumSnapFitnessGym => {
                    LocationName::AevumSnapFitnessGym.into()
                }
                GymLocationName::Sector12IronGym => LocationName::Sector12IronGym.into(),
                GymLocationName::Sector12PowerhouseGym => {
                    LocationName::Sector12PowerhouseGym.into()
                }
                GymLocationName::VolhavenMilleniumFitnessGym => {
                    LocationName::VolhavenMilleniumFitnessGym.into()
                }
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    impl From<GymLocationName> for crate::Any {
        fn from(value: GymLocationName) -> crate::Any {
            match value {
                GymLocationName::AevumCrushFitnessGym => {
                    LocationName::AevumCrushFitnessGym.into()
                }
                GymLocationName::AevumSnapFitnessGym => {
                    LocationName::AevumSnapFitnessGym.into()
                }
                GymLocationName::Sector12IronGym => LocationName::Sector12IronGym.into(),
                GymLocationName::Sector12PowerhouseGym => {
                    LocationName::Sector12PowerhouseGym.into()
                }
                GymLocationName::VolhavenMilleniumFitnessGym => {
                    LocationName::VolhavenMilleniumFitnessGym.into()
                }
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    pub enum CompanyName {
        ECorp,
        MegaCorp,
        BachmanAndAssociates,
        BladeIndustries,
        NWO,
        ClarkeIncorporated,
        OmniTekIncorporated,
        FourSigma,
        KuaiGongInternational,
        FulcrumTechnologies,
        StormTechnologies,
        DefComm,
        HeliosLabs,
        VitaLife,
        IcarusMicrosystems,
        UniversalEnergy,
        GalacticCybersystems,
        AeroCorp,
        OmniaCybersystems,
        SolarisSpaceSystems,
        DeltaOne,
        GlobalPharmaceuticals,
        NovaMedical,
        CIA,
        NSA,
        WatchdogSecurity,
        LexoCorp,
        RhoConstruction,
        AlphaEnterprises,
        Police,
        SysCoreSecurities,
        CompuTek,
        NetLinkTechnologies,
        CarmichaelSecurity,
        FoodNStuff,
        JoesGuns,
        OmegaSoftware,
        NoodleBar,
    }
    impl CompanyName {
        fn as_any(&self) -> crate::Any {
            match self {
                CompanyName::ECorp => "ECorp".into(),
                CompanyName::MegaCorp => "MegaCorp".into(),
                CompanyName::BachmanAndAssociates => "Bachman & Associates".into(),
                CompanyName::BladeIndustries => "Blade Industries".into(),
                CompanyName::NWO => "NWO".into(),
                CompanyName::ClarkeIncorporated => "Clarke Incorporated".into(),
                CompanyName::OmniTekIncorporated => "OmniTek Incorporated".into(),
                CompanyName::FourSigma => "Four Sigma".into(),
                CompanyName::KuaiGongInternational => "KuaiGong International".into(),
                CompanyName::FulcrumTechnologies => "Fulcrum Technologies".into(),
                CompanyName::StormTechnologies => "Storm Technologies".into(),
                CompanyName::DefComm => "DefComm".into(),
                CompanyName::HeliosLabs => "Helios Labs".into(),
                CompanyName::VitaLife => "VitaLife".into(),
                CompanyName::IcarusMicrosystems => "Icarus Microsystems".into(),
                CompanyName::UniversalEnergy => "Universal Energy".into(),
                CompanyName::GalacticCybersystems => "Galactic Cybersystems".into(),
                CompanyName::AeroCorp => "AeroCorp".into(),
                CompanyName::OmniaCybersystems => "Omnia Cybersystems".into(),
                CompanyName::SolarisSpaceSystems => "Solaris Space Systems".into(),
                CompanyName::DeltaOne => "DeltaOne".into(),
                CompanyName::GlobalPharmaceuticals => "Global Pharmaceuticals".into(),
                CompanyName::NovaMedical => "Nova Medical".into(),
                CompanyName::CIA => "Central Intelligence Agency".into(),
                CompanyName::NSA => "National Security Agency".into(),
                CompanyName::WatchdogSecurity => "Watchdog Security".into(),
                CompanyName::LexoCorp => "LexoCorp".into(),
                CompanyName::RhoConstruction => "Rho Construction".into(),
                CompanyName::AlphaEnterprises => "Alpha Enterprises".into(),
                CompanyName::Police => "Aevum Police Headquarters".into(),
                CompanyName::SysCoreSecurities => "SysCore Securities".into(),
                CompanyName::CompuTek => "CompuTek".into(),
                CompanyName::NetLinkTechnologies => "NetLink Technologies".into(),
                CompanyName::CarmichaelSecurity => "Carmichael Security".into(),
                CompanyName::FoodNStuff => "FoodNStuff".into(),
                CompanyName::JoesGuns => "Joe's Guns".into(),
                CompanyName::OmegaSoftware => "Omega Software".into(),
                CompanyName::NoodleBar => "Noodle Bar".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    impl From<CompanyName> for crate::Any {
        fn from(value: CompanyName) -> crate::Any {
            match value {
                CompanyName::ECorp => "ECorp".into(),
                CompanyName::MegaCorp => "MegaCorp".into(),
                CompanyName::BachmanAndAssociates => "Bachman & Associates".into(),
                CompanyName::BladeIndustries => "Blade Industries".into(),
                CompanyName::NWO => "NWO".into(),
                CompanyName::ClarkeIncorporated => "Clarke Incorporated".into(),
                CompanyName::OmniTekIncorporated => "OmniTek Incorporated".into(),
                CompanyName::FourSigma => "Four Sigma".into(),
                CompanyName::KuaiGongInternational => "KuaiGong International".into(),
                CompanyName::FulcrumTechnologies => "Fulcrum Technologies".into(),
                CompanyName::StormTechnologies => "Storm Technologies".into(),
                CompanyName::DefComm => "DefComm".into(),
                CompanyName::HeliosLabs => "Helios Labs".into(),
                CompanyName::VitaLife => "VitaLife".into(),
                CompanyName::IcarusMicrosystems => "Icarus Microsystems".into(),
                CompanyName::UniversalEnergy => "Universal Energy".into(),
                CompanyName::GalacticCybersystems => "Galactic Cybersystems".into(),
                CompanyName::AeroCorp => "AeroCorp".into(),
                CompanyName::OmniaCybersystems => "Omnia Cybersystems".into(),
                CompanyName::SolarisSpaceSystems => "Solaris Space Systems".into(),
                CompanyName::DeltaOne => "DeltaOne".into(),
                CompanyName::GlobalPharmaceuticals => "Global Pharmaceuticals".into(),
                CompanyName::NovaMedical => "Nova Medical".into(),
                CompanyName::CIA => "Central Intelligence Agency".into(),
                CompanyName::NSA => "National Security Agency".into(),
                CompanyName::WatchdogSecurity => "Watchdog Security".into(),
                CompanyName::LexoCorp => "LexoCorp".into(),
                CompanyName::RhoConstruction => "Rho Construction".into(),
                CompanyName::AlphaEnterprises => "Alpha Enterprises".into(),
                CompanyName::Police => "Aevum Police Headquarters".into(),
                CompanyName::SysCoreSecurities => "SysCore Securities".into(),
                CompanyName::CompuTek => "CompuTek".into(),
                CompanyName::NetLinkTechnologies => "NetLink Technologies".into(),
                CompanyName::CarmichaelSecurity => "Carmichael Security".into(),
                CompanyName::FoodNStuff => "FoodNStuff".into(),
                CompanyName::JoesGuns => "Joe's Guns".into(),
                CompanyName::OmegaSoftware => "Omega Software".into(),
                CompanyName::NoodleBar => "Noodle Bar".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    pub enum FactionName {
        Illuminati,
        Daedalus,
        TheCovenant,
        ECorp,
        MegaCorp,
        BachmanAssociates,
        BladeIndustries,
        NWO,
        ClarkeIncorporated,
        OmniTekIncorporated,
        FourSigma,
        KuaiGongInternational,
        FulcrumSecretTechnologies,
        BitRunners,
        TheBlackHand,
        NiteSec,
        Aevum,
        Chongqing,
        Ishima,
        NewTokyo,
        Sector12,
        Volhaven,
        SpeakersForTheDead,
        TheDarkArmy,
        TheSyndicate,
        Silhouette,
        Tetrads,
        SlumSnakes,
        Netburners,
        TianDiHui,
        CyberSec,
        Bladeburners,
        ChurchOfTheMachineGod,
        ShadowsOfAnarchy,
    }
    impl FactionName {
        fn as_any(&self) -> crate::Any {
            match self {
                FactionName::Illuminati => "Illuminati".into(),
                FactionName::Daedalus => "Daedalus".into(),
                FactionName::TheCovenant => "The Covenant".into(),
                FactionName::ECorp => "ECorp".into(),
                FactionName::MegaCorp => "MegaCorp".into(),
                FactionName::BachmanAssociates => "Bachman & Associates".into(),
                FactionName::BladeIndustries => "Blade Industries".into(),
                FactionName::NWO => "NWO".into(),
                FactionName::ClarkeIncorporated => "Clarke Incorporated".into(),
                FactionName::OmniTekIncorporated => "OmniTek Incorporated".into(),
                FactionName::FourSigma => "Four Sigma".into(),
                FactionName::KuaiGongInternational => "KuaiGong International".into(),
                FactionName::FulcrumSecretTechnologies => {
                    "Fulcrum Secret Technologies".into()
                }
                FactionName::BitRunners => "BitRunners".into(),
                FactionName::TheBlackHand => "The Black Hand".into(),
                FactionName::NiteSec => "NiteSec".into(),
                FactionName::Aevum => "Aevum".into(),
                FactionName::Chongqing => "Chongqing".into(),
                FactionName::Ishima => "Ishima".into(),
                FactionName::NewTokyo => "New Tokyo".into(),
                FactionName::Sector12 => "Sector-12".into(),
                FactionName::Volhaven => "Volhaven".into(),
                FactionName::SpeakersForTheDead => "Speakers for the Dead".into(),
                FactionName::TheDarkArmy => "The Dark Army".into(),
                FactionName::TheSyndicate => "The Syndicate".into(),
                FactionName::Silhouette => "Silhouette".into(),
                FactionName::Tetrads => "Tetrads".into(),
                FactionName::SlumSnakes => "Slum Snakes".into(),
                FactionName::Netburners => "Netburners".into(),
                FactionName::TianDiHui => "Tian Di Hui".into(),
                FactionName::CyberSec => "CyberSec".into(),
                FactionName::Bladeburners => "Bladeburners".into(),
                FactionName::ChurchOfTheMachineGod => "Church of the Machine God".into(),
                FactionName::ShadowsOfAnarchy => "Shadows of Anarchy".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    impl From<FactionName> for crate::Any {
        fn from(value: FactionName) -> crate::Any {
            match value {
                FactionName::Illuminati => "Illuminati".into(),
                FactionName::Daedalus => "Daedalus".into(),
                FactionName::TheCovenant => "The Covenant".into(),
                FactionName::ECorp => "ECorp".into(),
                FactionName::MegaCorp => "MegaCorp".into(),
                FactionName::BachmanAssociates => "Bachman & Associates".into(),
                FactionName::BladeIndustries => "Blade Industries".into(),
                FactionName::NWO => "NWO".into(),
                FactionName::ClarkeIncorporated => "Clarke Incorporated".into(),
                FactionName::OmniTekIncorporated => "OmniTek Incorporated".into(),
                FactionName::FourSigma => "Four Sigma".into(),
                FactionName::KuaiGongInternational => "KuaiGong International".into(),
                FactionName::FulcrumSecretTechnologies => {
                    "Fulcrum Secret Technologies".into()
                }
                FactionName::BitRunners => "BitRunners".into(),
                FactionName::TheBlackHand => "The Black Hand".into(),
                FactionName::NiteSec => "NiteSec".into(),
                FactionName::Aevum => "Aevum".into(),
                FactionName::Chongqing => "Chongqing".into(),
                FactionName::Ishima => "Ishima".into(),
                FactionName::NewTokyo => "New Tokyo".into(),
                FactionName::Sector12 => "Sector-12".into(),
                FactionName::Volhaven => "Volhaven".into(),
                FactionName::SpeakersForTheDead => "Speakers for the Dead".into(),
                FactionName::TheDarkArmy => "The Dark Army".into(),
                FactionName::TheSyndicate => "The Syndicate".into(),
                FactionName::Silhouette => "Silhouette".into(),
                FactionName::Tetrads => "Tetrads".into(),
                FactionName::SlumSnakes => "Slum Snakes".into(),
                FactionName::Netburners => "Netburners".into(),
                FactionName::TianDiHui => "Tian Di Hui".into(),
                FactionName::CyberSec => "CyberSec".into(),
                FactionName::Bladeburners => "Bladeburners".into(),
                FactionName::ChurchOfTheMachineGod => "Church of the Machine God".into(),
                FactionName::ShadowsOfAnarchy => "Shadows of Anarchy".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    pub enum CodingContractName {
        FindLargestPrimeFactor,
        SubarrayWithMaximumSum,
        TotalWaysToSum,
        TotalWaysToSumII,
        SpiralizeMatrix,
        ArrayJumpingGame,
        ArrayJumpingGameII,
        MergeOverlappingIntervals,
        GenerateIPAddresses,
        AlgorithmicStockTraderI,
        AlgorithmicStockTraderII,
        AlgorithmicStockTraderIII,
        AlgorithmicStockTraderIV,
        MinimumPathSumInATriangle,
        UniquePathsInAGridI,
        UniquePathsInAGridII,
        ShortestPathInAGrid,
        SanitizeParenthesesInExpression,
        FindAllValidMathExpressions,
        HammingCodesIntegerToEncodedBinary,
        HammingCodesEncodedBinaryToInteger,
        Proper2ColoringOfAGraph,
        CompressionIRLECompression,
        CompressionIILZDecompression,
        CompressionIIILZCompression,
        EncryptionICaesarCipher,
        EncryptionIIVigenereCipher,
        SquareRoot,
    }
    impl CodingContractName {
        fn as_any(&self) -> crate::Any {
            match self {
                CodingContractName::FindLargestPrimeFactor => {
                    "Find Largest Prime Factor".into()
                }
                CodingContractName::SubarrayWithMaximumSum => {
                    "Subarray with Maximum Sum".into()
                }
                CodingContractName::TotalWaysToSum => "Total Ways to Sum".into(),
                CodingContractName::TotalWaysToSumII => "Total Ways to Sum II".into(),
                CodingContractName::SpiralizeMatrix => "Spiralize Matrix".into(),
                CodingContractName::ArrayJumpingGame => "Array Jumping Game".into(),
                CodingContractName::ArrayJumpingGameII => "Array Jumping Game II".into(),
                CodingContractName::MergeOverlappingIntervals => {
                    "Merge Overlapping Intervals".into()
                }
                CodingContractName::GenerateIPAddresses => "Generate IP Addresses".into(),
                CodingContractName::AlgorithmicStockTraderI => {
                    "Algorithmic Stock Trader I".into()
                }
                CodingContractName::AlgorithmicStockTraderII => {
                    "Algorithmic Stock Trader II".into()
                }
                CodingContractName::AlgorithmicStockTraderIII => {
                    "Algorithmic Stock Trader III".into()
                }
                CodingContractName::AlgorithmicStockTraderIV => {
                    "Algorithmic Stock Trader IV".into()
                }
                CodingContractName::MinimumPathSumInATriangle => {
                    "Minimum Path Sum in a Triangle".into()
                }
                CodingContractName::UniquePathsInAGridI => {
                    "Unique Paths in a Grid I".into()
                }
                CodingContractName::UniquePathsInAGridII => {
                    "Unique Paths in a Grid II".into()
                }
                CodingContractName::ShortestPathInAGrid => {
                    "Shortest Path in a Grid".into()
                }
                CodingContractName::SanitizeParenthesesInExpression => {
                    "Sanitize Parentheses in Expression".into()
                }
                CodingContractName::FindAllValidMathExpressions => {
                    "Find All Valid Math Expressions".into()
                }
                CodingContractName::HammingCodesIntegerToEncodedBinary => {
                    "HammingCodes: Integer to Encoded Binary".into()
                }
                CodingContractName::HammingCodesEncodedBinaryToInteger => {
                    "HammingCodes: Encoded Binary to Integer".into()
                }
                CodingContractName::Proper2ColoringOfAGraph => {
                    "Proper 2-Coloring of a Graph".into()
                }
                CodingContractName::CompressionIRLECompression => {
                    "Compression I: RLE Compression".into()
                }
                CodingContractName::CompressionIILZDecompression => {
                    "Compression II: LZ Decompression".into()
                }
                CodingContractName::CompressionIIILZCompression => {
                    "Compression III: LZ Compression".into()
                }
                CodingContractName::EncryptionICaesarCipher => {
                    "Encryption I: Caesar Cipher".into()
                }
                CodingContractName::EncryptionIIVigenereCipher => {
                    "Encryption II: Vigenère Cipher".into()
                }
                CodingContractName::SquareRoot => "Square Root".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    impl From<CodingContractName> for crate::Any {
        fn from(value: CodingContractName) -> crate::Any {
            match value {
                CodingContractName::FindLargestPrimeFactor => {
                    "Find Largest Prime Factor".into()
                }
                CodingContractName::SubarrayWithMaximumSum => {
                    "Subarray with Maximum Sum".into()
                }
                CodingContractName::TotalWaysToSum => "Total Ways to Sum".into(),
                CodingContractName::TotalWaysToSumII => "Total Ways to Sum II".into(),
                CodingContractName::SpiralizeMatrix => "Spiralize Matrix".into(),
                CodingContractName::ArrayJumpingGame => "Array Jumping Game".into(),
                CodingContractName::ArrayJumpingGameII => "Array Jumping Game II".into(),
                CodingContractName::MergeOverlappingIntervals => {
                    "Merge Overlapping Intervals".into()
                }
                CodingContractName::GenerateIPAddresses => "Generate IP Addresses".into(),
                CodingContractName::AlgorithmicStockTraderI => {
                    "Algorithmic Stock Trader I".into()
                }
                CodingContractName::AlgorithmicStockTraderII => {
                    "Algorithmic Stock Trader II".into()
                }
                CodingContractName::AlgorithmicStockTraderIII => {
                    "Algorithmic Stock Trader III".into()
                }
                CodingContractName::AlgorithmicStockTraderIV => {
                    "Algorithmic Stock Trader IV".into()
                }
                CodingContractName::MinimumPathSumInATriangle => {
                    "Minimum Path Sum in a Triangle".into()
                }
                CodingContractName::UniquePathsInAGridI => {
                    "Unique Paths in a Grid I".into()
                }
                CodingContractName::UniquePathsInAGridII => {
                    "Unique Paths in a Grid II".into()
                }
                CodingContractName::ShortestPathInAGrid => {
                    "Shortest Path in a Grid".into()
                }
                CodingContractName::SanitizeParenthesesInExpression => {
                    "Sanitize Parentheses in Expression".into()
                }
                CodingContractName::FindAllValidMathExpressions => {
                    "Find All Valid Math Expressions".into()
                }
                CodingContractName::HammingCodesIntegerToEncodedBinary => {
                    "HammingCodes: Integer to Encoded Binary".into()
                }
                CodingContractName::HammingCodesEncodedBinaryToInteger => {
                    "HammingCodes: Encoded Binary to Integer".into()
                }
                CodingContractName::Proper2ColoringOfAGraph => {
                    "Proper 2-Coloring of a Graph".into()
                }
                CodingContractName::CompressionIRLECompression => {
                    "Compression I: RLE Compression".into()
                }
                CodingContractName::CompressionIILZDecompression => {
                    "Compression II: LZ Decompression".into()
                }
                CodingContractName::CompressionIIILZCompression => {
                    "Compression III: LZ Compression".into()
                }
                CodingContractName::EncryptionICaesarCipher => {
                    "Encryption I: Caesar Cipher".into()
                }
                CodingContractName::EncryptionIIVigenereCipher => {
                    "Encryption II: Vigenère Cipher".into()
                }
                CodingContractName::SquareRoot => "Square Root".into(),
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    struct CodingContractSignatures {}
    struct CodingContractData {}
    struct CodingContractAnswer {}
    struct CodingContractObject {}
    struct NSEnums {}
    pub struct OfficeAPI {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for OfficeAPI {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<OfficeAPI, Self::Error> {
            Ok(OfficeAPI {
                internal: Object::from(value),
            })
        }
    }
    impl OfficeAPI {
        pub fn hireEmployee<
            A: Into<crate::String>,
            B: Into<crate::Any>,
            C: Into<crate::Any>,
        >(
            &self,
            divisionName: A,
            city: B,
            employeePosition: C,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let hireEmployee: crate::Function = self.internal.get("hireEmployee")?;
            let result = hireEmployee
                .arg(divisionName.into())
                .arg(city.into())
                .arg(employeePosition.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn upgradeOfficeSize<
            A: Into<crate::String>,
            B: Into<crate::Any>,
            C: Into<crate::Number>,
        >(
            &self,
            divisionName: A,
            city: B,
            size: C,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let upgradeOfficeSize: crate::Function = self
                .internal
                .get("upgradeOfficeSize")?;
            let result = upgradeOfficeSize
                .arg(divisionName.into())
                .arg(city.into())
                .arg(size.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn throwParty<
            A: Into<crate::String>,
            B: Into<crate::Any>,
            C: Into<crate::Number>,
        >(
            &self,
            divisionName: A,
            city: B,
            costPerEmployee: C,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let throwParty: crate::Function = self.internal.get("throwParty")?;
            let result = throwParty
                .arg(divisionName.into())
                .arg(city.into())
                .arg(costPerEmployee.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn buyTea<A: Into<crate::String>, B: Into<crate::Any>>(
            &self,
            divisionName: A,
            city: B,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let buyTea: crate::Function = self.internal.get("buyTea")?;
            let result = buyTea.arg(divisionName.into()).arg(city.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn hireAdVert<A: Into<crate::String>>(
            &self,
            divisionName: A,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let hireAdVert: crate::Function = self.internal.get("hireAdVert")?;
            let result = hireAdVert.arg(divisionName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn research<A: Into<crate::String>, B: Into<crate::String>>(
            &self,
            divisionName: A,
            researchName: B,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let research: crate::Function = self.internal.get("research")?;
            let result = research
                .arg(divisionName.into())
                .arg(researchName.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getOffice<A: Into<crate::String>, B: Into<crate::Any>>(
            &self,
            divisionName: A,
            city: B,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getOffice: crate::Function = self.internal.get("getOffice")?;
            let result = getOffice.arg(divisionName.into()).arg(city.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getHireAdVertCost<A: Into<crate::String>>(
            &self,
            divisionName: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getHireAdVertCost: crate::Function = self
                .internal
                .get("getHireAdVertCost")?;
            let result = getHireAdVertCost.arg(divisionName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getHireAdVertCount<A: Into<crate::String>>(
            &self,
            divisionName: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getHireAdVertCount: crate::Function = self
                .internal
                .get("getHireAdVertCount")?;
            let result = getHireAdVertCount.arg(divisionName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getResearchCost<A: Into<crate::String>, B: Into<crate::String>>(
            &self,
            divisionName: A,
            researchName: B,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getResearchCost: crate::Function = self.internal.get("getResearchCost")?;
            let result = getResearchCost
                .arg(divisionName.into())
                .arg(researchName.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn hasResearched<A: Into<crate::String>, B: Into<crate::String>>(
            &self,
            divisionName: A,
            researchName: B,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let hasResearched: crate::Function = self.internal.get("hasResearched")?;
            let result = hasResearched
                .arg(divisionName.into())
                .arg(researchName.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn setAutoJobAssignment<
            A: Into<crate::String>,
            B: Into<crate::Any>,
            C: Into<crate::String>,
            D: Into<crate::Number>,
        >(
            &self,
            divisionName: A,
            city: B,
            job: C,
            amount: D,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let setAutoJobAssignment: crate::Function = self
                .internal
                .get("setAutoJobAssignment")?;
            let result = setAutoJobAssignment
                .arg(divisionName.into())
                .arg(city.into())
                .arg(job.into())
                .arg(amount.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getOfficeSizeUpgradeCost<
            A: Into<crate::String>,
            B: Into<crate::Any>,
            C: Into<crate::Number>,
        >(
            &self,
            divisionName: A,
            city: B,
            size: C,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getOfficeSizeUpgradeCost: crate::Function = self
                .internal
                .get("getOfficeSizeUpgradeCost")?;
            let result = getOfficeSizeUpgradeCost
                .arg(divisionName.into())
                .arg(city.into())
                .arg(size.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
    }
    pub struct WarehouseAPI {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for WarehouseAPI {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<WarehouseAPI, Self::Error> {
            Ok(WarehouseAPI {
                internal: Object::from(value),
            })
        }
    }
    impl WarehouseAPI {
        pub fn sellMaterial<
            A: Into<crate::String>,
            B: Into<crate::Any>,
            C: Into<crate::String>,
            D: Into<crate::String>,
            E: Into<crate::String>,
        >(
            &self,
            divisionName: A,
            city: B,
            materialName: C,
            amt: D,
            price: E,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let sellMaterial: crate::Function = self.internal.get("sellMaterial")?;
            let result = sellMaterial
                .arg(divisionName.into())
                .arg(city.into())
                .arg(materialName.into())
                .arg(amt.into())
                .arg(price.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn sellProduct<
            A: Into<crate::String>,
            B: Into<crate::Any>,
            C: Into<crate::String>,
            D: Into<crate::String>,
            E: Into<crate::String>,
            F: Into<crate::Boolean>,
        >(
            &self,
            divisionName: A,
            city: B,
            productName: C,
            amt: D,
            price: E,
            all: F,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let sellProduct: crate::Function = self.internal.get("sellProduct")?;
            let result = sellProduct
                .arg(divisionName.into())
                .arg(city.into())
                .arg(productName.into())
                .arg(amt.into())
                .arg(price.into())
                .arg(all.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn discontinueProduct<A: Into<crate::String>, B: Into<crate::String>>(
            &self,
            divisionName: A,
            productName: B,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let discontinueProduct: crate::Function = self
                .internal
                .get("discontinueProduct")?;
            let result = discontinueProduct
                .arg(divisionName.into())
                .arg(productName.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn setSmartSupply<
            A: Into<crate::String>,
            B: Into<crate::Any>,
            C: Into<crate::Boolean>,
        >(
            &self,
            divisionName: A,
            city: B,
            enabled: C,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let setSmartSupply: crate::Function = self.internal.get("setSmartSupply")?;
            let result = setSmartSupply
                .arg(divisionName.into())
                .arg(city.into())
                .arg(enabled.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn setSmartSupplyOption<
            A: Into<crate::String>,
            B: Into<crate::Any>,
            C: Into<crate::String>,
            D: Into<crate::Any>,
        >(
            &self,
            divisionName: A,
            city: B,
            materialName: C,
            option: D,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let setSmartSupplyOption: crate::Function = self
                .internal
                .get("setSmartSupplyOption")?;
            let result = setSmartSupplyOption
                .arg(divisionName.into())
                .arg(city.into())
                .arg(materialName.into())
                .arg(option.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn buyMaterial<
            A: Into<crate::String>,
            B: Into<crate::Any>,
            C: Into<crate::String>,
            D: Into<crate::Number>,
        >(
            &self,
            divisionName: A,
            city: B,
            materialName: C,
            amt: D,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let buyMaterial: crate::Function = self.internal.get("buyMaterial")?;
            let result = buyMaterial
                .arg(divisionName.into())
                .arg(city.into())
                .arg(materialName.into())
                .arg(amt.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn bulkPurchase<
            A: Into<crate::String>,
            B: Into<crate::Any>,
            C: Into<crate::String>,
            D: Into<crate::Number>,
        >(
            &self,
            divisionName: A,
            city: B,
            materialName: C,
            amt: D,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let bulkPurchase: crate::Function = self.internal.get("bulkPurchase")?;
            let result = bulkPurchase
                .arg(divisionName.into())
                .arg(city.into())
                .arg(materialName.into())
                .arg(amt.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getWarehouse<A: Into<crate::String>, B: Into<crate::Any>>(
            &self,
            divisionName: A,
            city: B,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getWarehouse: crate::Function = self.internal.get("getWarehouse")?;
            let result = getWarehouse.arg(divisionName.into()).arg(city.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getProduct<
            A: Into<crate::String>,
            B: Into<crate::Any>,
            C: Into<crate::String>,
        >(
            &self,
            divisionName: A,
            cityName: B,
            productName: C,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getProduct: crate::Function = self.internal.get("getProduct")?;
            let result = getProduct
                .arg(divisionName.into())
                .arg(cityName.into())
                .arg(productName.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getMaterial<
            A: Into<crate::String>,
            B: Into<crate::Any>,
            C: Into<crate::String>,
        >(
            &self,
            divisionName: A,
            city: B,
            materialName: C,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getMaterial: crate::Function = self.internal.get("getMaterial")?;
            let result = getMaterial
                .arg(divisionName.into())
                .arg(city.into())
                .arg(materialName.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn setMaterialMarketTA1<
            A: Into<crate::String>,
            B: Into<crate::Any>,
            C: Into<crate::String>,
            D: Into<crate::Boolean>,
        >(
            &self,
            divisionName: A,
            city: B,
            materialName: C,
            on: D,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let setMaterialMarketTA1: crate::Function = self
                .internal
                .get("setMaterialMarketTA1")?;
            let result = setMaterialMarketTA1
                .arg(divisionName.into())
                .arg(city.into())
                .arg(materialName.into())
                .arg(on.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn setMaterialMarketTA2<
            A: Into<crate::String>,
            B: Into<crate::Any>,
            C: Into<crate::String>,
            D: Into<crate::Boolean>,
        >(
            &self,
            divisionName: A,
            city: B,
            materialName: C,
            on: D,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let setMaterialMarketTA2: crate::Function = self
                .internal
                .get("setMaterialMarketTA2")?;
            let result = setMaterialMarketTA2
                .arg(divisionName.into())
                .arg(city.into())
                .arg(materialName.into())
                .arg(on.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn setProductMarketTA1<
            A: Into<crate::String>,
            B: Into<crate::String>,
            C: Into<crate::Boolean>,
        >(
            &self,
            divisionName: A,
            productName: B,
            on: C,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let setProductMarketTA1: crate::Function = self
                .internal
                .get("setProductMarketTA1")?;
            let result = setProductMarketTA1
                .arg(divisionName.into())
                .arg(productName.into())
                .arg(on.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn setProductMarketTA2<
            A: Into<crate::String>,
            B: Into<crate::String>,
            C: Into<crate::Boolean>,
        >(
            &self,
            divisionName: A,
            productName: B,
            on: C,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let setProductMarketTA2: crate::Function = self
                .internal
                .get("setProductMarketTA2")?;
            let result = setProductMarketTA2
                .arg(divisionName.into())
                .arg(productName.into())
                .arg(on.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn exportMaterial<
            A: Into<crate::String>,
            B: Into<crate::Any>,
            C: Into<crate::String>,
            D: Into<crate::Any>,
            E: Into<crate::String>,
            F: Into<crate::Any>,
        >(
            &self,
            sourceDivision: A,
            sourceCity: B,
            targetDivision: C,
            targetCity: D,
            materialName: E,
            amt: F,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let exportMaterial: crate::Function = self.internal.get("exportMaterial")?;
            let result = exportMaterial
                .arg(sourceDivision.into())
                .arg(sourceCity.into())
                .arg(targetDivision.into())
                .arg(targetCity.into())
                .arg(materialName.into())
                .arg(amt.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn cancelExportMaterial<
            A: Into<crate::String>,
            B: Into<crate::Any>,
            C: Into<crate::String>,
            D: Into<crate::Any>,
            E: Into<crate::String>,
        >(
            &self,
            sourceDivision: A,
            sourceCity: B,
            targetDivision: C,
            targetCity: D,
            materialName: E,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let cancelExportMaterial: crate::Function = self
                .internal
                .get("cancelExportMaterial")?;
            let result = cancelExportMaterial
                .arg(sourceDivision.into())
                .arg(sourceCity.into())
                .arg(targetDivision.into())
                .arg(targetCity.into())
                .arg(materialName.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn purchaseWarehouse<A: Into<crate::String>, B: Into<crate::Any>>(
            &self,
            divisionName: A,
            city: B,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let purchaseWarehouse: crate::Function = self
                .internal
                .get("purchaseWarehouse")?;
            let result = purchaseWarehouse
                .arg(divisionName.into())
                .arg(city.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn upgradeWarehouse<
            A: Into<crate::String>,
            B: Into<crate::Any>,
            C: Into<crate::Number>,
        >(
            &self,
            divisionName: A,
            city: B,
            amt: C,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let upgradeWarehouse: crate::Function = self
                .internal
                .get("upgradeWarehouse")?;
            let result = upgradeWarehouse
                .arg(divisionName.into())
                .arg(city.into())
                .arg(amt.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn makeProduct<
            A: Into<crate::String>,
            B: Into<crate::Any>,
            C: Into<crate::String>,
            D: Into<crate::Number>,
            E: Into<crate::Number>,
        >(
            &self,
            divisionName: A,
            city: B,
            productName: C,
            designInvest: D,
            marketingInvest: E,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let makeProduct: crate::Function = self.internal.get("makeProduct")?;
            let result = makeProduct
                .arg(divisionName.into())
                .arg(city.into())
                .arg(productName.into())
                .arg(designInvest.into())
                .arg(marketingInvest.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn limitMaterialProduction<
            A: Into<crate::String>,
            B: Into<crate::Any>,
            C: Into<crate::String>,
            D: Into<crate::Number>,
        >(
            &self,
            divisionName: A,
            city: B,
            materialName: C,
            qty: D,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let limitMaterialProduction: crate::Function = self
                .internal
                .get("limitMaterialProduction")?;
            let result = limitMaterialProduction
                .arg(divisionName.into())
                .arg(city.into())
                .arg(materialName.into())
                .arg(qty.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn limitProductProduction<
            A: Into<crate::String>,
            B: Into<crate::Any>,
            C: Into<crate::String>,
            D: Into<crate::Number>,
        >(
            &self,
            divisionName: A,
            city: B,
            productName: C,
            qty: D,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let limitProductProduction: crate::Function = self
                .internal
                .get("limitProductProduction")?;
            let result = limitProductProduction
                .arg(divisionName.into())
                .arg(city.into())
                .arg(productName.into())
                .arg(qty.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getUpgradeWarehouseCost<
            A: Into<crate::String>,
            B: Into<crate::Any>,
            C: Into<crate::Number>,
        >(
            &self,
            divisionName: A,
            city: B,
            amt: C,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getUpgradeWarehouseCost: crate::Function = self
                .internal
                .get("getUpgradeWarehouseCost")?;
            let result = getUpgradeWarehouseCost
                .arg(divisionName.into())
                .arg(city.into())
                .arg(amt.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn hasWarehouse<A: Into<crate::String>, B: Into<crate::Any>>(
            &self,
            divisionName: A,
            city: B,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let hasWarehouse: crate::Function = self.internal.get("hasWarehouse")?;
            let result = hasWarehouse.arg(divisionName.into()).arg(city.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
    }
    pub enum CreatingCorporationCheckResult {
        Success,
        NoSf3OrDisabled,
        CorporationExists,
        UseSeedMoneyOutsideBN3,
        DisabledBySoftCap,
    }
    impl CreatingCorporationCheckResult {
        fn as_any(&self) -> crate::Any {
            match self {
                CreatingCorporationCheckResult::Success => "Success".into(),
                CreatingCorporationCheckResult::NoSf3OrDisabled => {
                    "NoSf3OrDisabled".into()
                }
                CreatingCorporationCheckResult::CorporationExists => {
                    "CorporationExists".into()
                }
                CreatingCorporationCheckResult::UseSeedMoneyOutsideBN3 => {
                    "UseSeedMoneyOutsideBN3".into()
                }
                CreatingCorporationCheckResult::DisabledBySoftCap => {
                    "DisabledBySoftCap".into()
                }
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    impl From<CreatingCorporationCheckResult> for crate::Any {
        fn from(value: CreatingCorporationCheckResult) -> crate::Any {
            match value {
                CreatingCorporationCheckResult::Success => "Success".into(),
                CreatingCorporationCheckResult::NoSf3OrDisabled => {
                    "NoSf3OrDisabled".into()
                }
                CreatingCorporationCheckResult::CorporationExists => {
                    "CorporationExists".into()
                }
                CreatingCorporationCheckResult::UseSeedMoneyOutsideBN3 => {
                    "UseSeedMoneyOutsideBN3".into()
                }
                CreatingCorporationCheckResult::DisabledBySoftCap => {
                    "DisabledBySoftCap".into()
                }
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("This variant can not be converted to a string"),
                    );
                }
            }
        }
    }
    pub struct Corporation {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for Corporation {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<Corporation, Self::Error> {
            Ok(Corporation {
                internal: Object::from(value),
            })
        }
    }
    impl Corporation {
        pub fn hasCorporation(&self) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let hasCorporation: crate::Function = self.internal.get("hasCorporation")?;
            let result = hasCorporation.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn canCreateCorporation<A: Into<crate::Boolean>>(
            &self,
            selfFund: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let canCreateCorporation: crate::Function = self
                .internal
                .get("canCreateCorporation")?;
            let result = canCreateCorporation.arg(selfFund.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn createCorporation<A: Into<crate::String>, B: Into<crate::Boolean>>(
            &self,
            corporationName: A,
            selfFund: B,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let createCorporation: crate::Function = self
                .internal
                .get("createCorporation")?;
            let result = createCorporation
                .arg(corporationName.into())
                .arg(selfFund.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn hasUnlock<A: Into<crate::String>>(
            &self,
            upgradeName: A,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let hasUnlock: crate::Function = self.internal.get("hasUnlock")?;
            let result = hasUnlock.arg(upgradeName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getUnlockCost<A: Into<crate::String>>(
            &self,
            upgradeName: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getUnlockCost: crate::Function = self.internal.get("getUnlockCost")?;
            let result = getUnlockCost.arg(upgradeName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getUpgradeLevel<A: Into<crate::String>>(
            &self,
            upgradeName: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getUpgradeLevel: crate::Function = self.internal.get("getUpgradeLevel")?;
            let result = getUpgradeLevel.arg(upgradeName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getUpgradeLevelCost<A: Into<crate::String>>(
            &self,
            upgradeName: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getUpgradeLevelCost: crate::Function = self
                .internal
                .get("getUpgradeLevelCost")?;
            let result = getUpgradeLevelCost.arg(upgradeName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getInvestmentOffer(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getInvestmentOffer: crate::Function = self
                .internal
                .get("getInvestmentOffer")?;
            let result = getInvestmentOffer.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getConstants(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getConstants: crate::Function = self.internal.get("getConstants")?;
            let result = getConstants.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getIndustryData<A: Into<crate::Any>>(
            &self,
            industryName: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getIndustryData: crate::Function = self.internal.get("getIndustryData")?;
            let result = getIndustryData.arg(industryName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getMaterialData<A: Into<crate::Any>>(
            &self,
            materialName: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getMaterialData: crate::Function = self.internal.get("getMaterialData")?;
            let result = getMaterialData.arg(materialName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn acceptInvestmentOffer(
            &self,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let acceptInvestmentOffer: crate::Function = self
                .internal
                .get("acceptInvestmentOffer")?;
            let result = acceptInvestmentOffer.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn goPublic<A: Into<crate::Number>>(
            &self,
            numShares: A,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let goPublic: crate::Function = self.internal.get("goPublic")?;
            let result = goPublic.arg(numShares.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn bribe<A: Into<crate::String>, B: Into<crate::Number>>(
            &self,
            factionName: A,
            amountCash: B,
        ) -> Result<crate::Boolean, wasm_bindgen::JsValue> {
            let bribe: crate::Function = self.internal.get("bribe")?;
            let result = bribe.arg(factionName.into()).arg(amountCash.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getCorporation(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getCorporation: crate::Function = self.internal.get("getCorporation")?;
            let result = getCorporation.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getDivision<A: Into<crate::String>>(
            &self,
            divisionName: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let getDivision: crate::Function = self.internal.get("getDivision")?;
            let result = getDivision.arg(divisionName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn expandIndustry<A: Into<crate::Any>, B: Into<crate::String>>(
            &self,
            industryType: A,
            divisionName: B,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let expandIndustry: crate::Function = self.internal.get("expandIndustry")?;
            let result = expandIndustry
                .arg(industryType.into())
                .arg(divisionName.into())
                .call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn expandCity<A: Into<crate::String>, B: Into<crate::Any>>(
            &self,
            divisionName: A,
            city: B,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let expandCity: crate::Function = self.internal.get("expandCity")?;
            let result = expandCity.arg(divisionName.into()).arg(city.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn purchaseUnlock<A: Into<crate::String>>(
            &self,
            upgradeName: A,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let purchaseUnlock: crate::Function = self.internal.get("purchaseUnlock")?;
            let result = purchaseUnlock.arg(upgradeName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn levelUpgrade<A: Into<crate::String>>(
            &self,
            upgradeName: A,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let levelUpgrade: crate::Function = self.internal.get("levelUpgrade")?;
            let result = levelUpgrade.arg(upgradeName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn issueDividends<A: Into<crate::Number>>(
            &self,
            rate: A,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let issueDividends: crate::Function = self.internal.get("issueDividends")?;
            let result = issueDividends.arg(rate.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn issueNewShares<A: Into<crate::Number>>(
            &self,
            amount: A,
        ) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let issueNewShares: crate::Function = self.internal.get("issueNewShares")?;
            let result = issueNewShares.arg(amount.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn buyBackShares<A: Into<crate::Number>>(
            &self,
            amount: A,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let buyBackShares: crate::Function = self.internal.get("buyBackShares")?;
            let result = buyBackShares.arg(amount.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn sellShares<A: Into<crate::Number>>(
            &self,
            amount: A,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let sellShares: crate::Function = self.internal.get("sellShares")?;
            let result = sellShares.arg(amount.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn getBonusTime(&self) -> Result<crate::Number, wasm_bindgen::JsValue> {
            let getBonusTime: crate::Function = self.internal.get("getBonusTime")?;
            let result = getBonusTime.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn nextUpdate(&self) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let nextUpdate: crate::Function = self.internal.get("nextUpdate")?;
            let result = nextUpdate.call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
        pub fn sellDivision<A: Into<crate::String>>(
            &self,
            divisionName: A,
        ) -> Result<crate::Undefined, wasm_bindgen::JsValue> {
            let sellDivision: crate::Function = self.internal.get("sellDivision")?;
            let result = sellDivision.arg(divisionName.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
    }
    pub struct CorpProductData {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for CorpProductData {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<CorpProductData, Self::Error> {
            Ok(CorpProductData {
                internal: Object::from(value),
            })
        }
    }
    impl CorpProductData {}
    pub struct CorpIndustryData {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for CorpIndustryData {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<CorpIndustryData, Self::Error> {
            Ok(CorpIndustryData {
                internal: Object::from(value),
            })
        }
    }
    impl CorpIndustryData {}
    pub struct CorporationInfo {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for CorporationInfo {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<CorporationInfo, Self::Error> {
            Ok(CorporationInfo {
                internal: Object::from(value),
            })
        }
    }
    impl CorporationInfo {}
    pub struct CorpConstants {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for CorpConstants {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<CorpConstants, Self::Error> {
            Ok(CorpConstants {
                internal: Object::from(value),
            })
        }
    }
    impl CorpConstants {}
    struct CorpStateName {}
    struct CorpMaterialName {}
    struct CorpUnlockName {}
    struct CorpUpgradeName {}
    struct CorpResearchName {}
    pub struct CorpMaterialConstantData {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for CorpMaterialConstantData {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<CorpMaterialConstantData, Self::Error> {
            Ok(CorpMaterialConstantData {
                internal: Object::from(value),
            })
        }
    }
    impl CorpMaterialConstantData {}
    pub struct IndustryData {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for IndustryData {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<IndustryData, Self::Error> {
            Ok(IndustryData {
                internal: Object::from(value),
            })
        }
    }
    impl IndustryData {}
    pub struct Product {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for Product {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<Product, Self::Error> {
            Ok(Product {
                internal: Object::from(value),
            })
        }
    }
    impl Product {}
    pub struct Material {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for Material {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<Material, Self::Error> {
            Ok(Material {
                internal: Object::from(value),
            })
        }
    }
    impl Material {}
    pub struct Export {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for Export {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<Export, Self::Error> {
            Ok(Export {
                internal: Object::from(value),
            })
        }
    }
    impl Export {}
    pub struct Warehouse {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for Warehouse {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<Warehouse, Self::Error> {
            Ok(Warehouse {
                internal: Object::from(value),
            })
        }
    }
    impl Warehouse {}
    pub struct Office {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for Office {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<Office, Self::Error> {
            Ok(Office {
                internal: Object::from(value),
            })
        }
    }
    impl Office {}
    pub struct Division {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for Division {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<Division, Self::Error> {
            Ok(Division {
                internal: Object::from(value),
            })
        }
    }
    impl Division {}
    pub struct InvestmentOffer {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for InvestmentOffer {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<InvestmentOffer, Self::Error> {
            Ok(InvestmentOffer {
                internal: Object::from(value),
            })
        }
    }
    impl InvestmentOffer {}
    pub struct UserInterfaceTheme {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for UserInterfaceTheme {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<UserInterfaceTheme, Self::Error> {
            Ok(UserInterfaceTheme {
                internal: Object::from(value),
            })
        }
    }
    impl UserInterfaceTheme {}
    pub struct IStyleSettings {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for IStyleSettings {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<IStyleSettings, Self::Error> {
            Ok(IStyleSettings {
                internal: Object::from(value),
            })
        }
    }
    impl IStyleSettings {}
    pub struct GameInfo {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for GameInfo {
        type Error = wasm_bindgen::JsValue;
        fn try_from(value: wasm_bindgen::JsValue) -> Result<GameInfo, Self::Error> {
            Ok(GameInfo {
                internal: Object::from(value),
            })
        }
    }
    impl GameInfo {}
    pub struct AutocompleteData {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for AutocompleteData {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<AutocompleteData, Self::Error> {
            Ok(AutocompleteData {
                internal: Object::from(value),
            })
        }
    }
    impl AutocompleteData {
        pub fn flags<A: Into<crate::Any>>(
            &self,
            schema: A,
        ) -> Result<crate::Any, wasm_bindgen::JsValue> {
            let flags: crate::Function = self.internal.get("flags")?;
            let result = flags.arg(schema.into()).call()?;
            result.try_into().map_err(|err| Into::<crate::Any>::into(err).value)
        }
    }
    pub struct MoneyRequirement {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for MoneyRequirement {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<MoneyRequirement, Self::Error> {
            Ok(MoneyRequirement {
                internal: Object::from(value),
            })
        }
    }
    impl MoneyRequirement {}
    pub struct SkillRequirement {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for SkillRequirement {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<SkillRequirement, Self::Error> {
            Ok(SkillRequirement {
                internal: Object::from(value),
            })
        }
    }
    impl SkillRequirement {}
    pub struct KarmaRequirement {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for KarmaRequirement {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<KarmaRequirement, Self::Error> {
            Ok(KarmaRequirement {
                internal: Object::from(value),
            })
        }
    }
    impl KarmaRequirement {}
    pub struct PeopleKilledRequirement {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for PeopleKilledRequirement {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<PeopleKilledRequirement, Self::Error> {
            Ok(PeopleKilledRequirement {
                internal: Object::from(value),
            })
        }
    }
    impl PeopleKilledRequirement {}
    pub struct FileRequirement {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for FileRequirement {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<FileRequirement, Self::Error> {
            Ok(FileRequirement {
                internal: Object::from(value),
            })
        }
    }
    impl FileRequirement {}
    pub struct NumAugmentationsRequirement {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for NumAugmentationsRequirement {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<NumAugmentationsRequirement, Self::Error> {
            Ok(NumAugmentationsRequirement {
                internal: Object::from(value),
            })
        }
    }
    impl NumAugmentationsRequirement {}
    pub struct EmployedByRequirement {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for EmployedByRequirement {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<EmployedByRequirement, Self::Error> {
            Ok(EmployedByRequirement {
                internal: Object::from(value),
            })
        }
    }
    impl EmployedByRequirement {}
    pub struct CompanyReputationRequirement {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for CompanyReputationRequirement {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<CompanyReputationRequirement, Self::Error> {
            Ok(CompanyReputationRequirement {
                internal: Object::from(value),
            })
        }
    }
    impl CompanyReputationRequirement {}
    pub struct JobTitleRequirement {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for JobTitleRequirement {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<JobTitleRequirement, Self::Error> {
            Ok(JobTitleRequirement {
                internal: Object::from(value),
            })
        }
    }
    impl JobTitleRequirement {}
    pub struct CityRequirement {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for CityRequirement {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<CityRequirement, Self::Error> {
            Ok(CityRequirement {
                internal: Object::from(value),
            })
        }
    }
    impl CityRequirement {}
    pub struct LocationRequirement {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for LocationRequirement {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<LocationRequirement, Self::Error> {
            Ok(LocationRequirement {
                internal: Object::from(value),
            })
        }
    }
    impl LocationRequirement {}
    pub struct BackdoorRequirement {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for BackdoorRequirement {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<BackdoorRequirement, Self::Error> {
            Ok(BackdoorRequirement {
                internal: Object::from(value),
            })
        }
    }
    impl BackdoorRequirement {}
    pub struct HacknetRAMRequirement {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for HacknetRAMRequirement {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<HacknetRAMRequirement, Self::Error> {
            Ok(HacknetRAMRequirement {
                internal: Object::from(value),
            })
        }
    }
    impl HacknetRAMRequirement {}
    pub struct HacknetCoresRequirement {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for HacknetCoresRequirement {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<HacknetCoresRequirement, Self::Error> {
            Ok(HacknetCoresRequirement {
                internal: Object::from(value),
            })
        }
    }
    impl HacknetCoresRequirement {}
    pub struct HacknetLevelsRequirement {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for HacknetLevelsRequirement {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<HacknetLevelsRequirement, Self::Error> {
            Ok(HacknetLevelsRequirement {
                internal: Object::from(value),
            })
        }
    }
    impl HacknetLevelsRequirement {}
    pub struct BitNodeRequirement {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for BitNodeRequirement {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<BitNodeRequirement, Self::Error> {
            Ok(BitNodeRequirement {
                internal: Object::from(value),
            })
        }
    }
    impl BitNodeRequirement {}
    pub struct SourceFileRequirement {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for SourceFileRequirement {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<SourceFileRequirement, Self::Error> {
            Ok(SourceFileRequirement {
                internal: Object::from(value),
            })
        }
    }
    impl SourceFileRequirement {}
    pub struct BladeburnerRankRequirement {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for BladeburnerRankRequirement {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<BladeburnerRankRequirement, Self::Error> {
            Ok(BladeburnerRankRequirement {
                internal: Object::from(value),
            })
        }
    }
    impl BladeburnerRankRequirement {}
    pub struct NumInfiltrationsRequirement {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for NumInfiltrationsRequirement {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<NumInfiltrationsRequirement, Self::Error> {
            Ok(NumInfiltrationsRequirement {
                internal: Object::from(value),
            })
        }
    }
    impl NumInfiltrationsRequirement {}
    pub struct NotRequirement {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for NotRequirement {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<NotRequirement, Self::Error> {
            Ok(NotRequirement {
                internal: Object::from(value),
            })
        }
    }
    impl NotRequirement {}
    pub struct SomeRequirement {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for SomeRequirement {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<SomeRequirement, Self::Error> {
            Ok(SomeRequirement {
                internal: Object::from(value),
            })
        }
    }
    impl SomeRequirement {}
    pub struct EveryRequirement {
        internal: crate::Object,
    }
    impl TryFrom<wasm_bindgen::JsValue> for EveryRequirement {
        type Error = wasm_bindgen::JsValue;
        fn try_from(
            value: wasm_bindgen::JsValue,
        ) -> Result<EveryRequirement, Self::Error> {
            Ok(EveryRequirement {
                internal: Object::from(value),
            })
        }
    }
    impl EveryRequirement {}
    struct PlayerRequirement {}
}
