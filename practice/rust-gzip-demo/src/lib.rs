use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::prelude::*;

pub fn test_gzip(num: u32) {
    let mut i = 0;
    while i < num {
        let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
        e.write_all(b"foo").unwrap();
        e.write_all(b"bar").unwrap();
        let compressed_bytes = e.finish().unwrap();
        // println!("{} {:?}", i, String::from_utf8_lossy(&compressed_bytes));
        let mut d = ZlibDecoder::new(&compressed_bytes[..]);
        let mut s = String::new();
        d.read_to_string(&mut s).unwrap();
        assert_eq!(s, "foobar");
        i += 1;
    }
}
