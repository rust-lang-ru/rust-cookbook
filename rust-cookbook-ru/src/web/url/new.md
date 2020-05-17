## Создание новых URL адресов из базового URL адреса

[![url-badge]][url] [![cat-net-badge]][cat-net]

Метод [`join`](https://docs.rs/url/*/url/struct.Url.html#method.join) создаёт новый URL адрес из базового и относительного пути.

```rust
extern crate url;

use url::{Url, ParseError};

fn main() -> Result<(), ParseError> {
    let path = "/rust-lang/cargo";

    let gh = build_github_url(path)?;

    assert_eq!(gh.as_str(), "https://github.com/rust-lang/cargo");
    println!("The joined URL is: {}", gh);

    Ok(())
}

fn build_github_url(path: &str) -> Result<Url, ParseError> {
    const GITHUB: &'static str = "https://github.com";

    let base = Url::parse(GITHUB).expect("hardcoded URL is known to be valid");
    let joined = base.join(path)?;

    Ok(joined)
}
```


