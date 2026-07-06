use bzip2::read::BzDecoder;
use std::fs::File;
use std::io;

fn main() -> io::Result<()> {
    let input = File::open("input.txt.bz2")?;
    let mut decoder = BzDecoder::new(input);
    let mut output = File::create("input.txt")?;
    io::copy(&mut decoder, &mut output)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bzip2::Compression;
    use bzip2::write::BzEncoder;
    use std::io::{Read, Write};

    #[test]
    fn decodes_bzip2_compressed_bytes() -> io::Result<()> {
        let data = b"hello, bzip2!".repeat(100);

        let mut encoder = BzEncoder::new(Vec::new(), Compression::best());
        encoder.write_all(&data)?;
        let compressed = encoder.finish()?;

        let mut decoder = BzDecoder::new(compressed.as_slice());
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)?;

        assert_eq!(decompressed, data);
        Ok(())
    }
}
