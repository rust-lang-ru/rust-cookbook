## Найти зацикливания у заданного пути

[![same_file-badge]][same_file] [![cat-filesystem-badge]][cat-filesystem]

Используется [`same_file::is_same_file`](https://docs.rs/same-file/*/same_file/fn.is_same_file.html) для обнаружения зацикливания по заданному пути. Например, цикл может быть создан в системе Unix через символические ссылки:

```bash
mkdir -p /tmp/foo/bar/baz
ln -s /tmp/foo/  /tmp/foo/bar/baz/qux
```

Следующей код будет утверждать, что цикл существует.

```rust,no_run
extern crate same_file;

use std::io;
use std::path::{Path, PathBuf};
use same_file::is_same_file;

fn contains_loop<P: AsRef<Path>>(path: P) -> io::Result<Option<(PathBuf, PathBuf)>> {
    let path = path.as_ref();
    let mut path_buf = path.to_path_buf();
    while path_buf.pop() {
        if is_same_file(&path_buf, path)? {
            return Ok(Some((path_buf, path.to_path_buf())));
        } else if let Some(looped_paths) = contains_loop(&path_buf)? {
            return Ok(Some(looped_paths));
        }
    }
    return Ok(None);
}

fn main() {
    assert_eq!(
        contains_loop("/tmp/foo/bar/baz/qux/bar/baz").unwrap(),
        Some((
            PathBuf::from("/tmp/foo"),
            PathBuf::from("/tmp/foo/bar/baz/qux")
        ))
    );
}
```


