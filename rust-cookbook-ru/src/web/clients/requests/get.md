## Выполнение HTTP GET-запроса

[![reqwest-badge]][reqwest] [![cat-net-badge]][cat-net]

В примере разбирается переданный адрес URL и выполняется синхронный HTTP GET запрос с помощью [`reqwest::get`](https://docs.rs/reqwest/*/reqwest/fn.get.html). Затем печатается статус ответа [`reqwest::Response`](https://docs.rs/reqwest/*/reqwest/struct.Response.html) и его заголовки. Также читается тело HTTP ответа в предварительно выделенную строку [`String`](https://doc.rust-lang.org/std/string/struct.String.html) используя [`read_to_string`](https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_string).

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate reqwest;

use std::io::Read;
#
# error_chain! {
#     foreign_links {
#         Io(std::io::Error);
#         HttpRequest(reqwest::Error);
#     }
# }

fn main() -> Result<()> {
    let mut res = reqwest::get("http://httpbin.org/get")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);

    Ok(())
}
```


