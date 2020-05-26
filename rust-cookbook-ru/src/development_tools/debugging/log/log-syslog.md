## Логирование в Unix syslog

[![log-badge]][log] [![syslog-badge]][syslog] [![cat-debugging-badge]][cat-debugging]

Код в примере выводит сообщения в [UNIX syslog]. Инициализируется движок логирования
через [`syslog::init`]. Объект [`syslog::Facility`] регистрирует программу путём передачи
класса логирования, [`log::LevelFilter`] определяет уровень логирования, через `Option<&str>`
передаётся необязательное имя программы.

```rust
#[macro_use]
extern crate log;
# #[cfg(target_os = "linux")]
extern crate syslog;

# #[cfg(target_os = "linux")]
use syslog::{Facility, Error};

# #[cfg(target_os = "linux")]
fn main() -> Result<(), Error> {
    syslog::init(Facility::LOG_USER,
                 log::LevelFilter::Debug,
                 Some("My app name"))?;
    debug!("this is a debug {}", "message");
    error!("this is an error!");
    Ok(())
}

# #[cfg(not(target_os = "linux"))]
# fn main() {
#     println!("So far, only Linux systems are supported.");
# }
```

[`log::LevelFilter`]: https://docs.rs/log/*/log/enum.LevelFilter.html
[`syslog::Facility`]: https://docs.rs/syslog/*/syslog/enum.Facility.html
[`syslog::init`]: https://docs.rs/syslog/*/syslog/fn.init.html

[UNIX syslog]: https://www.gnu.org/software/libc/manual/html_node/Overview-of-Syslog.html
