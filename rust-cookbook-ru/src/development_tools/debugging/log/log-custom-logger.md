## Логирование сообщений с помощью пользовательского логгера

[![log-badge]][log] [![cat-debugging-badge]][cat-debugging]

Реализует особый логгер в консоль `ConsoleLogger`, который печатает в стандартный вывод. Чтобы иметь возможность использовать макросы для логгеров, `ConsoleLogger` реализует типаж [`log::Log`](https://docs.rs/log/*/log/trait.Log.html). Вызов функции [`log::set_logger`](https://docs.rs/log/*/log/fn.set_logger.html) подключает логгер.

```rust
#[macro_use]
extern crate log;

use log::{Record, Level, Metadata, LevelFilter, SetLoggerError};

static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

struct ConsoleLogger;

impl log::Log for ConsoleLogger {
  fn enabled(&self, metadata: &Metadata) -> bool {
     metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("Rust says: {} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

fn main() -> Result<(), SetLoggerError> {
    log::set_logger(&CONSOLE_LOGGER)?;
    log::set_max_level(LevelFilter::Info);

    info!("hello log");
    warn!("warning");
    error!("oops");
    Ok(())
}
```


