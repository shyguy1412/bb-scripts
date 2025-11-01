use proc_macro_error::abort;
use swc_common::SourceMap;
use swc_ecma_ast::Decl;

mod r#enum;
pub use r#enum::ts_enum_to_token_stream;

mod interface;
pub use interface::interface_to_token_stream;

mod r#type;
pub use r#type::type_alias_to_token_stream;

pub mod error;

mod expression;
mod function;

pub fn declaration_to_struct_token_stream(decl: &Decl, cm: &SourceMap) -> proc_macro::TokenStream {
    match decl {
        Decl::TsInterface(decl) => interface_to_token_stream(decl.as_ref(), cm),
        Decl::TsTypeAlias(decl) => type_alias_to_token_stream(decl.as_ref(), cm),
        Decl::TsEnum(decl) => ts_enum_to_token_stream(decl.as_ref(), cm),
        _ => proc_macro::TokenStream::new(),
    }
}

/**
 * Safely converts TS ident to Rust Ident by prefixing with _ if the ident is a reserved keyword in rust
 */
pub(self) fn safe_convert_ident(ident: &swc_ecma_ast::Ident, _cm: &SourceMap) -> syn::Ident {
    let ident_str = ident.sym.as_str();
    let result = parse_string!(ident_str => syn::Ident)
        .or_else(|_| parse_string!(&format!("_{}", ident_str) => syn::Ident));

    match result {
        Ok(val) => val,
        Err(err) => abort!(err),
    }
}
macro_rules! parse_string {
    ($expr:expr => $t:ty) => {
        syn::parse_str::<$t>($expr).map_err(|e| crate::transform::error::Error::syn(e))
    };
}
pub(crate) use parse_string;

macro_rules! parse_quote {
    ({$($tt:tt)*} as $t:ty) => {
        syn::parse::<$t>(quote::quote!{$($tt)*}.into()).map_err(|e|crate::transform::error::Error::syn(e))
    };
}
pub(crate) use parse_quote;

pub fn rename_impl_item_function(ident:&str, func: syn::ImplItemFn) -> syn::ImplItemFn {
    syn::ImplItemFn {
        attrs: func.attrs,
        vis: func.vis,
        defaultness: func.defaultness,
        sig: syn::Signature {
            constness: func.sig.constness,
            asyncness: func.sig.asyncness,
            unsafety: func.sig.unsafety,
            abi: func.sig.abi,
            fn_token: func.sig.fn_token,
            ident: syn::Ident::new(
                ident,
                func.sig.ident.span(),
            ),
            generics: func.sig.generics,
            paren_token: func.sig.paren_token,
            inputs: func.sig.inputs,
            variadic: func.sig.variadic,
            output: func.sig.output,
        },
        block: func.block,
    }
}
