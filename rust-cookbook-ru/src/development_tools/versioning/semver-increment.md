## Анализ и инкремент строки с версией

[![semver-badge]][semver] [![cat-config-badge]][cat-config]

Пример строит [`semver::Version`] из строкового значения используя [`Version::parse`],
 затем осуществляет инкремент номера патча, минорного и мажорного номера версии один за одним.

Заметим, что в соответствии со спецификацией [Semantic Versioning Specification]
увеличение минорного номера версии сбрасывает счётчик патча на 0. Аналогично увеличение
мажорного номера версии сбрасывает и минорный номер и номер патча на 0.

```rust
extern crate semver;

use semver::{Version, SemVerError};

fn main() -> Result<(), SemVerError> {
    let mut parsed_version = Version::parse("0.2.6")?;

    assert_eq!(
        parsed_version,
        Version {
            major: 0,
            minor: 2,
            patch: 6,
            pre: vec![],
            build: vec![],
        }
    );

    parsed_version.increment_patch();
    assert_eq!(parsed_version.to_string(), "0.2.7");
    println!("New patch release: v{}", parsed_version);

    parsed_version.increment_minor();
    assert_eq!(parsed_version.to_string(), "0.3.0");
    println!("New minor release: v{}", parsed_version);

    parsed_version.increment_major();
    assert_eq!(parsed_version.to_string(), "1.0.0");
    println!("New major release: v{}", parsed_version);

    Ok(())
}
```

[`semver::Version`]: https://docs.rs/semver/*/semver/struct.Version.html
[`Version::parse`]: https://docs.rs/semver/*/semver/struct.Version.html#method.parse

[Semantic Versioning Specification]: http://semver.org/
