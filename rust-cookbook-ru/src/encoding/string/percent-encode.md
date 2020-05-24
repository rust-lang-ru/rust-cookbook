## Процент (или URL) кодирование строки

[![percent-encoding-badge]](https://en.wikipedia.org/wiki/Percent-encoding) [![cat-encoding-badge]][cat-encoding]

Кодирование входной строки с помощью [percent-encoding], используя [`utf8_percent_encode`](https://docs.rs/percent-encoding/*/percent_encoding/fn.utf8_percent_encode.html) из крейта`percent-encoding`. Затем выполнение декодирования с помощью функции [`percent_decode`](https://docs.rs/percent-encoding/*/percent_encoding/fn.percent_decode.html).

```rust,ignore
extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, percent_decode, AsciiSet, CONTROLS};
use std::str::Utf8Error;

/// https://url.spec.whatwg.org/#fragment-percent-encode-set
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

fn main() -> Result<(), Utf8Error> {
    let input = "confident, productive systems programming";

    let iter = utf8_percent_encode(input, FRAGMENT);
    let encoded: String = iter.collect();
    assert_eq!(encoded, "confident,%20productive%20systems%20programming");

    let iter = percent_decode(encoded.as_bytes());
    let decoded = iter.decode_utf8()?;
    assert_eq!(decoded, "confident, productive systems programming");

    Ok(())
}
```

Набор кодирования определяет, какие байты (в дополнение к не-ASCII и символам управления) должны кодироваться в процентах. Выбор этого набора зависит от контекста. Например, `url` кодирует `?` в пути URL, но не в строке запроса.

Возвращаемое значение кодировки - это итератор срезов `&str`, которые собираются в тип `String`.


[percent-encoding]: https://docs.rs/percent-encoding/*/percent_encoding/fn.percent_decode.html