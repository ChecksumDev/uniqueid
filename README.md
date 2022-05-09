# uniqueid 🔍

 Generates a unique hash/identifier for a system given a set of parameters.

## Example usage

```rust
use uniqueid;

pub fn main() {
    let data = vec![
        identifier_data!("Vendor", "Intel"),
        identifier_data!("Model", "Xeon E5-2670"),
    ];

    let type_ = identifier_type!(CPU, data);

    let identifier = uniqueid::identifier::Identifier::new(Some("HWID"), vec![type_.clone()]);

    let output = identifier.build(false); // true = hash

    println!("{}", output); // Outputs HWID[CPU(Vendor=Intel, Model=Xeon E5-2670)]
}
```

___

## Specification

```text
UniqueID uses its own specification.

The spec consists of three parts:
    - The Identifier; []
    - The IdentifierType; ()
    - The IdentifierTypeData; (a=b, ...)
      
A basic identifier would look like this: 

NAME[TYPE(a=b, ...)]

NAME - Optional, defaults to None
TYPE - The type of identifier, e.g. CPU, GPU, etc.
DATA - The data for the identifier, e.g. Vendor=Intel, Model=Xeon E5-2670

This is a very basic example of an identifier, and most use cases will have more types, more data, and hash the output.
```

## Roadmap

- [ ] Calculate the HWID **based on the system's hardware** in the library without needing to specify it manually.
- [ ] Add support for other hashing algorithms. (currently only supports SHA3-512)

## License

This software is licensed under the GNU General Public License v3.0

For more information, see the [LICENSE](LICENSE) file in the root directory of this project or [see here](https://www.gnu.org/licenses/gpl-3.0.html).
