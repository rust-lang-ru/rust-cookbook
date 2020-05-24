## Имена файлов, которые были изменены за последние 24 часа

[![std-badge]][std] [![cat-filesystem-badge]][cat-filesystem]

Получение текущего рабочего каталога, вызывая [`env::current_dir`](https://doc.rust-lang.org/std/env/fn.current_dir.html), затем для каждой записи в [`fs::read_dir`](https://doc.rust-lang.org/std/fs/fn.read_dir.html) извлекаем [`DirEntry::path`](https://doc.rust-lang.org/std/fs/struct.DirEntry.html#method.path) и получаем метаданные с помощью [`fs::Metadata`](https://doc.rust-lang.org/std/fs/struct.Metadata.html). Метод [`Metadata::modified`](https://doc.rust-lang.org/std/fs/struct.Metadata.html#method.modified) возвращает время  [`SystemTime::elapsed`](https://doc.rust-lang.org/std/time/struct.SystemTime.html#method.elapsed) с момента последней модификации. Метод [`Duration::as_secs`](https://doc.rust-lang.org/std/time/struct.Duration.html#method.as_secs) преобразует время в секунды и сравнивается с 24 часами (24 * 60 * 60 секунд). Метод [`Metadata::is_file`](https://doc.rust-lang.org/std/fs/struct.Metadata.html#method.is_file) отфильтровывает каталоги.

```rust
# #[macro_use]
# extern crate error_chain;
#
use std::{env, fs};

# error_chain! {
#     foreign_links {
#         Io(std::io::Error);
#         SystemTimeError(std::time::SystemTimeError);
#     }
# }
#
fn main() -> Result<()> {
    let current_dir = env::current_dir()?;
    println!(
        "Entries modified in the last 24 hours in {:?}:",
        current_dir
    );

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        let metadata = fs::metadata(&path)?;
        let last_modified = metadata.modified()?.elapsed()?.as_secs();

        if last_modified < 24 * 3600 && metadata.is_file() {
            println!(
                "Last modified: {:?} seconds, is read only: {:?}, size: {:?} bytes, filename: {:?}",
                last_modified,
                metadata.permissions().readonly(),
                metadata.len(),
                path.file_name().ok_or("No filename")?
            );
        }
    }

    Ok(())
}
```


