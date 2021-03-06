use derive_more::Display;

use yarte_parser::{source_map::Span, ErrorMessage};

pub type GResult<T> = Result<T, GError>;

pub(crate) struct MiddleError {
    message: GError,
    range: (u32, u32),
    span: Span,
}

impl MiddleError {
    pub(crate) fn new(message: GError, range: (usize, usize), span: Span) -> Self {
        let range = (range.0 as u32, range.1 as u32);
        Self {
            message,
            range,
            span,
        }
    }
}

impl Into<ErrorMessage<GError>> for MiddleError {
    fn into(self) -> ErrorMessage<GError> {
        let MiddleError {
            message,
            range,
            span,
        } = self;
        assert!(span.lo + range.1 <= span.hi);
        ErrorMessage {
            message,
            span: Span {
                lo: span.lo + range.0,
                hi: span.lo + range.1,
            },
        }
    }
}

// TODO: #39 improve error messages
#[derive(Display, Clone)]
pub enum GError {
    #[display(fmt = "Recursion limit")]
    RecursionLimit,
    #[display(fmt = "Not available Rust expression in a template expression")]
    ValidatorExpression,
    #[display(fmt = "Not available Rust expression in a template `if helper` arguments")]
    ValidatorIfs,
    #[display(fmt = "Not available Rust expression in a template `each helper` argument")]
    ValidatorEach,
    #[display(fmt = "Unary negate operator in `unless helper`, use `if helper` instead")]
    ValidatorUnlessNegate,
    #[display(fmt = "Not available Rust expression in a template `unless helper` expression")]
    ValidatorUnless,
    #[display(fmt = "Not available Rust expression in partial scope argument")]
    ValidatorPartialScope,
    #[display(fmt = "Not available Rust expression in partial assign argument")]
    ValidatorPartialAssign,
    #[display(fmt = "Use inside partial block")]
    PartialBlockNoParent,
    #[display(fmt = "Not available in a template expression")]
    NotAvailable,
    #[display(fmt = "Not available in partial argument")]
    PartialArguments,
    #[display(fmt = "Not available Rust expression in partial scope argument")]
    PartialArgumentsScope,
    #[display(fmt = "place scope argument at first position")]
    PartialArgumentsScopeFirst,
    #[display(fmt = "Use reserved word")]
    ReservedWord,
    #[display(fmt = "Not exist in current scope")]
    NotExist,
    #[display(fmt = "Unimplemented")]
    Unimplemented,
    #[display(fmt = "Compile error: {}", _0)]
    UserCompileError(String),
    #[display(fmt = "Internal")]
    Internal,
    #[display(fmt = "use super without any parent")]
    SuperWithoutParent,
}
