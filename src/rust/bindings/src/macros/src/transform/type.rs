use swc_common::SourceMap;
use swc_ecma_ast::{
    TsArrayType, TsConditionalType, TsFnOrConstructorType, TsImportType, TsIndexedAccessType,
    TsInferType, TsKeywordType, TsKeywordTypeKind, TsLit, TsLitType, TsMappedType, TsOptionalType,
    TsParenthesizedType, TsRestType, TsThisType, TsTupleType, TsType, TsTypeAliasDecl, TsTypeAnn,
    TsTypeLit, TsTypeOperator, TsTypePredicate, TsTypeQuery, TsTypeRef, TsUnionOrIntersectionType,
};

use super::{error::TransformResult, safe_convert_ident};
use crate::transform::{error::Error, parse_quote};

pub fn type_alias_to_token_stream(
    decl: &TsTypeAliasDecl,
    cm: &SourceMap,
) -> proc_macro::TokenStream {
    let ident: syn::Ident = safe_convert_ident(&decl.id, cm);

    quote::quote! {
      struct #ident{}
    }
    .into()
}

pub fn type_annotation_to_type(ann: &TsTypeAnn, cm: &SourceMap) -> TransformResult<syn::Type> {
    match &*ann.type_ann {
        TsType::TsKeywordType(node) => keyword_type_to_type(node, cm),
        TsType::TsThisType(node) => this_type_to_type(node, cm),
        TsType::TsFnOrConstructorType(node) => fn_or_constructor_type_to_type(node, cm),
        TsType::TsTypeRef(node) => type_ref_to_type(node, cm),
        TsType::TsTypeQuery(node) => type_query_to_type(node, cm),
        TsType::TsTypeLit(node) => type_lit_to_type(node, cm),
        TsType::TsArrayType(node) => array_type_to_type(node, cm),
        TsType::TsTupleType(node) => tuple_type_to_type(node, cm),
        TsType::TsOptionalType(node) => optional_type_to_type(node, cm),
        TsType::TsRestType(node) => rest_type_to_type(node, cm),
        TsType::TsUnionOrIntersectionType(node) => union_or_intersection_type_to_type(node, cm),
        TsType::TsConditionalType(node) => conditional_type_to_type(node, cm),
        TsType::TsInferType(node) => infer_type_to_type(node, cm),
        TsType::TsParenthesizedType(node) => parenthesized_type_to_type(node, cm),
        TsType::TsTypeOperator(node) => type_operator_to_type(node, cm),
        TsType::TsIndexedAccessType(node) => indexed_access_type_to_type(node, cm),
        TsType::TsMappedType(node) => mapped_type_to_type(node, cm),
        TsType::TsLitType(node) => lit_type_to_type(node, cm),
        TsType::TsTypePredicate(node) => type_predicate_to_type(node, cm),
        TsType::TsImportType(node) => import_type_to_type(node, cm),
    }
}

pub fn keyword_type_to_type(ty: &TsKeywordType, cm: &SourceMap) -> TransformResult<syn::Type> {
    let ty = match &ty.kind {
        TsKeywordTypeKind::TsAnyKeyword => parse_quote!({ crate::Any } as syn::Type),
        TsKeywordTypeKind::TsUnknownKeyword => parse_quote!({ crate::Any } as syn::Type),
        TsKeywordTypeKind::TsNumberKeyword => parse_quote!({ crate::Number } as syn::Type),
        TsKeywordTypeKind::TsObjectKeyword => parse_quote!({ crate::Object } as syn::Type),
        TsKeywordTypeKind::TsBooleanKeyword => parse_quote!({ crate::Boolean } as syn::Type),
        TsKeywordTypeKind::TsBigIntKeyword => parse_quote!({ crate::BigInt } as syn::Type),
        TsKeywordTypeKind::TsStringKeyword => parse_quote!({ crate::String } as syn::Type),
        TsKeywordTypeKind::TsSymbolKeyword => parse_quote!({ crate::Symbol } as syn::Type),
        TsKeywordTypeKind::TsVoidKeyword => parse_quote!({ crate::Undefined } as syn::Type),
        TsKeywordTypeKind::TsUndefinedKeyword => parse_quote!({ crate::Undefined } as syn::Type),
        TsKeywordTypeKind::TsNullKeyword => parse_quote!({ crate::Object } as syn::Type),
        TsKeywordTypeKind::TsNeverKeyword => parse_quote!({ crate::Undefined } as syn::Type),
        TsKeywordTypeKind::TsIntrinsicKeyword => {
            Err(Error::unsupported("Intrinsic keyword type", ty.span, cm))
        }
    }?;
    
    Ok(ty)
}

pub fn this_type_to_type(_ty: &TsThisType, _cm: &SourceMap) -> TransformResult<syn::Type> {
    parse_quote!({ crate::Any } as syn::Type)
}

pub fn fn_or_constructor_type_to_type(
    _ty: &TsFnOrConstructorType,
    _cm: &SourceMap,
) -> TransformResult<syn::Type> {
    parse_quote!({ crate::Any } as syn::Type)
}

pub fn type_ref_to_type(_ty: &TsTypeRef, _cm: &SourceMap) -> TransformResult<syn::Type> {
    parse_quote!({ crate::Any } as syn::Type)
}

pub fn type_query_to_type(_ty: &TsTypeQuery, _cm: &SourceMap) -> TransformResult<syn::Type> {
    parse_quote!({ crate::Any } as syn::Type)
}

pub fn type_lit_to_type(_ty: &TsTypeLit, _cm: &SourceMap) -> TransformResult<syn::Type> {
    parse_quote!({ crate::Any } as syn::Type)
}

pub fn array_type_to_type(_ty: &TsArrayType, _cm: &SourceMap) -> TransformResult<syn::Type> {
    parse_quote!({ crate::Any } as syn::Type)
}

pub fn tuple_type_to_type(_ty: &TsTupleType, _cm: &SourceMap) -> TransformResult<syn::Type> {
    parse_quote!({ crate::Any } as syn::Type)
}

pub fn optional_type_to_type(_ty: &TsOptionalType, _cm: &SourceMap) -> TransformResult<syn::Type> {
    parse_quote!({ crate::Any } as syn::Type)
}

pub fn rest_type_to_type(_ty: &TsRestType, _cm: &SourceMap) -> TransformResult<syn::Type> {
    parse_quote!({ crate::Any } as syn::Type)
}

pub fn union_or_intersection_type_to_type(
    _ty: &TsUnionOrIntersectionType,
    _cm: &SourceMap,
) -> TransformResult<syn::Type> {
    parse_quote!({ crate::Any } as syn::Type)
}

pub fn conditional_type_to_type(
    _ty: &TsConditionalType,
    _cm: &SourceMap,
) -> TransformResult<syn::Type> {
    parse_quote!({ crate::Any } as syn::Type)
}

pub fn infer_type_to_type(_ty: &TsInferType, _cm: &SourceMap) -> TransformResult<syn::Type> {
    parse_quote!({ crate::Any } as syn::Type)
}

pub fn parenthesized_type_to_type(
    _ty: &TsParenthesizedType,
    _cm: &SourceMap,
) -> TransformResult<syn::Type> {
    parse_quote!({ crate::Any } as syn::Type)
}

pub fn type_operator_to_type(_ty: &TsTypeOperator, _cm: &SourceMap) -> TransformResult<syn::Type> {
    parse_quote!({ crate::Any } as syn::Type)
}

pub fn indexed_access_type_to_type(
    _ty: &TsIndexedAccessType,
    _cm: &SourceMap,
) -> TransformResult<syn::Type> {
    parse_quote!({ crate::Any } as syn::Type)
}

pub fn mapped_type_to_type(_ty: &TsMappedType, _cm: &SourceMap) -> TransformResult<syn::Type> {
    parse_quote!({ crate::Any } as syn::Type)
}

pub fn lit_type_to_type(ty: &TsLitType, _cm: &SourceMap) -> TransformResult<syn::Type> {
    let ty = match &ty.lit {
        TsLit::Number(_) => parse_quote!({ crate::Number } as syn::Type),
        TsLit::Str(_) => parse_quote!({ crate::String } as syn::Type),
        TsLit::Bool(_) => parse_quote!({ crate::Boolean } as syn::Type),
        TsLit::BigInt(_) => parse_quote!({ crate::Number } as syn::Type),
        TsLit::Tpl(_) => parse_quote!({ crate::String } as syn::Type),
    }?;

    Ok(ty)
}

pub fn type_predicate_to_type(
    _ty: &TsTypePredicate,
    _cm: &SourceMap,
) -> TransformResult<syn::Type> {
    parse_quote!({ crate::Any } as syn::Type)
}

pub fn import_type_to_type(ty: &TsImportType, cm: &SourceMap) -> TransformResult<syn::Type> {
    Err(Error::unsupported("Import Type", ty.span, cm))
    // parse_quote!({crate::Any} as syn::Type)
}
