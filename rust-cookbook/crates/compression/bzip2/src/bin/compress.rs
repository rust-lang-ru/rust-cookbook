use bzip2::Compression;
use bzip2::write::BzEncoder;
use std::fs::File;
use std::io;

fn main() -> io::Result<()> {
    let mut input = File::open("input.txt")?;
    let output = File::create("input.txt.bz2")?;
    let mut encoder = BzEncoder::new(output, Compression::best());
    io::copy(&mut input, &mut encoder)?;
    encoder.finish()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bzip2::read::BzDecoder;
    use std::io::{Read, Write};

    #[test]
    fn round_trips_through_bzip2() -> io::Result<()> {
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
