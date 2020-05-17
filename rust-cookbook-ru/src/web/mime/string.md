## Получение MIME типа из строки

[![mime-badge]][mime] [![cat-encoding-badge]][cat-encoding]

Этот пример показывает, как осуществить разбор в тип [`MIME`](https://docs.rs/mime/*/mime/struct.Mime.html) из некоторой строки при помощи функций из крейта [mime]. [`FromStrError`](https://docs.rs/mime/*/mime/struct.FromStrError.html) выводит [`MIME`](https://docs.rs/mime/*/mime/struct.Mime.html) тип по умолчанию для параметра, переданного в `unwrap_or`.

```rust
extern crate mime;
use mime::{Mime, APPLICATION_OCTET_STREAM};

fn main() {
    let invalid_mime_type = "i n v a l i d";
    let default_mime = invalid_mime_type
        .parse::<Mime>()
        .unwrap_or(APPLICATION_OCTET_STREAM);

    println!(
        "MIME for {:?} used default value {:?}",
        invalid_mime_type, default_mime
    );

    let valid_mime_type = "TEXT/PLAIN";
    let parsed_mime = valid_mime_type
        .parse::<Mime>()
        .unwrap_or(APPLICATION_OCTET_STREAM);

    println!(
        "MIME for {:?} was parsed as {:?}",
        valid_mime_type, parsed_mime
    );
}
```

