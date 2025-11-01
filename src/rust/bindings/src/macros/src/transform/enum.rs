use proc_macro_error::abort;
use proc_macro_error::emit_call_site_warning;
use swc_common::SourceMap;
use swc_ecma_ast::{TsEnumDecl, TsEnumMember};

use super::{
    expression::ts_expression_to_expression, parse_quote, parse_string, safe_convert_ident,
};

/// Converts a TypeScript enum member into a Rust enum variant and an optional match arm.
///
/// This function takes an identifier, a TypeScript enum member AST node, and a SourceMap reference.
/// It returns a tuple containing:
/// - A `syn::Variant` representing the Rust enum variant.
/// - An optional `syn::Arm` that matches the enum variant to a string representation.
///
/// # Arguments
///
/// * `ident` - The identifier for the enum.
/// * `member` - The TypeScript enum member AST node.
/// * `_cm` - The SourceMap for the current context (unused in this function).
///
/// # Returns
///
/// A tuple containing:
/// - A `syn::Variant` representing the Rust enum variant.
/// - An optional `syn::Arm` for pattern matching the variant to a string.
///
/// # Example
///
/// ```rust
/// use syn::{Ident, Variant, Arm};
/// use some_ts_parser::{TsEnumMember, SourceMap};
/// use bitburner_binding_macro::enum_member_to_match_arm;
///
/// let ident = Ident::new("MyEnum", Span::call_site());
/// let member = TsEnumMember::from_string("MyValue");
/// let cm = SourceMap::new();
///
/// let (variant, match_arm) = enum_member_to_match_arm(&ident, &member, &cm);
///
/// // Use the returned variant and match arm in further code generation
/// ```
fn enum_member_to_match_arm(
    ident: &syn::Ident,
    member: &TsEnumMember,
    cm: &SourceMap,
) -> (syn::Variant, Option<syn::Arm>) {
    let str_ident = match &member.id {
        swc_ecma_ast::TsEnumMemberId::Ident(ident) => ident.sym.as_str(),
        swc_ecma_ast::TsEnumMemberId::Str(str) => str.value.as_str(),
    };

    let variant = match parse_string!(str_ident => syn::Variant) {
        Ok(ok) => ok,
        Err(err) => abort!(err),
    };

    let arm = member.init.as_ref().map(Box::as_ref).and_then(|expr| {
        ts_expression_to_expression(expr, cm)
            .and_then(|expr| parse_quote!({#ident::#variant => #expr} as syn::Arm))
            .inspect_err(|e| emit_call_site_warning!(e))
            .ok()
    });

    (variant, arm)
}

pub fn ts_enum_to_token_stream(decl: &TsEnumDecl, cm: &SourceMap) -> proc_macro::TokenStream {
    let ident: syn::Ident = safe_convert_ident(&decl.id, cm);

    let (variants, match_arms): (Vec<syn::Variant>, Vec<Option<syn::Arm>>) = decl
        .members
        .iter()
        .map(|m| enum_member_to_match_arm(&ident, m, cm))
        .unzip();

    let match_arms = match_arms.into_iter().collect::<Option<Vec<_>>>();

    let mut declaration = quote::quote! {
      pub enum #ident {
        #(#variants),*
      }
    };

    if let Some(match_arms) = match_arms {
        declaration.extend(quote::quote! {
            impl #ident {
                fn as_any(&self) -> crate::Any {
                    match self {
                        #(#match_arms),*,
                        _ => panic!("This variant can not be converted to a string"),
                    }
                }
              }

            impl From<#ident> for crate::Any{
                fn from(value:#ident) -> crate::Any {
                    match value {
                        #(#match_arms),*,
                        _ => panic!("This variant can not be converted to a string"),
                    }
                }
            }
        });
    }

    declaration.into()
}
