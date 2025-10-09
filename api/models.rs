use serde::Serialize;

pub use self::user::User;

#[derive(Serialize)]
#[serde(rename_all = "snake_case", rename_all_fields = "snake_case")]
pub enum FieldError {
    Duplicate,
    TooLong,
    TooShort,
}

macro_rules! invalid_field {
    ($($path:ident).+, $($field:ident).+, $error:expr) => {
        {
            let _ = &$($path).+.$($field).+;
            (stringify!($($field).+), $error)
        }
    };
}

mod user;
