# uniqueid 🔍

 Generates a unique hash/identifier for a system given a set of parameters.

## Example usage

 ```rust
 use uniqueid::IdentifierBuilder;
 
 let identifier = IdentifierBuilder::new()
     .name("HWID") // optional
     .add(IdentifierTypeName::CPU)
     .add(IdentifierTypeName::RAM) 
     .add(IdentifierTypeName::DISK)
     .build();
 
 let result = identifier.build(false); // false = don't hash the output
 
 println!("{}", result);
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

NAME[TYPE(a=b, ...), ...]

NAME - Optional name of the identifier. (Can be used as a label or salt)
TYPE - The type of identifier, e.g. CPU, GPU, etc.
DATA - The data for the identifier, e.g. Vendor=Intel, Model=Xeon E5-2670

This is a very basic example of an identifier, and most use cases will have more types, more data, and hash the output.
```

## Roadmap

- [X] Calculate the HWID **based on the system's hardware** in the library without needing to specify it manually.
- [ ] Add support for other identifier types. (currently only supports CPU, RAM, and DISK) - **PR's welcome**!
- [ ] Add disk serial checking.
- [ ] Add support for other hashing algorithms. (currently only supports SHA3-512)

## License

This software is licensed under the GNU General Public License v3.0

For more information, see the [LICENSE](LICENSE) file in the root directory of this project or [see here](https://www.gnu.org/licenses/gpl-3.0.html).
