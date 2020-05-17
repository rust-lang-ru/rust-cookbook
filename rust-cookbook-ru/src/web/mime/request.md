## Разбор MIME типа из HTTP ответа

[![reqwest-badge]][reqwest] [![mime-badge]][mime] [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding]

В ответе запроса по HTTP через `reqwest` найти [MIME type](https://developer.mozilla.org/docs/Web/HTTP/Basics_of_HTTP/MIME_types)-тип или тип мультимедиа может быть возможно через заголовок [Content-Type](https://developer.mozilla.org/docs/Web/HTTP/Headers/Content-Type). [`reqwest::header::HeaderMap::get`](https://docs.rs/reqwest/*/reqwest/header/struct.HeaderMap.html#method.get) разбирает содержимое заголовка и делает любое значение доступным как [`reqwest::header::HeaderValue`](https://docs.rs/reqwest/*/reqwest/header/struct.HeaderValue.html), которое может быть преобразовано в строку. Крейт `mime` может затем произвести разбор этой строки и выдать значение типа [`mime::Mime`](https://docs.rs/mime/*/mime/struct.Mime.html).

Крейт `mime` также определяет широко используемые типы MIME.

Заметим, что модуль [`reqwest::header`](https://docs.rs/reqwest/*/reqwest/header/index.html) экспортируется из крейта [`http`](https://docs.rs/http/*/http/).

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate mime;
extern crate reqwest;

use mime::Mime;
use std::str::FromStr;
use reqwest::header::CONTENT_TYPE;

#
# error_chain! {
#    foreign_links {
#        Reqwest(reqwest::Error);
#        Header(reqwest::header::ToStrError);
#        Mime(mime::FromStrError);
#    }
# }

fn main() -> Result<()> {
    let response = reqwest::get("https://www.rust-lang.org/logos/rust-logo-32x32.png")?;
    let headers = response.headers();

    match headers.get(CONTENT_TYPE) {
        None => {
            println!("The response does not contain a Content-Type header.");
        }
        Some(content_type) => {
            let content_type = Mime::from_str(content_type.to_str()?)?;
            let media_type = match (content_type.type_(), content_type.subtype()) {
                (mime::TEXT, mime::HTML) => "a HTML document",
                (mime::TEXT, _) => "a text document",
                (mime::IMAGE, mime::PNG) => "a PNG image",
                (mime::IMAGE, _) => "an image",
                _ => "neither text nor image",
            };

            println!("The reponse contains {}.", media_type);
        }
    };

    Ok(())
}
```


