## Удаление идентификаторов фрагментов и пар "параметр-значение" из URL адреса

[![url-badge]][url] [![cat-net-badge]][cat-net]

Пример делает разбор [`Url`](https://docs.rs/url/*/url/struct.Url.html) и разбивает их с помощью [`url::Position`](https://docs.rs/url/*/url/enum.Position.html) чтобы отрезать ненужные части URL адреса.

```rust

extern crate url;

use url::{Url, Position, ParseError};

fn main() -> Result<(), ParseError> {
    let parsed = Url::parse("https://github.com/rust-lang/rust/issues?labels=E-easy&state=open")?;
    let cleaned: &str = &parsed[..Position::AfterPath];
    println!("cleaned: {}", cleaned);
    Ok(())
}
```


