#![forbid(unsafe_code)]
#![allow(dead_code, unused_macros)]

use std::fmt::Display;

use sha3::{Digest, Sha3_512};
use sysinfo::{self, DiskExt, ProcessorExt, System, SystemExt};

/// Enum representing the different types of possible identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IdentifierType {
    CPU,
    // GPU, // TODO: Add GPU support
    RAM,
    DISK,
    // NET, // TODO: Add network identifier
    // OS, // TODO: Add OS identifier
}

impl IdentifierType {
    /// Returns the identifier type as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            IdentifierType::CPU => "CPU",
            // IdentifierType::GPU => "GPU",
            IdentifierType::RAM => "RAM",
            IdentifierType::DISK => "DISK",
            // IdentifierType::NET => "NET",
            // IdentifierType::OS => "OS",
        }
    }
}

impl From<&str> for IdentifierType {
    /// Converts a string to an IdentifierType
    fn from(name: &str) -> Self {
        match name {
            "CPU" => IdentifierType::CPU,
            // "GPU" => IdentifierType::GPU,
            "RAM" => IdentifierType::RAM,
            "DISK" => IdentifierType::DISK,
            // "NET" => IdentifierType::NET,
            // "OS" => IdentifierType::OS,
            _ => panic!("Unknown identifier type name: {}", name),
        }
    }
}

/// A struct representing the key-value pairs of an identifier's type data.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdentifierTypeData {
    /// The key of the IdentifierTypeData object.
    pub key: String,
    /// The value of the IdentifierTypeData object.
    pub value: String,
}

impl IdentifierTypeData {
    /// Creates a new IdentifierTypeData object
    /// # Arguments
    /// * `key` - The key of the IdentifierTypeData object
    /// * `value` - The value of the IdentifierTypeData object
    /// # Example
    /// ```
    /// use uniqueid::identifier::IdentifierTypeData;
    ///
    /// let data = IdentifierTypeData::new("key", "value");
    ///
    /// assert_eq!(data.key, "key");
    /// assert_eq!(data.value, "value");
    /// ```
    /// # Returns
    /// * IdentifierTypeData - The new IdentifierTypeData object
    pub fn new(key: &str, value: &str) -> Self {
        IdentifierTypeData {
            key: key.to_string(),
            value: value.to_string(),
        }
    }

    /// Returns the key of the IdentifierTypeData object.
    /// # Examples
    /// ```
    /// use uniqueid::identifier::IdentifierTypeData;
    ///
    /// let data = IdentifierTypeData::new("key", "value");
    ///
    /// assert_eq!(data.key, "key");
    /// ```
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the value of the IdentifierTypeData object.
    /// # Examples
    /// ```
    /// use uniqueid::identifier::IdentifierTypeData;
    ///
    /// let data = IdentifierTypeData::new("key", "value");
    ///
    /// assert_eq!(data.value, "value");
    /// ```
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Display for IdentifierTypeData {
    /// Returns the key and value in normal format. (key=value)
    /// # Examples
    /// ```
    /// use uniqueid::identifier::IdentifierTypeData;
    ///
    /// let data = IdentifierTypeData::new("key", "value");
    ///
    /// assert_eq!(data.to_string(), "key=value");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}={}", self.key, self.value)
    }
}

/// A helper struct for building IdentifierTypeData objects.
pub struct IdentifierTypeDataBuilder {
    identifier: IdentifierType,
    data: Vec<IdentifierTypeData>,
}

impl IdentifierTypeDataBuilder {
    /// Creates a new IdentifierTypeDataBuilder object.
    /// # Examples
    /// ```
    /// use uniqueid::identifier::IdentifierTypeDataBuilder;
    /// use uniqueid::identifier::IdentifierType;
    ///
    /// let builder = IdentifierTypeDataBuilder::new(IdentifierType::CPU);
    /// ```
    /// # Panics
    /// Panics if the identifier type is not valid.
    /// ```
    pub fn new(identifier: IdentifierType) -> Self {
        IdentifierTypeDataBuilder {
            identifier,
            data: Vec::new(),
        }
    }

    /// Adds a key-value pair to the IdentifierTypeDataBuilder object.
    /// # Examples
    /// ```
    /// use uniqueid::identifier::IdentifierTypeDataBuilder;
    /// use uniqueid::identifier::IdentifierType;
    ///
    /// let mut builder = IdentifierTypeDataBuilder::new(IdentifierType::CPU);
    /// builder.add("key", "value");
    /// ```
    /// # Panics
    /// Panics if the IdentifierTypeDataBuilder object is empty.
    /// ```
    pub fn add<T: Into<String>>(&mut self, key: T, value: T) -> &mut Self {
        self.data.push(IdentifierTypeData {
            key: key.into(),
            value: value.into(),
        });

        self
    }

    /// Builds the IdentifierTypeData object into a string.
    /// # Examples
    /// ```
    /// use uniqueid::identifier::IdentifierTypeDataBuilder;
    /// use uniqueid::identifier::IdentifierType;
    ///
    /// let mut builder = IdentifierTypeDataBuilder::new(IdentifierType::CPU);
    ///
    /// builder.add("key", "value");
    ///
    /// assert_eq!(builder.build(), "CPU(key=value)");
    /// ```
    pub fn build(self) -> String {
        let mut data = String::new();

        data.push_str(self.identifier.as_str());
        data.push('(');

        for item in self.data {
            data.push_str(&format!("{}={}, ", item.key, item.value));
        }

        data.pop();
        data.pop();

        data.push(')');

        data
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdentifierTypeDataList {
    /// The name of the IdentifierType object. (CPU, RAM, DISK, ...)
    pub identifier: IdentifierType,
    /// The data of the IdentifierType object. (key=value, key=value, key=value ...)
    pub data: Vec<IdentifierTypeData>,
}

impl IdentifierTypeDataList {
    /// Creates a new IdentifierType object.
    /// # Examples
    /// ```
    /// use uniqueid::identifier::IdentifierTypeDataList;
    /// use uniqueid::identifier::IdentifierType;
    ///
    /// let data = IdentifierTypeDataList::new(IdentifierType::CPU);
    /// ```
    /// # Panics
    /// Panics if the identifier type is not valid.
    /// ```
    pub fn new(identifier: IdentifierType) -> Self {
        IdentifierTypeDataList {
            identifier,
            data: Vec::new(),
        }
    }

    pub fn build(&self) -> String {
        match self.identifier {
            IdentifierType::CPU => self.build_cpu(),
            // IdentifierType::GPU => self.build_gpu(),
            IdentifierType::RAM => self.build_ram(),
            IdentifierType::DISK => self.build_disk(),
            // IdentifierType::NET => self.build_net(),
            // IdentifierType::OS => self.build_os(),
        }
    }

    fn build_cpu(&self) -> String {
        let mut sys = System::new_all();

        sys.refresh_all();

        let cpu = sys.processors();
        let brand = cpu[0].brand();
        let vendor = cpu[0].vendor_id();
        let frequency = cpu[0].frequency();
        let cores = cpu.len();

        let mut result = String::new();

        let mut identifier_type = IdentifierTypeDataBuilder::new(IdentifierType::CPU);
        identifier_type.add("b", brand.to_lowercase().trim());
        identifier_type.add("v", vendor.to_lowercase().trim());
        identifier_type.add("f", &frequency.to_string());
        identifier_type.add("c", &cores.to_string());
        result.push_str(&identifier_type.build());

        result
    }

    fn build_ram(&self) -> String {
        let mut sys = System::new_all();

        sys.refresh_all();

        let ram = sys.total_memory();

        let mut result = String::new();

        let mut identifier_type = IdentifierTypeDataBuilder::new(IdentifierType::RAM);
        identifier_type.add("t", &ram.to_string());
        result.push_str(&identifier_type.build());

        result
    }

    fn build_disk(&self) -> String {
        let mut sys = System::new_all();

        sys.refresh_all();

        let disks = sys.disks();

        let mut result = String::new();

        for disk in disks {
            if disk.is_removable() {
                continue;
            }

            let total_space = disk.total_space();

            let mut identifier_type = IdentifierTypeDataBuilder::new(IdentifierType::DISK);
            identifier_type.add("t", &total_space.to_string());
            result.push_str(&identifier_type.build());
        }

        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Identifier {
    /// The name of the Identifier.
    pub name: Option<String>,
    /// The data of the Identifier.
    pub data: Vec<IdentifierTypeDataList>,
}

impl Identifier {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Identifier {
            name: Some(name.into()),
            data: Vec::new(),
        }
    }

    /// Builds the Identifier object and returns it as a String.
    /// # Arguments
    /// * `hash` - If true, the Identifier will be hashed with SHA3-512.
    pub fn to_string(&self, hash: bool) -> String {
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

/// IdentifierBuilder is a helper struct for building Identifier objects.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct IdentifierBuilder {
    pub name: Option<String>,
    pub data: Vec<IdentifierTypeDataList>,
}

impl IdentifierBuilder {
    /// Creates a new IdentifierBuilder object.
    /// # Examples
    /// ```
    /// use uniqueid::identifier::IdentifierBuilder;
    /// let builder = IdentifierBuilder::default();
    /// ```
    pub fn new<T: Into<String>>(name: Option<T>, data: Vec<IdentifierTypeDataList>) -> Self {
        if let Some(name) = name {
            IdentifierBuilder {
                name: Some(name.into()),
                data,
            }
        } else {
            IdentifierBuilder { name: None, data }
        }
    }

    /// Sets the name of the Identifier.
    /// # Examples
    /// ```
    /// use uniqueid::identifier::IdentifierBuilder;
    /// let mut builder = IdentifierBuilder::default();
    /// builder.name("test");
    ///
    /// assert_eq!(builder.name, Some("test".to_string()));
    /// ```
    /// # Panics
    /// Panics if the name is not valid.
    pub fn name<T: Into<String>>(&mut self, name: T) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    /// Adds a new IdentifierType object to the IdentifierBuilder.
    /// # Arguments
    /// * `identifier` - The IdentifierType object to add.
    /// # Examples
    /// ```
    /// use uniqueid::identifier::IdentifierBuilder;
    /// use uniqueid::identifier::IdentifierType;
    ///
    /// let mut builder = IdentifierBuilder::default();
    /// builder.add(IdentifierType::CPU);
    ///
    /// assert_eq!(builder.data.len(), 1);
    /// ```
    /// # Panics
    /// Panics if the IdentifierType is not valid.
    pub fn add(&mut self, identifier: IdentifierType) -> &mut Self {
        self.data.push(IdentifierTypeDataList::new(identifier));
        self
    }

    /// Returns an Identifier object from the IdentifierBuilder.
    /// # Examples
    /// ```
    /// use uniqueid::identifier::IdentifierBuilder;
    /// use uniqueid::identifier::IdentifierType;
    ///
    /// let mut builder = IdentifierBuilder::default();
    /// builder.add(IdentifierType::CPU);
    ///
    /// let identifier = builder.build();
    /// ```
    pub fn build(self) -> Identifier {
        Identifier {
            name: self.name,
            data: self.data,
        }
    }
}

mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_identifier_builder() {
        let mut builder = IdentifierBuilder::default();

        builder.name("test");
        builder.add(IdentifierType::CPU);
        builder.add(IdentifierType::RAM);
        builder.add(IdentifierType::DISK);

        let identifier = builder.build();

        assert_eq!(identifier.name, Some("test".to_string()));
        assert_eq!(identifier.data.len(), 3);

        println!("{}", identifier.to_string(false));
        println!("{}", identifier.to_string(true));
    }
}
