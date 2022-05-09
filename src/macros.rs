/// Creates a new IdentifierTypeData object.
#[macro_export]
macro_rules! identifier_data {
    ($key:expr, $value:expr) => {
        identifier::IdentifierTypeData::new($key, $value)
    };
}

/// Creates a new IdentifierType object
#[macro_export]
macro_rules! identifier_type {
    ($name:ident, $data:expr) => {
        identifier::IdentifierType {
            name: stringify!($name).to_string(),
            data: $data,
        }
    };
}