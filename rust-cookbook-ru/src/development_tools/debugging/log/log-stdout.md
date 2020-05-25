## Вывод в stdout вместо stderr

[![log-badge]][log] [![env_logger-badge]][env_logger] [![cat-debugging-badge]][cat-debugging]

Пример создаёт особую конфигурацию логгера используя [`Builder::target`](https://docs.rs/env_logger/*/env_logger/struct.Builder.html#method.target), чтобы установить стандартный вывод для логирования в [`Target::Stdout`](https://docs.rs/env_logger/*/env_logger/fmt/enum.Target.html).

```rust
#[macro_use]
extern crate log;
extern crate env_logger;

use env_logger::{Builder, Target};

fn main() {
    Builder::new()
        .target(Target::Stdout)
        .init();

    error!("This error has been printed to Stdout");
}
```


