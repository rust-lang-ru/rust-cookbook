## Вычисление SHA-256 хеша для файла

[![ring-badge]][ring] [![data-encoding-badge]][data-encoding] [![cat-cryptography-badge]][cat-cryptography]

Код в примере пишет некоторые данные в файл на диске, затем вычисляет SHA-256 [`digest::Digest`] из содержимое файла посредством [`digest::Context`].

```rust
# #[macro_use]
# extern crate error_chain;
extern crate data_encoding;
extern crate ring;

use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Read, Write};
#
# error_chain! {
#     foreign_links {
#         Io(std::io::Error);
#         Decode(data_encoding::DecodeError);
#     }
# }

fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

fn main() -> Result<()> {
    let path = "file.txt";

    let mut output = File::create(path)?;
    write!(output, "We will generate a digest of this text")?;

    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let digest = sha256_digest(reader)?;

    println!("SHA-256 digest is {}", HEXUPPER.encode(digest.as_ref()));

    Ok(())
}
```


[`digest::Context`]: https://briansmith.org/rustdoc/ring/digest/struct.Context.html
[`digest::Digest`]: https://briansmith.org/rustdoc/ring/digest/struct.Digest.html