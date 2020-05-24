## Произвольный доступ к файлу с помощью карты памяти

[![memmap-badge]][memmap] [![cat-filesystem-badge]][cat-filesystem]

Создает карту памяти файла с использованием крейта [memmap] и моделирует некоторые непоследовательные чтения из файла. Использование карты памяти означает, что вы просто индексируете срезы, а не делаете с помощью [`seek`] навигацию по файлу.

Функция [`Mmap::map`](https://docs.rs/memmap/*/memmap/struct.Mmap.html#method.map) предполагает, что файл индексированный картой памяти не изменяется одновременно с другим процессом, иначе возникает [состояние гонки] .

```rust
extern crate memmap;

use memmap::Mmap;
use std::fs::File;
use std::io::{Write, Error};

fn main() -> Result<(), Error> {
#     write!(File::create("content.txt")?, "My hovercraft is full of eels!")?;
#
    let file = File::open("content.txt")?;
    let map = unsafe { Mmap::map(&file)? };

    let random_indexes = [0, 1, 2, 19, 22, 10, 11, 29];
    assert_eq!(&map[3..13], b"hovercraft");
    let random_bytes: Vec<u8> = random_indexes.iter()
        .map(|&idx| map[idx])
        .collect();
    assert_eq!(&random_bytes[..], b"My loaf!");
    Ok(())
}
```


[`seek`]: https://docs.rs/memmap/*/memmap/struct.Mmap.html#method.map
[состояние гонки]: https://doc.rust-lang.org/std/fs/struct.File.html#method.seek