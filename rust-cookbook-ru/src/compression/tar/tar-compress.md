## Запаковка каталога в tar-архив

[![flate2-badge]][flate2] [![tar-badge]][tar] [![cat-compression-badge]][cat-compression]

Сжатие каталога `/var/log` directory в `archive.tar.gz`.

Этот пример создаёт [`File`], обёрнутый в [`GzEncoder`](https://docs.rs/flate2/*/flate2/write/struct.GzEncoder.html) и [`tar::Builder`](https://docs.rs/tar/*/tar/struct.Builder.html). Рекурсивно добавляет содержимое каталога `/var/log` в архив по пути `backup/logs` с помощью [`Builder::append_dir_all`](https://docs.rs/tar/*/tar/struct.Builder.html#method.append_dir_all).
[`GzEncoder`](https://docs.rs/flate2/*/flate2/write/struct.GzEncoder.html) ответственен за прозрачный интерфейс для сжатия данных до записи их в файл `archive.tar.gz`.

```rust,no_run
extern crate tar;
extern crate flate2;

use std::fs::File;
use flate2::Compression;
use flate2::write::GzEncoder;

fn main() -> Result<(), std::io::Error> {
    let tar_gz = File::create("archive.tar.gz")?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("backup/logs", "/var/log")?;
    Ok(())
}
```


[`File`]: https://docs.rs/tar/*/tar/struct.Builder.html#method.append_dir_all