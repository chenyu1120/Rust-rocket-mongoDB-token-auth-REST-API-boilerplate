pub mod hello_name;
pub mod refresh_tokens;

enum HelloNameError {
    OnlyLogin(String),
    NoOnlyLogin(String),
    ErrorID,
    ErrorUnknown,
}