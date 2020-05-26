## Разбор строки содержащей сложную версию

[![semver-badge]][semver] [![cat-config-badge]][cat-config]

Строит [`semver::Version`] из строки используя [`Version::parse`]. Строка содержит номер
предварительного релиза и метаданные сборки как определяется в спецификации [Semantic Versioning Specification].

Заметим, что в соответствии со спецификации метаданные сборки анализируются, но не
принимаются во внимание в процессе сравнения версий. Другими словами, две версии могут
быть равны даже если их данные сборки различаются.

```rust
extern crate semver;

use semver::{Identifier, Version, SemVerError};

fn main() -> Result<(), SemVerError> {
    let version_str = "1.0.49-125+g72ee7853";
    let parsed_version = Version::parse(version_str)?;

    assert_eq!(
        parsed_version,
        Version {
            major: 1,
            minor: 0,
            patch: 49,
            pre: vec![Identifier::Numeric(125)],
            build: vec![],
        }
    );
    assert_eq!(
        parsed_version.build,
        vec![Identifier::AlphaNumeric(String::from("g72ee7853"))]
    );

    let serialized_version = parsed_version.to_string();
    assert_eq!(&serialized_version, version_str);

    Ok(())
}
```

[`semver::Version`]: https://docs.rs/semver/*/semver/struct.Version.html
[`Version::parse`]: https://docs.rs/semver/*/semver/struct.Version.html#method.parse

[Semantic Versioning Specification]: http://semver.org/
