## Использование переменной среды для настройки логирования

[![log-badge]][log] [![env_logger-badge]][env_logger] [![cat-debugging-badge]][cat-debugging]

Структура [`Builder`](https://docs.rs/env_logger/*/env_logger/struct.Builder.html) настраивает логирование.

Сруктура [`Builder::parse`](https://docs.rs/env_logger/*/env_logger/struct.Builder.html#method.parse) анализирует содержимое переменной среды `MY_APP_LOG` в форме синтаксиса [`RUST_LOG`](https://docs.rs/env_logger/*/env_logger/#enabling-logging). Затем [`Builder::init`](https://docs.rs/env_logger/*/env_logger/struct.Builder.html#method.init) инициализирует логгер. Все эти шаги обычно выполняются внутри с помощью [`env_logger::init`](https://docs.rs/env_logger/*/env_logger/fn.init.html).

```rust
#[macro_use] extern crate log; extern crate env_logger;  use std::env; use env_logger::Builder;  fn main() {     Builder::new()         .parse(&env::var("MY_APP_LOG").unwrap_or_default())         .init();      info!("informational message");     warn!("warning message");     error!("this is an error {}", "message"); }
```


