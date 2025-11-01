use itertools::Itertools;
use proc_macro_error::{Diagnostic, Level};
use swc_common::{CharPos, SourceMap, Span, Spanned};

#[derive(Debug)]
pub enum ErrorType {
    Unsupported,
    Syn,
    FuckYou,
    TSSyntax,
}

#[derive(Debug)]
pub struct Error {
    ty: ErrorType,
    message: String,
}

impl Error {
    pub fn ty(&self) -> &ErrorType {
        return &self.ty;
    }

    pub fn message(&self) -> &String {
        return &self.message;
    }

    fn create_message(msg: String, span: Span, cm: &SourceMap) -> String {
        let file_name = cm.span_to_filename(span);
        let source_file = cm.get_source_file(&file_name).unwrap();
        let line = source_file.lookup_line(span.hi).unwrap();

        let text = source_file
            .get_line(line)
            .map(std::borrow::Cow::into_owned)
            .unwrap_or(format!("<no source available>"));

        let CharPos(col_start) = cm.lookup_char_pos(span.lo).col;
        let CharPos(col_end) = cm.lookup_char_pos(span.hi).col;

        let underline = format!(
            "{}{}{}",
            (0..col_start).map(|_| " ").join(""),
            (col_start..col_end).map(|_| "^").join(""),
            (col_end..text.len()).map(|_| " ").join("")
        );

        format!(
            "{}\nFile: {}\nLine: {}\n\n{}\n{}\n\n",
            msg, file_name, line, text, underline
        )
    }

    pub fn unsupported(feat: &str, span: Span, cm: &SourceMap) -> Self {
        Self {
            message: Self::create_message(format!("Unsupported feature: {}", feat), span, cm),
            ty: ErrorType::Unsupported,
        }
    }

    pub fn fuck_you(feat: &str, span: Span, cm: &SourceMap) -> Self {
        Self {
            message: Self::create_message(format!("Fuck You: {}", feat), span, cm),
            ty: ErrorType::FuckYou,
        }
    }

    pub fn raw(message: String, ty:ErrorType) -> Self {
        Self {
            message,
            ty,
        }
    }

    pub fn ts_syntax(err: swc_ecma_parser::error::Error, cm: &SourceMap) -> Self {
        Self {
            message: Self::create_message(format!("TS Syntax Error:"), err.span(), cm),
            ty: ErrorType::TSSyntax,
        }
    }

    pub fn syn(err: syn::Error) -> Self {
        Self {
            message: format!("SYN:{}", err),
            ty: ErrorType::Syn,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<Error> for Diagnostic {
    fn from(value: Error) -> Self {
        Diagnostic::new(Level::Error, value.into())
    }
}

impl From<&Error> for Diagnostic {
    fn from(value: &Error) -> Self {
        Diagnostic::new(Level::Error, value.message.clone())
    }
}

impl From<Error> for String {
    fn from(value: Error) -> Self {
        value.message
    }
}

impl From<&Error> for String {
    fn from(value: &Error) -> Self {
        value.message.clone()
    }
}

pub type TransformResult<T> = std::result::Result<T, Error>;
