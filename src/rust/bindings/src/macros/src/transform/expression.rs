use proc_macro_error::emit_error;
use swc_common::{SourceMap, Spanned};
use swc_ecma_ast::{Expr, Lit, MemberExpr, Str};

use super::{
    error::{Error, TransformResult},
    parse_quote, parse_string, safe_convert_ident,
};

pub fn ts_expression_to_expression(expr: &Expr, cm: &SourceMap) -> TransformResult<syn::Expr> {
    match expr {
        Expr::Lit(lit) => literal_to_any_expression(lit, cm),
        Expr::Member(member) => member_to_any_expression(member, cm),
        Expr::Ident(ident) => {
            let ident = safe_convert_ident(ident, cm);
            parse_quote!({#ident} as syn::Expr).inspect_err(|_| {
                emit_error!(Error::fuck_you(&ident.to_string(), expr.span(), cm))
            })
        }
        _ => Err(Error::unsupported("expression", expr.span(), cm)),
    }
}

pub fn literal_to_any_expression(lit: &Lit, cm: &SourceMap) -> TransformResult<syn::Expr> {
    match lit {
        swc_ecma_ast::Lit::Str(str) => string_literal_to_any_expression(str, cm),
        swc_ecma_ast::Lit::Bool(_bool) => todo!("ool"),
        swc_ecma_ast::Lit::Null(_null) => todo!("ull"),
        swc_ecma_ast::Lit::Num(_number) => todo!("um"),
        swc_ecma_ast::Lit::BigInt(_big_int) => todo!("igInt"),
        swc_ecma_ast::Lit::Regex(_regex) => todo!("egex"),
        swc_ecma_ast::Lit::JSXText(_jsxtext) => todo!("SXText"),
    }
}

pub fn string_literal_to_any_expression(str: &Str, _cm: &SourceMap) -> TransformResult<syn::Expr> {
    let value = str.value.as_str();

    parse_quote!({
        #value.into()
    } as syn::Expr)
}

pub fn member_to_any_expression(member: &MemberExpr, cm: &SourceMap) -> TransformResult<syn::Expr> {
    let expr = ts_expression_to_expression(&member.obj, cm).inspect_err(|err| emit_error!(err))?;
    let ident = member
        .prop
        .as_ident()
        .map(|ident| ident.sym.as_str())
        .map(|str| parse_string!(str => syn::Ident))
        .transpose()?
        .ok_or(Error::unsupported(
            "Computed member expression",
            member.span,
            cm,
        ))?;

    parse_quote!({ #expr::#ident.into() } as syn::Expr).inspect_err(|err| emit_error!(err))
}
