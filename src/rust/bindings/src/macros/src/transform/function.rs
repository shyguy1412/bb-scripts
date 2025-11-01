use std::char;

use itertools::Itertools;
use proc_macro_error::{emit_call_site_warning, emit_error};
use swc_common::SourceMap;
use swc_ecma_ast::{ArrayPat, BindingIdent, ObjectPat, RestPat, TsFnParam, TsMethodSignature};

use crate::transform::{
    parse_quote, parse_string, safe_convert_ident,
    r#type::type_annotation_to_type,
};

use super::error::{Error, TransformResult};

pub fn ts_fn_param_to_arg(param: &TsFnParam, cm: &SourceMap) -> TransformResult<syn::FnArg> {
    match param {
        TsFnParam::Ident(node) => binding_ident_to_arg(node, cm),
        TsFnParam::Array(node) => array_arg_to_arg(node, cm),
        TsFnParam::Rest(node) => rest_arg_to_arg(node, cm),
        TsFnParam::Object(node) => object_arg_to_arg(node, cm),
    }
}

pub fn array_arg_to_arg(arg: &ArrayPat, cm: &SourceMap) -> TransformResult<syn::FnArg> {
    Err(Error::unsupported("array argument", arg.span, cm))
}
pub fn rest_arg_to_arg(arg: &RestPat, cm: &SourceMap) -> TransformResult<syn::FnArg> {
    match arg.arg.as_ref() {
        swc_ecma_ast::Pat::Ident(node) => binding_ident_to_arg(node, &cm),
        _ => Err(Error::fuck_you(
            "This shouldnt even be valid syntax?",
            arg.span,
            cm,
        )),
    }
}
pub fn object_arg_to_arg(arg: &ObjectPat, cm: &SourceMap) -> TransformResult<syn::FnArg> {
    Err(Error::unsupported("object argument", arg.span, cm))
}

pub fn binding_ident_to_arg(
    binding_ident: &BindingIdent,
    cm: &SourceMap,
) -> TransformResult<syn::FnArg> {
    let arg_type = binding_ident
        .type_ann
        .as_ref()
        .map(|type_ann| type_annotation_to_type(&type_ann, cm))
        .transpose()
        .inspect_err(|err| emit_call_site_warning!(err))
        .ok()
        .flatten()
        .unwrap_or(parse_quote!({ crate::Any } as syn::Type).expect("Guranteed by argument"));

    let ident = safe_convert_ident(&binding_ident.id, cm);
    parse_quote!({#ident:#arg_type} as syn::FnArg)
}

pub fn method_signature_to_impl_item_fn(
    method: &TsMethodSignature,
    cm: &SourceMap,
) -> TransformResult<syn::ImplItemFn> {
    let ident = method
        .key
        .as_ident()
        .map(|ident| safe_convert_ident(ident, cm))
        .ok_or(Error::unsupported("Computed method name", method.span, cm))?;

    let ident_str =
        parse_string!(&format!("\"{}\"", ident) => syn::LitStr).expect("Guranteed by arguments");

    let (args, errors): (Vec<_>, Vec<Error>) = method
        .params
        .iter()
        .map(|param| ts_fn_param_to_arg(param, cm))
        .filter_map_ok(|arg| match arg {
            syn::FnArg::Receiver(_) => None,
            syn::FnArg::Typed(arg) => Some((*arg.pat, *arg.ty)),
        })
        .partition_result();

    if errors.len() != 0 {
        errors.into_iter().for_each(|e| emit_error!(e));
    }

    let (arg_idents, arg_types): (Vec<_>, Vec<_>) = args.into_iter().unzip();

    let (generics, arg_types): (Vec<_>, Vec<_>) = (0u32..)
        .zip(arg_types.into_iter())
        .map(|(i, ty)| {
            let generic = get_generic_name(i + 1);
            let generic = parse_string!(&generic => syn::Ident)?;

            let param = parse_quote!(
                { #generic: Into<#ty> } as syn::GenericParam
            )?;

            let arg = parse_quote!(
                { #generic } as syn::Type
            )?;

            Ok((param, arg))
        })
        .map(|r| r.inspect_err(|e| emit_error!(e)).ok())
        .filter_map(|o| o)
        .collect();

    let return_type = method
        .type_ann
        .as_ref()
        .map(|ann| ann.as_ref())
        .map(|ann| type_annotation_to_type(ann, cm))
        .transpose()
        .inspect_err(|err| emit_call_site_warning!(err))
        .ok()
        .flatten()
        .unwrap_or(parse_quote!({ crate::Any } as syn::Type).expect("Guranteed by Argument"));

    parse_quote!({
        pub fn #ident<#(#generics),*>(&self, #(#arg_idents:#arg_types),*) -> Result<#return_type, wasm_bindgen::JsValue> {
            let #ident: crate::Function = self.internal.get(#ident_str)?;

            let result = #ident #(.arg(Into::<crate::Any>::into(#arg_idents.into())))*.call()?;

            result
                .try_into()
                .map_err(|err| Into::<crate::Any>::into(err).value)
        } 
    } as syn::ImplItemFn)
}

fn get_generic_name(mut i: u32) -> String {
    let mut name: String = String::new();

    while i > 0 {
        let modulo = (i - 1) % 26;
        name.push(char::from_u32(65 + modulo).expect("Guranteed by argument"));
        i = (i - modulo) / 26;
    }

    name.chars().rev().collect()
}
