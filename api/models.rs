use serde::Serialize;

pub use self::user::User;

#[derive(Serialize)]
#[serde(rename_all = "snake_case", rename_all_fields = "snake_case")]
pub enum FieldError {
    Duplicate,
    TooLong,
    TooShort,
}

#[derive(Serialize)]
pub struct InvalidField {
    field: &'static str,
    error: FieldError,
}

macro_rules! invalid_field {
    ($struct:ident, $($field:ident).+, $error:expr) => {
        {
            let _ = &$struct.$($field).+;
            InvalidField {
                field: stringify!($($field).+),
                error: $error,
            }
        }
    };
}

mod user;
