## Рекурсивно рассчитать размеры файлов на заданной глубине

[![walkdir-badge]][walkdir] [![cat-filesystem-badge]][cat-filesystem]

Глубина рекурсии может быть гибко установлена методами [`WalkDir::min_depth`](https://docs.rs/walkdir/*/walkdir/struct.WalkDir.html#method.min_depth) и [`WalkDir::max_depth`](https://docs.rs/walkdir/*/walkdir/struct.WalkDir.html#method.max_depth). Вычисление суммы всех размеров файлов до глубины под папок в 3 уровня, игнорируя файлы в корневой папке.

```rust
extern crate walkdir;

use walkdir::WalkDir;

fn main() {
    let total_size = WalkDir::new(".")
        .min_depth(1)
        .max_depth(3)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.metadata().ok())
        .filter(|metadata| metadata.is_file())
        .fold(0, |acc, m| acc + m.len());

    println!("Total size: {} bytes.", total_size);
}
```


