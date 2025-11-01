use proc_macro_error::emit_error;
use swc_common::SourceMap;
use swc_ecma_ast::{
    TsCallSignatureDecl, TsConstructSignatureDecl, TsGetterSignature, TsIndexSignature,
    TsInterfaceDecl, TsMethodSignature, TsPropertySignature, TsSetterSignature, TsTypeElement,
};

use super::{function::method_signature_to_impl_item_fn, rename_impl_item_function};

struct TypeElements<'a> {
    constructors: Vec<&'a TsConstructSignatureDecl>,
    getters: Vec<&'a TsGetterSignature>,
    setters: Vec<&'a TsSetterSignature>,
    props: Vec<&'a TsPropertySignature>,
    index_props: Vec<&'a TsIndexSignature>,
    methods: Vec<&'a TsMethodSignature>,
    call: Vec<&'a TsCallSignatureDecl>,
}

impl TypeElements<'_> {
    fn new() -> Self {
        TypeElements {
            constructors: vec![],
            getters: vec![],
            setters: vec![],
            props: vec![],
            index_props: vec![],
            methods: vec![],
            call: vec![],
        }
    }
}

pub fn interface_to_token_stream(
    decl: &TsInterfaceDecl,
    cm: &SourceMap,
) -> proc_macro::TokenStream {
    let ident: syn::Ident = syn::parse_str(&format!("{}{}", "", decl.id.sym.as_str())).expect("");
    let TypeElements { methods, .. } =
        decl.body
            .body
            .iter()
            .fold(TypeElements::new(), |mut prev, m| {
                match m {
                    TsTypeElement::TsCallSignatureDecl(node) => prev.call.push(node),
                    TsTypeElement::TsConstructSignatureDecl(node) => prev.constructors.push(node),
                    TsTypeElement::TsPropertySignature(node) => prev.props.push(node),
                    TsTypeElement::TsGetterSignature(node) => prev.getters.push(node),
                    TsTypeElement::TsSetterSignature(node) => prev.setters.push(node),
                    TsTypeElement::TsMethodSignature(node) => prev.methods.push(node),
                    TsTypeElement::TsIndexSignature(node) => prev.index_props.push(node),
                };
                prev
            });

    //I dont like myself :3
    let methods: Vec<_> = methods
        .iter()
        .map(|sig| method_signature_to_impl_item_fn(sig, &cm))
        .filter_map(|res| res.inspect_err(|e| emit_error!(e)).ok())
        .collect::<Vec<syn::ImplItemFn>>()
        .chunk_by(|prev, cur| prev.sig.ident.to_string() == cur.sig.ident.to_string())
        .flat_map(|methods| match methods.len() {
            0 => panic!("WHAT?? D:"),
            1 => methods.to_vec(),
            _ => methods
                .to_vec()
                .into_iter()
                .enumerate()
                .map(|(i, method)| {
                    rename_impl_item_function(
                        &format!("{}{}", method.sig.ident.to_string(), i), // ""
                        method,
                    )
                })
                .collect(),
        })
        .collect();

    quote::quote! {
        pub struct #ident{
            internal: crate::Object
        }
        impl TryFrom<wasm_bindgen::JsValue> for #ident {
            type Error = wasm_bindgen::JsValue;

            fn try_from(value: wasm_bindgen::JsValue) -> Result<#ident, Self::Error> {
                Ok(#ident { internal: Object::from(value) })
            }
        }
        impl #ident {
            #(#methods)*
        }
    }
    .into()
}
