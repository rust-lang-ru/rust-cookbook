## Постоянная обработка вывода дочернего процесса

[![std-badge]][std] [![cat-os-badge]][cat-os]

В разделе [Выполнить внешнюю команду и обработать её стандартный вывод](#run-an-external-command-and-process-stdout) обработка не начинается до тех пор, пока не завершится внешняя [`Command`]. Приведенный ниже рецепт вызывает [`Stdio::piped`] для создания канала и непрерывно читает стандартный `stdout` как только обновляется [`BufReader`] .

Приведённый ниже рецепт эквивалентен команде оболочки Unix `journalctl | grep usb`.

```rust,no_run
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, Error, ErrorKind};

fn main() -> Result<(), Error> {
    let stdout = Command::new("journalctl")
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other,"Could not capture standard output."))?;

    let reader = BufReader::new(stdout);

    reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| line.find("usb").is_some())
        .for_each(|line| println!("{}", line));

     Ok(())
}
```


[`BufReader`]: https://doc.rust-lang.org/std/io/struct.BufReader.html
[`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
[`Stdio::piped`]: https://doc.rust-lang.org/std/process/struct.Stdio.html
