## Чтение строк из файла

[![std-badge]][std] [![cat-filesystem-badge]][cat-filesystem]

Записывает в файл сообщение из трёх строк, а затем читает его обратно по строчке за раз с помощью итератора [`Lines`], созданного с помощью [`BufRead::lines`]. [`File`] реализует [`Read`], который предоставляет типаж [`BufReader`]. Метод [`File::create`] открывает [`File`](https://doc.rust-lang.org/std/fs/struct.File.html) для записи, а [`File::open`] для чтения.

```rust
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "lines.txt";

    let mut output = File::create(path)?;
    write!(output, "Rust\n💖\nFun")?;

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}
```


[`BufRead::lines`]: https://doc.rust-lang.org/std/io/trait.BufRead.html#method.lines
[`BufReader`]: https://doc.rust-lang.org/std/io/trait.BufRead.html
[`File::create`]: https://doc.rust-lang.org/std/io/struct.BufReader.html
[`File::open`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.create
[`File`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.open
[`Lines`]: https://doc.rust-lang.org/std/fs/struct.File.html
[`Read`]: https://doc.rust-lang.org/std/io/struct.Lines.html