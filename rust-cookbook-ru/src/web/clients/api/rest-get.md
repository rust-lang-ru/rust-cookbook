## Запрос к API GitHub

[![reqwest-badge]][reqwest] [![serde-badge]][serde] [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding]

Пример посылает запрос к GitHub [stargazers API v3](https://developer.github.com/v3/activity/starring/#list-stargazers) используя метод [`reqwest::get`](https://docs.rs/reqwest/*/reqwest/fn.get.html), чтобы получить список всех пользователей, кто отметил какой-то проект на GitHub звёздочкой. [`reqwest::Response`](https://docs.rs/reqwest/*/reqwest/struct.Response.html) десериализуется с помощью [`Response::json`](https://docs.rs/reqwest/*/reqwest/struct.Response.html#method.json) в объекты типа `User`, которые реализуют типаж [`serde::Deserialize`](https://docs.rs/serde/*/serde/trait.Deserialize.html).

```rust,no_run
#[macro_use]
extern crate serde;
extern crate serde_derive;
extern crate reqwest;
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

fn main() -> Result<(), Error> {
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
                              owner = "rust-lang-nursery",
                              repo = "rust-cookbook");
    println!("{}", request_url);
    let mut response = reqwest::get(&request_url)?;

    let users: Vec<User> = response.json()?;
    println!("{:?}", users);
    Ok(())
}
```


