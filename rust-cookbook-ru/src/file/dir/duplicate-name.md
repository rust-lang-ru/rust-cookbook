## Рекурсивно найти повторяющиеся имена файлов

[![walkdir-badge]][walkdir] [![cat-filesystem-badge]][cat-filesystem]

Найти рекурсивно в текущем каталоге дубликаты имён файлов, печатая их только один раз.

```rust,no_run
extern crate walkdir;

use std::collections::HashMap;
use walkdir::WalkDir;

fn main() {
    let mut filenames = HashMap::new();

    for entry in WalkDir::new(".")
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir()) {
        let f_name = String::from(entry.file_name().to_string_lossy());
        let counter = filenames.entry(f_name.clone()).or_insert(0);
        *counter += 1;

        if *counter == 2 {
            println!("{}", f_name);
        }
    }
}
```
