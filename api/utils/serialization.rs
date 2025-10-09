use serde::{Serialize, Serializer};

pub fn default_serializer<T: Default + Serialize, S: Serializer>(
  _: &T,
  serializer: S,
) -> Result<S::Ok, S::Error> {
  T::default().serialize(serializer)
}
