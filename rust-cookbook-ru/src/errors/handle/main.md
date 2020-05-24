## Правильная обработка ошибки в main функции

[![error-chain-badge]][error-chain] [![cat-rust-patterns-badge]][cat-rust-patterns]

Обрабатывает ошибку, возникающую при попытке открыть файл, который не существует. Это достигается с помощью библиотеки [error-chain], которая заботится об автоматической генерации большого количества стандартного кода, необходимого для [обработки ошибок в Rust](https://doc.rust-lang.org/book/second-edition/ch09-00-error-handling.html) .

`Io(std::io::Error)` внутри [`foreign_links`](https://docs.rs/error-chain/*/error_chain/#foreign-links) позволяет автоматическое преобразование из [`std::io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html) в объявленный тип в [`error_chain!`](https://docs.rs/error-chain/*/error_chain/macro.error_chain.html), реализующий типаж [`Error`](https://doc.rust-lang.org/std/error/trait.Error.html).

Приведённый ниже рецепт расскажет, как долго работает система, открыв файл Unix `/proc/uptime` и проанализировав его содержимое, получив первое число. Он возвращает время безотказной работы, если нет ошибки.

Другие рецепты в этой книге скрывают сгенерированный код [error-chain], его можно увидеть, развернув код с помощью кнопки ⤢.

```rust
#[macro_use]
extern crate error_chain;

use std::fs::File;
use std::io::Read;

error_chain!{
    foreign_links {
        Io(std::io::Error);
        ParseInt(::std::num::ParseIntError);
    }
}

fn read_uptime() -> Result<u64> {
    let mut uptime = String::new();
    File::open("/proc/uptime")?.read_to_string(&mut uptime)?;

    Ok(uptime
        .split('.')
        .next()
        .ok_or("Cannot parse uptime data")?
        .parse()?)
}

fn main() {
    match read_uptime() {
        Ok(uptime) => println!("uptime: {} seconds", uptime),
        Err(err) => eprintln!("error: {}", err),
    };
}
```

