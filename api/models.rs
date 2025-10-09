use serde::{Serialize, Serializer};

pub use self::user::User;

pub fn default_serializer<T: Default + Serialize, S: Serializer>(
    _: &T,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    T::default().serialize(serializer)
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case", rename_all_fields = "kebab-case")]
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
