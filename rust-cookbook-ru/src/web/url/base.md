## Получение базового URL адреса удалением сегментов пути

[![url-badge]][url] [![cat-net-badge]][cat-net]

Базовый URL адрес включает в себя протокол и домен. Базовые URL адреса не имеют каталогов, файлов или параметров адресной строки. Каждый из этих элементов вырезается из переданного URL адреса. [`PathSegmentsMut::clear`](https://docs.rs/url/*/url/struct.PathSegmentsMut.html#method.clear) удаляет путь и [`Url::set_query`](https://docs.rs/url/*/url/struct.Url.html#method.set_query) удаляет строку параметров.

```rust
# #[macro_use]
# extern crate error_chain;
extern crate url;

use url::Url;
#
# error_chain! {
#     foreign_links {
#         UrlParse(url::ParseError);
#     }
#     errors {
#         CannotBeABase
#     }
# }

fn main() -> Result<()> {
    let full = "https://github.com/rust-lang/cargo?asdf";

    let url = Url::parse(full)?;
    let base = base_url(url)?;

    assert_eq!(base.as_str(), "https://github.com/");
    println!("The base of the URL is: {}", base);

    Ok(())
}

fn base_url(mut url: Url) -> Result<Url> {
    match url.path_segments_mut() {
        Ok(mut path) => {
            path.clear();
        }
        Err(_) => {
            return Err(Error::from_kind(ErrorKind::CannotBeABase));
        }
    }

    url.set_query(None);

    Ok(url)
}
```


