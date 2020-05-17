## Скачивание файла во временный каталог

[![reqwest-badge]][reqwest] [![tempdir-badge]][tempdir] [![cat-net-badge]][cat-net] [![cat-filesystem-badge]][cat-filesystem]

Пример создаёт временный каталог с помощью [`tempfile::Builder`](https://docs.rs/tempfile/*/tempfile/struct.Builder.html) и синхронно скачивает файл через HTTP используя [`reqwest::get`](https://docs.rs/reqwest/*/reqwest/fn.get.html).

Далее создаёт целевой файл [`File`](https://doc.rust-lang.org/std/fs/struct.File.html) с именем, полученным от [`Response::url`](https://docs.rs/reqwest/*/reqwest/struct.Response.html#method.url) в каталоге [`tempdir()`](https://docs.rs/tempfile/3.1.0/tempfile/struct.Builder.html#method.tempdir) и копирует скачанные данные в этот файл посредством [`io::copy`](https://doc.rust-lang.org/std/io/fn.copy.html). Временный каталог автоматически удаляется после возврата из функции `run`.

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate reqwest;
extern crate tempfile;

use std::io::copy;
use std::fs::File;
use tempfile::Builder;
#
# error_chain! {
#     foreign_links {
#         Io(std::io::Error);
#         HttpRequest(reqwest::Error);
#     }
# }

fn main() -> Result<()> {
    let tmp_dir = Builder::new().prefix("example").tempdir()?;
    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    let mut response = reqwest::get(target)?;

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("file to download: '{}'", fname);
        let fname = tmp_dir.path().join(fname);
        println!("will be located under: '{:?}'", fname);
        File::create(fname)?
    };
    copy(&mut response, &mut dest)?;
    Ok(())
}
```


