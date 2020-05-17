## Разбор строки в экземпляр типа `Url`

[![url-badge]][url] [![cat-net-badge]][cat-net]

Метод [`parse`](https://docs.rs/url/*/url/struct.Url.html#method.parse) из крейта `url` делает синтаксический разбор и валидацию некоторой `&str` в структуру [`Url`](https://docs.rs/url/*/url/struct.Url.html). Поскольку входная строка может быть некорректной, то метод возвращает `Result<Url, ParseError>`.

Как только строка, представляющая собой URL адрес оказывается разобрана в структуру типа `Url`, на этом экземпляре можно вызвать любой нужный метод.

```rust
extern crate url;

use url::{Url, ParseError};

fn main() -> Result<(), ParseError> {
    let s = "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open";

    let parsed = Url::parse(s)?;
    println!("The path part of the URL is: {}", parsed.path());

    Ok(())
}
```


