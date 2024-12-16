## Extract all links from a webpage HTML

[![reqwest-badge]][reqwest] [![select-badge]][select] [![cat-net-badge]][cat-net]

Use [`reqwest::get`] to perform a HTTP GET request and then use
[`Document::from_read`] to parse the response into a HTML document.
[`find`] with the criteria of [`Name`] is "a" retrieves all links.
Call [`filter_map`] on the [`Selection`] retrieves URLs
from links that have the "href" [`attr`] (attribute).

```rust,edition2024,no_run
mod links {
  {{#include ../../../crates/web/src/links.rs}}
}

#[tokio::main]
async fn main() -> Result<(), links::LinkError> {
    let page_links = links::get_links("https://www.rust-lang.org/en-US/").await?;
    for link in page_links {
        println!("{}", link);
    }
    Ok(())
}
```

[`attr`]: https://docs.rs/select/*/select/node/struct.Node.html#method.attr
[`Document::from_read`]: https://docs.rs/select/*/select/document/struct.Document.html#method.from_read
[`filter_map`]: https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.filter_map
[`find`]: https://docs.rs/select/*/select/document/struct.Document.html#method.find
[`Name`]: https://docs.rs/select/*/select/predicate/struct.Name.html
[`reqwest::get`]: https://docs.rs/reqwest/*/reqwest/fn.get.html
[`Selection`]: https://docs.rs/select/*/select/selection/struct.Selection.html
