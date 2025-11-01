use proc_macro::{Span, TokenStream};
use proc_macro_error::{abort, proc_macro_error};
use swc_common::{FileName, SourceMap};
use swc_ecma_ast::Decl;
use swc_ecma_parser::{Parser, StringInput, Syntax, lexer::Lexer};
use syn::parse_macro_input;
mod transform;
use transform::{declaration_to_struct_token_stream, error::Error};

/// Use this to expose your function to bitburner
#[proc_macro_attribute]
pub fn bb_bindgen(_: TokenStream, input: TokenStream) -> TokenStream {
    let mut body = parse_macro_input!(input as syn::ItemFn);

    let og_ident = body.sig.ident.clone();
    let fn_ident = format!("__bb_bind_{}", body.sig.ident);
    body.sig.ident = syn::Ident::new(fn_ident.as_str(), Span::call_site().into());
    let fn_ident = body.sig.ident.clone();

    quote::quote! {
        #[wasm_bindgen::prelude::wasm_bindgen]
        pub async fn #og_ident(val:JsValue) -> Result<(), wasm_bindgen::JsValue>{
            let ns = Box::leak(Box::new(bitburner_bindings::NS::try_from(val.clone())?));

            std::panic::set_hook(Box::new(|err| {
                bitburner_bindings::log_error(err.to_string().into())
            }));

            let ret = #fn_ident(ns).await;

            drop(std::panic::take_hook());

            //This is unsafe af. Its so easy to write code that uses NS after cleanup
            //idc tho, better cause errors than memory leaks
            ns.atExit(bitburner_bindings::js_closure!(|_: Any| -> Result<Any, JsValue> {
                let ns = unsafe {Box::from_raw(ns as *const NS as *mut NS)};
                drop(ns);
                Ok(bitburner_bindings::Any::from(
                    wasm_bindgen::JsValue::undefined(),
                ))
            }),bitburner_bindings::v4uuid())?;

            ret
        }

        #body
    }
    .into()
}

/// Generates WASM glue for an API defined with a d.ts file
/// # Example
/// ```
/// use bitburner_bindings_macros::from_dts;
///
/// from_dts!("./path/to/api.d.ts")
/// ```
#[proc_macro_error]
#[proc_macro]
pub fn from_dts(input: TokenStream) -> TokenStream {
    let path = parse_macro_input!(input as syn::LitStr).value();

    let source_code = std::fs::read_to_string(&path).expect("File not found");
    let source_map: SourceMap = Default::default();
    let source_file = source_map.new_source_file(FileName::Custom(path).into(), source_code.into());

    let lexer = Lexer::new(
        Syntax::Typescript(Default::default()),
        swc_ecma_ast::EsVersion::Es2024,
        StringInput::from(&*source_file),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    let module = match parser.parse_typescript_module() {
        Ok(module) => module,
        Err(err) => abort!(Error::ts_syntax(err, &source_map)),
    };

    let structs_stream: TokenStream = module
        .body
        .iter()
        .filter_map(|item| match item.is_stmt() {
            true => item.as_stmt().and_then(|item| item.as_decl()),
            false => item
                .as_module_decl()
                .and_then(|item| item.as_export_decl())
                .and_then(|item| Some(&item.decl)),
        })
        .filter_map(|decl| match decl {
            Decl::Using(_) => None,
            Decl::TsModule(_) => None,
            node => Some(declaration_to_struct_token_stream(&node, &source_map)),
        })
        .collect();

    structs_stream
}
