## Логирование сообщений в пользовательское место

[![log-badge]][log] [![log4rs-badge]][log4rs] [![cat-debugging-badge]][cat-debugging]

Крейт [log4rs] настраивает вывод журнала в пользовательское место. [log4rs]
может использовать либо внешний файл YAML, либо программную конфигурацию.

Создайте конфигурацию логов с помощью [`log4rs::append::file::FileAppender`].
Appender определяет место назначения логирования. Конфигурация продолжается кодированием 
формата одной записи с использованием пользовательского шаблона из 
[`log4rs::encode::pattern`]. Конфигурация присваивается [`log4rs::config::Config`]
и устанавливается по умолчанию [`log::LevelFilter`].

```rust,no_run
# #[macro_use]
# extern crate error_chain;
#[macro_use]
extern crate log;
extern crate log4rs;

use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};
#
# error_chain! {
#     foreign_links {
#         Io(std::io::Error);
#         LogConfig(log4rs::config::Errors);
#         SetLogger(log::SetLoggerError);
#     }
# }

fn main() -> Result<()> {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("log/output.log")?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
                   .appender("logfile")
                   .build(LevelFilter::Info))?;

    log4rs::init_config(config)?;

    info!("Hello, world!");

    Ok(())
}
```

[`log4rs::append::file::FileAppender`]: https://docs.rs/log4rs/*/log4rs/append/file/struct.FileAppender.html
[`log4rs::config::Config`]: https://docs.rs/log4rs/*/log4rs/config/struct.Config.html
[`log4rs::encode::pattern`]: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
[`log::LevelFilter`]: https://docs.rs/log/*/log/enum.LevelFilter.html
