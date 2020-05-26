## Вывод сообщения об ошибке в консоль

[![log-badge]][log] [![env_logger-badge]][env_logger] [![cat-debugging-badge]][cat-debugging]

Правильная обработка ошибок рассматривает исключительные ситуации как - гхм - исключительные. 
Здесь ошибка выводится в stderr с помощью удобного макроса [`error!`], определённого в крейте `log`.

```rust
#[macro_use]
extern crate log;
extern crate env_logger;

fn execute_query(_query: &str) -> Result<(), &'static str> {
    Err("I'm afraid I can't do that")
}

fn main() {
    env_logger::init();

    let response = execute_query("DROP TABLE students");
    if let Err(err) = response {
        error!("Failed to execute query: {}", err);
    }
}
```

[`error!`]: https://docs.rs/log/*/log/macro.error.html
