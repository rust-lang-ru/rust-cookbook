## Запрос к API GitHub

[![reqwest-badge]][reqwest] [![serde-badge]][serde] [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding]

Пример посылает запрос к GitHub [stargazers API v3](https://developer.github.com/v3/activity/starring/#list-stargazers) используя метод [`reqwest::get`](https://docs.rs/reqwest/*/reqwest/fn.get.html), чтобы получить список всех пользователей, кто отметил какой-то проект на GitHub звёздочкой. [`reqwest::Response`](https://docs.rs/reqwest/*/reqwest/struct.Response.html) десериализуется с помощью [`Response::json`](https://docs.rs/reqwest/*/reqwest/struct.Response.html#method.json) в объекты типа `User`, которые реализуют типаж [`serde::Deserialize`](https://docs.rs/serde/*/serde/trait.Deserialize.html).

[tokio::main] используется, чтобы установить асинхронный движок и обрабатывать ожидания выполнения для [`reqwet::get`] до обработки запросов и преобразования в значения типа User.

```rust,edition2018,no_run
use serde::Deserialize;
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
                              owner = "rust-lang-nursery",
                              repo = "rust-cookbook");
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await?;

    let users: Vec<User> = response.json().await?;
    println!("{:?}", users);
    Ok(())
}
```


