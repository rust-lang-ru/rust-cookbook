## Отправить POST-запросом файл в paste-rs

[![reqwest-badge]][reqwest] [![cat-net-badge]][cat-net]

[`reqwest::Client`](https://docs.rs/reqwest/*/reqwest/struct.Client.html) устанавливает соединение с https://paste.rs в соответствии с шаблоном [`reqwest::RequestBuilder`](https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html). Вызов [`Client::post`](https://docs.rs/reqwest/*/reqwest/struct.Client.html#method.post) с передачей туда URL-адреса устанавливает связь с пунктом назначения, а [`RequestBuilder::body`](https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.body) устанавливает тело запроса для отсылки через чтение из файла, и, наконец, [`RequestBuilder::send`](https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.send) блокирует выполнение до тех пор, пока файл не отправится и не вернётся ответ.  [`read_to_string`](https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_string) возвращает ответ и выводит его на экран в консоль.

```rust,no_run
extern crate reqwest;

# #[macro_use]
# extern crate error_chain;
#
use std::fs::File;
use std::io::Read;
use reqwest::Client;
#
# error_chain! {
#     foreign_links {
#         HttpRequest(reqwest::Error);
#         IoError(::std::io::Error);
#     }
# }

fn main() -> Result<()> {
    let paste_api = "https://paste.rs";
    let file = File::open("message")?;

    let mut response = Client::new().post(paste_api).body(file).send()?;
    let mut response_body = String::new();
    response.read_to_string(&mut response_body)?;
    println!("Your paste is located at: {}", response_body);
    Ok(())
}
```


