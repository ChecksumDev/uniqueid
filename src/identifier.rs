use sha3::{Digest, Sha3_512};
use sysinfo::{self, DiskExt, ProcessorExt, System, SystemExt};

/// IdentifierTypeName are the available types of identifiers.
/// # Examples
/// ```
/// use uniqueid::identifier::IdentifierTypeName;
/// assert_eq!(IdentifierTypeName::CPU.as_str(), "CPU");
/// ```
/// # See Also
/// * [IdentifierType](https://docs.rs/uniqueid/latest/uniqueid/identifier/struct.IdentifierType.html)
/// * [Identifier](https://docs.rs/uniqueid/latest/uniqueid/identifier/struct.Identifier.html)
/// * [IdentifierTypeData](https://docs.rs/uniqueid/latest/uniqueid/identifier/struct.IdentifierTypeData.html)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IdentifierTypeName {
    CPU,
    // GPU, // TODO: Add GPU support
    RAM,
    DISK,
    // NET, // TODO: Add network identifier
    // OS, // TODO: Add OS identifier
}

impl IdentifierTypeName {
    pub fn as_str(&self) -> &'static str {
        match self {
            IdentifierTypeName::CPU => "CPU",
            // IdentifierTypeName::GPU => "GPU",
            IdentifierTypeName::RAM => "RAM",
            IdentifierTypeName::DISK => "DISK",
            // IdentifierTypeName::NET => "NET",
            // IdentifierTypeName::OS => "OS",
        }
    }
}

impl From<&str> for IdentifierTypeName {
    fn from(name: &str) -> Self {
        match name {
            "CPU" => IdentifierTypeName::CPU,
            // "GPU" => IdentifierTypeName::GPU,
            "RAM" => IdentifierTypeName::RAM,
            "DISK" => IdentifierTypeName::DISK,
            // "NET" => IdentifierTypeName::NET,
            // "OS" => IdentifierTypeName::OS,
            _ => panic!("Unknown identifier type name: {}", name),
        }
    }
}

/// The IdentifierTypeData object is the data associated with an IdentifierType.
/// # Examples
/// ```
/// use uniqueid::identifier::IdentifierTypeData;
///
/// let data = IdentifierTypeData::new("key", "value");
/// assert_eq!(data.key, "key");
/// assert_eq!(data.value, "value");
/// ```
/// # See Also
/// * [IdentifierType](https://docs.rs/uniqueid/latest/uniqueid/identifier/struct.IdentifierType.html)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdentifierTypeData {
    /// The key of the IdentifierTypeData object.
    pub key: String,
    /// The value of the IdentifierTypeData object.
    pub value: String,
}

pub struct IdentifierTypeDataBuilder {
    r#type: IdentifierTypeName,
    data: Vec<IdentifierTypeData>,
}

impl IdentifierTypeDataBuilder {
    pub fn new(r#type: IdentifierTypeName) -> Self {
        IdentifierTypeDataBuilder {
            r#type,
            data: Vec::new(),
        }
    }

    pub fn add(&mut self, key: &str, value: &str) -> &mut Self {
        self.data.push(IdentifierTypeData {
            key: key.to_string(),
            value: value.to_string(),
        });
        self
    }

    pub fn build(self) -> String {
        // Build the IdentifierTypeData objects in format:
        // TYPE_NAME(key=value, key=value, key=value ...)
        let mut data = String::new();
        data.push_str(self.r#type.as_str());

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

impl IdentifierTypeData {
    /// Creates a new IdentifierTypeData object.
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
    /// let data = IdentifierTypeData::new("key", "value");
    /// assert_eq!(data.key(), "key");
    /// ```
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the value of the IdentifierTypeData object.
    /// # Examples
    /// ```
    /// use uniqueid::identifier::IdentifierTypeData;
    /// let data = IdentifierTypeData::new("key", "value");
    /// assert_eq!(data.value(), "value");
    /// ```
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns the key and value in normal format. (key=value)
    /// # Examples
    /// ```
    /// use uniqueid::identifier::IdentifierTypeData;
    /// let data = IdentifierTypeData::new("key", "value");
    /// assert_eq!(data.to_string(), "key=value");
    /// ```
    pub fn to_string(&self) -> String {
        format!("{}={}", self.key, self.value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdentifierType {
    /// The name of the IdentifierType object.
    pub name: IdentifierTypeName,
    /// The data of the IdentifierType object.
    pub data: Vec<IdentifierTypeData>,
}

impl IdentifierType {
    /// Creates a new IdentifierType object.
    pub fn new(r#type: IdentifierTypeName) -> Self {
        IdentifierType {
            name: r#type,
            data: Vec::new(),
        }
    }

    pub fn build(&self) -> String {
        match self.name {
            IdentifierTypeName::CPU => self.build_cpu(),
            // IdentifierTypeName::GPU => self.build_gpu(),
            IdentifierTypeName::RAM => self.build_ram(),
            IdentifierTypeName::DISK => self.build_disk(),
            // IdentifierTypeName::NET => self.build_net(),
            // IdentifierTypeName::OS => self.build_os(),
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

        let mut identifier_type = IdentifierTypeDataBuilder::new(IdentifierTypeName::CPU);
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

        let mut identifier_type = IdentifierTypeDataBuilder::new(IdentifierTypeName::RAM);
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

            let mut identifier_type = IdentifierTypeDataBuilder::new(IdentifierTypeName::DISK);
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
    pub data: Vec<IdentifierType>,
}

impl Identifier {
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

/// Builds an Identifier object.
/// # Examples
/// ```
/// use uniqueid::identifier::{IdentifierBuilder, IdentifierTypeName};
///
/// let identifier = IdentifierBuilder::new()
///    .name("name") // Optional
///     .add(IdentifierTypeName::CPU)
///     .add(IdentifierTypeName::RAM)
///     .add(IdentifierTypeName::DISK)
///     .build();
/// ```
/// # See Also
/// * [Identifier](https://docs.rs/uniqueid/latest/uniqueid/identifier/struct.Identifier.html)
/// * [IdentifierType](https://docs.rs/uniqueid/latest/uniqueid/identifier/struct.IdentifierType.html)
/// * [IdentifierTypeName](https://docs.rs/uniqueid/latest/uniqueid/identifier/enum.IdentifierTypeName.html)
/// * [IdentifierTypeData](https://docs.rs/uniqueid/latest/uniqueid/identifier/struct.IdentifierTypeData.html)
/// * [IdentifierTypeDataBuilder](https://docs.rs/uniqueid/latest/uniqueid/identifier/struct.IdentifierTypeDataBuilder.html)
pub struct IdentifierBuilder {
    name: Option<String>,
    data: Vec<IdentifierType>,
}

impl IdentifierBuilder {
    pub fn new() -> Self {
        IdentifierBuilder {
            name: None,
            data: Vec::new(),
        }
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn add(&mut self, type_: IdentifierTypeName) -> &mut Self {
        self.data.push(IdentifierType::new(type_));
        self
    }

    pub fn build(&self) -> Identifier {
        Identifier {
            name: self.name.clone(),
            data: self.data.clone(),
        }
    }
}
