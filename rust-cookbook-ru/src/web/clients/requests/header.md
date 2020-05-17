## Установка пользовательских заголовков и параметров в адресе URL для REST запроса

[![reqwest-badge]][reqwest] [![hyper-badge]][hyper] [![url-badge]][url] [![cat-net-badge]][cat-net]

Данный код устанавливает стандартные и пользовательские HTTP заголовки, добавляет параметры в URL адрес для GET-запроса. Создаёт пользовательский заголовок типа `XPoweredBy` посредством макроса [`hyper::header!`](https://doc.servo.org/hyper/macro.header.html).

Затем строит сложный URL адрес с помощью [`Url::parse_with_params`](https://docs.rs/url/*/url/struct.Url.html#method.parse_with_params).  Устанавливает стандартные заголовки  [`header::UserAgent`](https://doc.servo.org/hyper/header/struct.UserAgent.html), [`header::Authorization`](https://doc.servo.org/hyper/header/struct.Authorization.html), и пользовательский `XPoweredBy` используя метод [`RequestBuilder::header`](https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.header), а затем собственно выполняет запрос, используя метод [`RequestBuilder::send`](https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.send).

Данный запрос отправляется сервису [http://httpbin.org/headers](http://httpbin.org/headers), который отвечает JSON словарём, содержащим все переданные заголовки в запросе для простого подтверждения, что всё прошло нормально.

```rust,no_run,ignore
# #[macro_use]
# extern crate error_chain;
extern crate url;
extern crate reqwest;
#[macro_use]
extern crate hyper;
#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use url::Url;
use reqwest::Client;
use reqwest::header::{UserAgent, Authorization, Bearer};

header! { (XPoweredBy, "X-Powered-By") => [String] }

#[derive(Deserialize, Debug)]
pub struct HeadersEcho {
    pub headers: HashMap<String, String>,
}
#
# error_chain! {
#     foreign_links {
#         Reqwest(reqwest::Error);
#         UrlParse(url::ParseError);
#     }
# }

fn main() -> Result<()> {
    let url = Url::parse_with_params("http://httpbin.org/headers",
                                     &[("lang", "rust"), ("browser", "servo")])?;

    let mut response = Client::new()
        .get(url)
        .header(UserAgent::new("Rust-test"))
        .header(Authorization(Bearer { token: "DEadBEEfc001cAFeEDEcafBAd".to_owned() }))
        .header(XPoweredBy("Guybrush Threepwood".to_owned()))
        .send()?;

    let out: HeadersEcho = response.json()?;
    assert_eq!(out.headers["Authorization"],
               "Bearer DEadBEEfc001cAFeEDEcafBAd");
    assert_eq!(out.headers["User-Agent"], "Rust-test");
    assert_eq!(out.headers["X-Powered-By"], "Guybrush Threepwood");
    assert_eq!(response.url().as_str(),
               "http://httpbin.org/headers?lang=rust&browser=servo");

    println!("{:?}", out);
    Ok(())
}
```


