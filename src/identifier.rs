use sha3::{Digest, Sha3_512, Sha3_512Core};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdentifierTypeData {
    /// The key of the IdentifierTypeData object.
    pub key: String,
    /// The value of the IdentifierTypeData object.
    pub value: String,
}

impl IdentifierTypeData {
    /// Creates a new IdentifierTypeData object.
    pub fn new(key: &str, value: &str) -> Self {
        IdentifierTypeData {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Contains the type of an identifier.
pub struct IdentifierType {
    /// The name of the IdentifierType object.
    pub name: String,
    /// The data of the IdentifierType object.
    pub data: Vec<IdentifierTypeData>,
}

impl IdentifierType {
    /// Creates a new IdentifierType object.
    pub fn new(name: String, data: Vec<IdentifierTypeData>) -> Self {
        IdentifierType { name, data }
    }

    /// Builds the IdentifierType object.
    pub fn build(&self) -> String {
        let mut result = String::new();
        result.push_str(&self.name.to_string());
        result.push('(');
        for i in &self.data {
            result.push_str(&i.key);
            result.push('=');
            result.push_str(&i.value);
            result.push_str(", ");
        }

        result.pop();
        result.pop();
        result.push(')');

        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Identifier {
    /// The name of the Identifier.
    pub name: Option<String>,
    /// The data of the Identifier.
    pub data: Vec<IdentifierType>,
}

impl Identifier {
    /// Creates a new Identifier object.
    pub fn new(name: Option<&str>, data: Vec<IdentifierType>) -> Identifier {
        Identifier {
            name: name.map(|s| s.to_string()),
            data,
        }
    }

    /// Builds the Identifier object and returns it as a String.
    /// # Arguments
    /// * `hash` - If true, the Identifier will be hashed with SHA3-512.
    pub fn build(&self, hash: bool) -> String {
        let mut result = String::new();
        if let Some(name) = &self.name {
            result.push_str(name);
        }
        result.push('[');
        for i in &self.data {
            result.push_str(&i.build());
            result.push_str(", ");
        }
        result.pop();
        result.pop();
        result.push(']');

        if hash {
            let mut hasher = Sha3_512::default();
            let mut result_bytes = result.as_bytes();

            Digest::update(&mut hasher, &mut result_bytes);
            let result_hash = format!("{:x}", hasher.finalize());

            return result_hash;
        }

        result
    }
}
