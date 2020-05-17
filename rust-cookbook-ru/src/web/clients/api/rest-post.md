## Создать и удалить Gist через GitHub API

[![reqwest-badge]][reqwest] [![serde-badge]][serde] [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding]

Пример создаёт фрагмент кода (в терминах GitHub называется gist) с помощью POST-запроса [gists API v3](https://developer.github.com/v3/gists/) используя [`Client::post`](https://docs.rs/reqwest/*/reqwest/struct.Client.html#method.post) и удаляет только что созданный фрагмент с помощью DELETE-запроса используя [`Client::delete`](https://docs.rs/reqwest/*/reqwest/struct.Client.html#method.delete).

Объект [`reqwest::Client`](https://docs.rs/reqwest/*/reqwest/struct.Client.html) отвечает за детали для обоих типов запросов выше, детали включают URL адрес, тело запроса и аутентификацию. Тело POST-запроса как JSON можно подготовить с помощью макроса [`serde_json::json!`](https://docs.rs/serde_json/*/serde_json/macro.json.html). Вызов [`RequestBuilder::json`](https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.json) определяет тело запроса. [`RequestBuilder::basic_auth`](https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.basic_auth) управляет аутентификацией. А вызов [`RequestBuilder::send`](https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.send) синхронно выполняет созданные запросы.

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::env;
use reqwest::Client;
#
# error_chain! {
#     foreign_links {
#         EnvVar(env::VarError);
#         HttpRequest(reqwest::Error);
#     }
# }

#[derive(Deserialize, Debug)]
struct Gist {
    id: String,
    html_url: String,
}

fn main() -> Result<()> {
    let gh_user = env::var("GH_USER")?;
    let gh_pass = env::var("GH_PASS")?;

    let gist_body = json!({
        "description": "the description for this gist",
        "public": true,
        "files": {
             "main.rs": {
             "content": r#"fn main() { println!("hello world!");}"#
            }
        }});

    let request_url = "https://api.github.com/gists";
    let mut response = Client::new()
        .post(request_url)
        .basic_auth(gh_user.clone(), Some(gh_pass.clone()))
        .json(&gist_body)
        .send()?;

    let gist: Gist = response.json()?;
    println!("Created {:?}", gist);

    let request_url = format!("{}/{}",request_url, gist.id);
    let response = Client::new()
        .delete(&request_url)
        .basic_auth(gh_user, Some(gh_pass))
        .send()?;

    println!("Gist {} deleted! Status code: {}",gist.id, response.status());
    Ok(())
}
```

Данный пример использует [HTTP Basic Auth](https://tools.ietf.org/html/rfc2617) чтобы получить доступ к [GitHub API](https://developer.github.com/v3/auth/). Но в промышленном коде типичный пример использования может задействовать гораздо более сложный [OAuth](https://oauth.net/getting-started/) процесс авторизации.


