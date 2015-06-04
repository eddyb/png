extern crate inflate;
extern crate png;


fn main() {
    use std::fs::File;
    use std::io::Read;
    let files = [
        "tests/pngsuite/oi2n0g16.png",
    ];
    for file in files.iter() {
        let mut decoder = png::Decoder::new(File::open(file).unwrap());
        let (_, mut reader) = decoder.read_info().unwrap();
        let mut raw_out = Vec::new();
        let mut out = Vec::new();
        while let Some(obj) = reader.decode_next().unwrap() {
            match obj {
                png::Decoded::PartialChunk(_, data) => raw_out.extend(data.iter().map(|&v| v)),
                png::Decoded::ImageData(data) => out.extend(data.iter().map(|&v| v)),
                _ => ()
            }
        }
        // Feed the two chunks separately
        let mut stream = inflate::InflateStream::from_zlib();
        println!("data produced {}", stream.update(&raw_out[..64]).unwrap().1.len());
        println!("data produced {}", stream.update(&raw_out[64..]).unwrap().1.len());
        // Feed the two chunks at once
        let mut stream = inflate::InflateStream::from_zlib();
        println!("data produced {}", stream.update(&raw_out).unwrap().1.len());
    }
}