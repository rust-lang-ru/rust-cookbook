## Кодирование строки в application/x-www-form-urlencoded

[![url-badge]][url] [![cat-encoding-badge]][cat-encoding]

Кодирует строку в синтаксис [application/x-www-form-urlencoded] используя [`form_urlencoded::byte_serialize`](https://docs.rs/url/*/url/form_urlencoded/fn.byte_serialize.html), а затем декодирует ее с помощью [`form_urlencoded::parse`](https://docs.rs/url/*/url/form_urlencoded/fn.parse.html). Обе функции возвращают итераторы, которые преобразуются в `String`.

```rust
extern crate url;
use url::form_urlencoded::{byte_serialize, parse};

fn main() {
    let urlencoded: String = byte_serialize("What is ❤?".as_bytes()).collect();
    assert_eq!(urlencoded, "What+is+%E2%9D%A4%3F");
    println!("urlencoded:'{}'", urlencoded);

    let decoded: String = parse(urlencoded.as_bytes())
        .map(|(key, val)| [key, val].concat())
        .collect();
    assert_eq!(decoded, "What is ❤?");
    println!("decoded:'{}'", decoded);
}
```


[application/x-www-form-urlencoded]: https://docs.rs/url/*/url/form_urlencoded/fn.byte_serialize.html