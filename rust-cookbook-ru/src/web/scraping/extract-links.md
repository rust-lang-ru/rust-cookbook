## Извлечение всех ссылок из веб-страницы HTML

[![reqwest-badge]][reqwest] [![select-badge]][select] [![cat-net-badge]][cat-net]

Используется [`reqwest::get`](https://docs.rs/reqwest/*/reqwest/fn.get.html) для выполнения HTTP GET-запроса и затем используется метод [`Document::from_read`](https://docs.rs/select/*/select/document/struct.Document.html#method.from_read) для разбора ответа как HTML документа. [`find`](https://docs.rs/select/*/select/document/struct.Document.html#method.find) с условием, что '[`Name`](https://docs.rs/select/*/select/predicate/struct.Name.html) должен быть "a"' извлекает все ссылки. Вызов [`filter_map`](https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.filter_map) на результате [`Selection`](https://docs.rs/select/*/select/selection/struct.Selection.html) извлекает все URL адреса из ссылок с аттрибутом "href", атрибуты можно получить с помощью [`attr`](https://docs.rs/select/*/select/node/struct.Node.html#method.attr) (attribute).

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::Name;
#
# error_chain! {
#    foreign_links {
#        ReqError(reqwest::Error);
#        IoError(std::io::Error);
#    }
# }

fn main() -> Result<()> {
    let res = reqwest::get("https://www.rust-lang.org/en-US/")?;

    Document::from_read(res)?
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));

    Ok(())
}
```


