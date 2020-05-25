## Включить метку времени в сообщения лога

[![log-badge]][log] [![env_logger-badge]][env_logger] [![chrono-badge]][chrono] [![cat-debugging-badge]][cat-debugging]

Создаёт пользовательскую конфигурацию логгера с помощью [`Builder`](https://docs.rs/env_logger/*/env_logger/struct.Builder.html). Каждая запись в журнале вызывает [`Local::now`](https://docs.rs/chrono/*/chrono/offset/struct.Local.html#method.now) для получения текущего [`DateTime`](https://docs.rs/chrono/*/chrono/datetime/struct.DateTime.html) в местном часовом поясе и использует метод [`DateTime::format`](https://docs.rs/chrono/*/chrono/struct.DateTime.html#method.format) со [`strftime::specifiers`](https://docs.rs/chrono/*/chrono/format/strftime/index.html#specifiers) для форматирования метки времени, используемой в конечном журнале.

В примере вызывается [`Builder::format`](https://docs.rs/env_logger/*/env_logger/struct.Builder.html#method.format) для установки замыкания, которое форматирует каждый текст сообщения с отметкой времени [`Record::level`](https://docs.rs/log/*/log/struct.Record.html#method.level) и телом ([`Record::args`](https://docs.rs/log/*/log/struct.Record.html#method.args)).

```rust
#[macro_use]
extern crate log;
extern crate chrono;
extern crate env_logger;

use std::io::Write;
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;

fn main() {
    Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    warn!("warn");
    info!("info");
    debug!("debug");
}
```

вывод stderr будет содержать

```
2017-05-22T21:57:06 [WARN] - warn
2017-05-22T21:57:06 [INFO] - info
```


