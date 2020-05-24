## Рекурсивно найти все файлы с заданным предикатом

[![walkdir-badge]][walkdir] [![cat-filesystem-badge]][cat-filesystem]

Найти JSON файлы, измененные за последний день в текущем каталоге. Использование метода [`follow_links`](https://docs.rs/walkdir/*/walkdir/struct.WalkDir.html#method.follow_links) гарантирует, что символические ссылки будут пройдены, как если бы они были обычными каталогами и файлами.

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate walkdir;

use walkdir::WalkDir;
#
# error_chain! {
#     foreign_links {
#         WalkDir(walkdir::Error);
#         Io(std::io::Error);
#         SystemTime(std::time::SystemTimeError);
#     }
# }

fn main() -> Result<()> {
    for entry in WalkDir::new(".")
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok()) {
        let f_name = entry.file_name().to_string_lossy();
        let sec = entry.metadata()?.modified()?;

        if f_name.ends_with(".json") && sec.elapsed()?.as_secs() < 86400 {
            println!("{}", f_name);
        }
    }

    Ok(())
}
```


