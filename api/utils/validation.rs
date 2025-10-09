use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case", rename_all_fields = "kebab-case")]
pub enum Validation {
  Duplicate,
  TooLong,
  TooShort,
}

macro_rules! invalid {
  ($($path:ident).+, $($field:ident).+, $error:expr) => {
    {
      let _ = &$($path).+.$($field).+;
      (stringify!($($field).+), $error)
    }
  };
}

pub(crate) use invalid;
