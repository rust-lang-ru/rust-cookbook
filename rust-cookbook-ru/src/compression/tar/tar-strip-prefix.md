## Распаковка tar-архива с удалением префиксов путей

[![flate2-badge]][flate2] [![tar-badge]][tar] [![cat-compression-badge]][cat-compression]

Код в примере итерируется с помощью [`Archive::entries`](https://docs.rs/tar/*/tar/struct.Archive.html#method.entries). Мы используем [`Path::strip_prefix`], чтобы удалить заданный префикс (`bundle/logs`). Наконец, распаковываем каждый [`tar::Entry`](https://docs.rs/tar/*/tar/struct.Entry.html) с помощью [`Entry::unpack`](https://docs.rs/tar/*/tar/struct.Entry.html#method.unpack).

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate flate2;
extern crate tar;

use std::fs::File;
use std::path::PathBuf;
use flate2::read::GzDecoder;
use tar::Archive;
# 
# error_chain! {
#   foreign_links {
#     Io(std::io::Error);
#     StripPrefixError(::std::path::StripPrefixError);
#   }
# }

fn main() -> Result<()> {
    let file = File::open("archive.tar.gz")?;
    let mut archive = Archive::new(GzDecoder::new(file));
    let prefix = "bundle/logs";

    println!("Extracted the following files:");
    archive
        .entries()?
        .filter_map(|e| e.ok())
        .map(|mut entry| -> Result<PathBuf> {
            let path = entry.path()?.strip_prefix(prefix)?.to_owned();
            entry.unpack(&path)?;
            Ok(path)
        })
        .filter_map(|e| e.ok())
        .for_each(|x| println!("> {}", x.display()));

    Ok(())
}
```


[`Path::strip_prefix`]: https://docs.rs/tar/*/tar/struct.Archive.html#method.entries