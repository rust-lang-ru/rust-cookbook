## Проверка существования ресурса в API

[![reqwest-badge]][reqwest] [![cat-net-badge]][cat-net]

Данный рецепт запрашивает точку Users широко известного сервиса GitHub с использованием метода HEAD ([`Client::head`](https://docs.rs/reqwest/*/reqwest/struct.Client.html#method.head)) и затем смотрит код ответа для определения того, был ли вызов успешен или нет. Такой способ подходит для того, чтобы быстро "ткнуть" в REST ресурс без необходимости загружать тело ответа. [`reqwest::Client`](https://docs.rs/reqwest/*/reqwest/struct.Client.html) сконфигурированный с [`ClientBuilder::timeout`](https://docs.rs/reqwest/*/reqwest/struct.ClientBuilder.html#method.timeout), который гарантирует то, что запрос не будет длиться дольше данного временного интервала.

```rust,no_run
extern crate reqwest;

use reqwest::Error;
use std::time::Duration;
use reqwest::ClientBuilder;


fn main() -> Result<(), Error> {
    let user = "ferris-the-crab";
    let request_url = format!("https://api.github.com/users/{}", user);
    println!("{}", request_url);

    let timeout = Duration::new(5, 0);
    let client = ClientBuilder::new().timeout(timeout).build()?;
    let response = client.head(&request_url).send()?;

    if response.status().is_success() {
        println!("{} is a user!", user);
    } else {
        println!("{} is not a user!", user);
    }

    Ok(())
}
```


