## Разбор аргументов командной строки

[![clap-badge]][clap] [![cat-command-line-badge]][cat-command-line]

Это приложение описывает структуру своего интерфейса командной строки используя стиль цепочек вызовов из `clap`. [Документация] даёт два других возможных способа для построения приложения.

В стиле цепочек вызовов (builder style), `with_name` это уникальный идентификатор, который использует `value_of` для получения переданного значения. Опции `short` и `long` контролируют флаг, который пользователь будет использовать и печатать в командной строке; короткие флаги выглядят как `-f`, а длинные флаги выглядят как `--file`.

```rust
extern crate clap;

use clap::{Arg, App};

fn main() {
    let matches = App::new("My Test Program")
        .version("0.1.0")
        .author("Hackerman Jones <hckrmnjones@hack.gov>")
        .about("Teaches argument parsing")
        .arg(Arg::with_name("file")
                 .short("f")
                 .long("file")
                 .takes_value(true)
                 .help("A cool file"))
        .arg(Arg::with_name("num")
                 .short("n")
                 .long("number")
                 .takes_value(true)
                 .help("Five less than your favorite number"))
        .get_matches();

    let myfile = matches.value_of("file").unwrap_or("input.txt");
    println!("The file passed is: {}", myfile);

    let num_str = matches.value_of("num");
    match num_str {
        None => println!("No idea what your favorite number is."),
        Some(s) => {
            match s.parse::<i32>() {
                Ok(n) => println!("Your favorite number must be {}.", n + 5),
                Err(_) => println!("That's not a number! {}", s),
            }
        }
    }
}
```

Документация по интерфейсу командной строки генерируется непосредственно библиотекой `clap`. Для данного приложения вывод выглядит примерно следующим образом.

```
My Test Program 0.1.0
Hackerman Jones <hckrmnjones@hack.gov>
Teaches argument parsing

USAGE:
    testing [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <file>     A cool file
    -n, --number <num>    Five less than your favorite number
```

Мы можем протестировать приложение запустив команду следующим образом.

```
$ cargo run -- -f myfile.txt -n 251
```

Вывод команды:

```
The file passed is: myfile.txt
Your favorite number must be 256.
```


[Документация]: https://docs.rs/clap/