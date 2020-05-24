## Кодирование и декодирование шестнадцатеричного кода

[![data-encoding-badge]][data-encoding] [![cat-encoding-badge]][cat-encoding]

Крейт [`data_encoding`](https://docs.rs/data-encoding/*/data_encoding/) предоставляет метод `HEXUPPER::encode`, который принимает `&[u8]` и возвращает `String` содержащую шестнадцатеричное представление данных.

Аналогичным образом предоставляется метод `HEXUPPER::decode`, который принимает `&[u8]` и возвращает `Vec<u8>`, если входные данные успешно декодированы.

Пример ниже конвертирует данные `&[u8]` в шестнадцатеричный эквивалент. Сравнивает эти значение с ожидаемым значением.

```rust
extern crate data_encoding;

use data_encoding::{HEXUPPER, DecodeError};

fn main() -> Result<(), DecodeError> {
    let original = b"The quick brown fox jumps over the lazy dog.";
    let expected = "54686520717569636B2062726F776E20666F78206A756D7073206F76\
        657220746865206C617A7920646F672E";

    let encoded = HEXUPPER.encode(original);
    assert_eq!(encoded, expected);

    let decoded = HEXUPPER.decode(&encoded.into_bytes())?;
    assert_eq!(&decoded[..], &original[..]);

    Ok(())
}
```


