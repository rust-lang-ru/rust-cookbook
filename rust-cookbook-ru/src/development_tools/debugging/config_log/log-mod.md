## Включение уровней логирования для каждого модуля

[![log-badge]][log] [![env_logger-badge]][env_logger] [![cat-debugging-badge]][cat-debugging]

Крейты с двымя модулями `foo` и вложенным `foo::bar` с директивами логирования, управляемыми отдельно с помощью переменной среды [`RUST_LOG`](https://docs.rs/env_logger/*/env_logger/#enabling-logging).

```rust
#[macro_use] extern crate log; extern crate env_logger;  mod foo {     mod bar {         pub fn run() {             warn!("[bar] warn");             info!("[bar] info");             debug!("[bar] debug");         }     }      pub fn run() {         warn!("[foo] warn");         info!("[foo] info");         debug!("[foo] debug");         bar::run();     } }  fn main() {     env_logger::init();     warn!("[root] warn");     info!("[root] info");     debug!("[root] debug");     foo::run(); }
```

Переменная среды [`RUST_LOG`](https://docs.rs/env_logger/*/env_logger/#enabling-logging) управляет выводом [`env_logger`][env_logger] в [env_logger]. Объявления модуля принимают разделенные запятыми записи, отформатированные как `path::to::module=log_level`. Запустите приложение `test` следующим образом:

```bash
RUST_LOG="warn,test::foo=info,test::foo::bar=debug" ./test
```

Устанавливает по умолчанию [`log::Level`](https://docs.rs/log/*/log/enum.Level.html) в `warn`, модуль `foo` и модуль `foo::bar` в `info` и `debug` .

```bash
WARN:test: [root] warn WARN:test::foo: [foo] warn INFO:test::foo: [foo] info WARN:test::foo::bar: [bar] warn INFO:test::foo::bar: [bar] info DEBUG:test::foo::bar: [bar] debug
```


