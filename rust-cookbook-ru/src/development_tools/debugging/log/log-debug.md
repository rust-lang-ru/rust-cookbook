## Вывод отладочного сообщения в консоль

[![log-badge]][log] [![env_logger-badge]][env_logger] [![cat-debugging-badge]][cat-debugging]

Крейт `log` обеспечивает инструменты для логирования. Крейт `env_logger` конфигурирует логирование с помощью переменных окружения. Макрос [`debug!`](https://docs.rs/log/*/log/macro.debug.html) работает подобно другим макросам, принимая строки форматирования по образцу [`std::fmt`].

```rust
#[macro_use]
extern crate log;
extern crate env_logger;

fn execute_query(query: &str) {
    debug!("Executing query: {}", query);
}

fn main() {
    env_logger::init();

    execute_query("DROP TABLE students");
}
```

Ничего не выводится после запуска этого кода. По умолчанию, уровень логикрования установлен в `error`, и все сообщения более низкого уровня отбрасываются.

Установка переменной окружения `RUST_LOG` включает печать:

```
$ RUST_LOG=debug cargo run
```

Сначала Cargo печатает отладочную информацию, и затем будет напечатана строка в самом конце вывода на экран:

```
DEBUG:main: Executing query: DROP TABLE students
```


[`std::fmt`]: https://docs.rs/log/*/log/macro.debug.html