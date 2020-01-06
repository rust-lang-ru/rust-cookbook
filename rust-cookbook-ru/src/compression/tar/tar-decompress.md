## Распаковка tar-архива

[![flate2-badge]][flate2] [![tar-badge]][tar] [![cat-compression-badge]][cat-compression]

Пример разжимает с помощью ([`GzDecoder`](https://docs.rs/flate2/*/flate2/read/struct.GzDecoder.html)) и распаковывает с помощью ([`Archive::unpack`](https://docs.rs/tar/*/tar/struct.Archive.html#method.unpack)) все файлы из сжатого tar-архива `archive.tar.gz` в текущем рабочем каталоге. Распаковка осуществляется в том же самом каталоге.

```rust,no_run
extern crate flate2;
extern crate tar;

use std::fs::File;
use flate2::read::GzDecoder;
use tar::Archive;

fn main() -> Result<(), std::io::Error> {
    let path = "archive.tar.gz";

    let tar_gz = File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(".")?;

    Ok(())
}
```


