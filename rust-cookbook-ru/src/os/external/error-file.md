## Перенаправить stdout и stderr дочернего процесса в один файл

[![std-badge]][std] [![cat-os-badge]][cat-os]

Создает дочерний процесс и перенаправляет `stdout` и `stderr` в один и тот же файл. Рецепт следует той же идее, что и [запуск внешних команд цепочкой](#run-piped-external-commands) , однако [`process::Stdio`] записывает в указанный файл. [`File::try_clone`] ссылается на один и тот же дескриптор файла для `stdout` и `stderr`. Это гарантирует, что оба дескриптора пишут с одинаковой позиции курсора.

Приведенный ниже рецепт эквивалентен запуску команды оболочки Unix `ls . oops >out.txt 2>&1`.

```rust,no_run
use std::fs::File;
use std::io::Error;
use std::process::{Command, Stdio};

fn main() -> Result<(), Error> {
    let outputs = File::create("out.txt")?;
    let errors = outputs.try_clone()?;

    Command::new("ls")
        .args(&[".", "oops"])
        .stdout(Stdio::from(outputs))
        .stderr(Stdio::from(errors))
        .spawn()?
        .wait_with_output()?;

    Ok(())
}
```


[`File::try_clone`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.try_clone
[`process::Stdio`]: https://doc.rust-lang.org/std/process/struct.Stdio.html