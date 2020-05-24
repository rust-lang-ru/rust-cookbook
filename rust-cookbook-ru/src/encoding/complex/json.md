## Сериализация и десериализация неструктурированного JSON

[![serde-json-badge]][serde-json] [![cat-encoding-badge]][cat-encoding]

Крейт [`serde_json`] предоставляет функцию [`from_str`] для разбора и анализа `&str` JSON.

Неструктурированный JSON можно проанализировать в универсальное значение [`serde_json::Value`], которое может представлять любые допустимые данные JSON.

В приведенном ниже примере показано, как анализируется `&str` с JSON. Ожидаемое значение объявляется с помощью макро [`json!`].

```rust
#[macro_use]
extern crate serde_json;

use serde_json::{Value, Error};

fn main() -> Result<(), Error> {
    let j = r#"{
                 "userid": 103609,
                 "verified": true,
                 "access_privileges": [
                   "user",
                   "admin"
                 ]
               }"#;

    let parsed: Value = serde_json::from_str(j)?;

    let expected = json!({
        "userid": 103609,
        "verified": true,
        "access_privileges": [
            "user",
            "admin"
        ]
    });

    assert_eq!(parsed, expected);

    Ok(())
}
```


[`from_str`]: https://docs.serde.rs/serde_json/fn.from_str.html
[`json!`]: https://docs.serde.rs/serde_json/macro.json.html
[`serde_json`]: https://docs.serde.rs/serde_json/
[`serde_json::Value`]: https://docs.serde.rs/serde_json/enum.Value.html