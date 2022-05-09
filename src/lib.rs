#![deny(unsafe_code)]
#![allow(dead_code, unused_macros)]

pub mod identifier;
pub mod macros;

pub mod tests {
    use super::*;

    #[test]
    fn test_identifier_type_data() {
        let data = identifier::IdentifierTypeData::new("key", "value");
        assert_eq!(data.key, "key");
        assert_eq!(data.value, "value");
    }

    #[test]
    fn test_identifier_type() {
        let data = vec![identifier::IdentifierTypeData::new("key", "value")];
        let type_ = identifier::IdentifierType::new("name".to_string(), data.clone());
        assert_eq!(type_.name, "name");
        assert_eq!(type_.data, data);

        let type_ = type_.build();
        assert_eq!(type_, "name(key=value)");
    }

    #[test]
    fn test_identifier() {
        let data = vec![identifier::IdentifierTypeData::new("key", "value")];
        let type_ = identifier::IdentifierType::new("name".to_string(), data.clone());

        let identifier = identifier::Identifier::new(Some("name"), vec![type_.clone()]);

        assert_eq!(identifier.name, Some("name".to_string()));
        assert_eq!(identifier.data, vec![type_]);

        let identifier_nohash = identifier.build(false);
        assert_eq!(identifier_nohash, "name[name(key=value)]");

        let identifier_hash = identifier.build(true);
        assert_eq!(identifier_hash, "e49dc6621f68ebd6e9cf46441f36222bc8325727f278edf20fb990da70fa90db9a5ee8d5acec458703b49701682ff1e2de2483a0e7dba87f1fc14a7a1c20fb7e");
    }

    #[test]
    fn test_macros() {
        let data = vec![
            identifier_data!("Vendor", "Intel"),
            identifier_data!("Model", "Xeon E5-2670"),
        ];

        let type_ = identifier_type!(CPU, data);
        let identifier = identifier::Identifier::new(Some("HWID"), vec![type_.clone()]);

        assert_eq!(identifier.name, Some("HWID".to_string()));
        assert_eq!(identifier.data, vec![type_]);

        let identifier_nohash = identifier.build(false);
        assert_eq!(
            identifier_nohash,
            "HWID[CPU(Vendor=Intel, Model=Xeon E5-2670)]"
        );

        let identifier_hash = identifier.build(true);
        assert_eq!(identifier_hash, "92187e765ecc39a93e9f46fa0a951d52822f8c1691cedd004db1a0e1b86a82aa5a1cd916f6766c8f798787b00eaedc4ada62e1043d1649abbe02161c42a59cfa");
    }
}
