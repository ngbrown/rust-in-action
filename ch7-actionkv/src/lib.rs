use std::io;
use std::path::Path;

type ByteString = Vec<u8>;
type ByteStr = [u8];

pub struct ActionKV {}

impl ActionKV {
    pub fn open(path: &Path) -> io::Result<Self> {
        unimplemented!();
    }

    pub fn load(&mut self) -> io::Result<()> {
        unimplemented!();
    }

    pub fn get(&mut self, key: &ByteStr) -> io::Result<Option<ByteString>> {
        unimplemented!();
    }

    pub fn insert(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<()> {
        unimplemented!();
    }

    #[inline]
    pub fn update(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<()> {
        unimplemented!();
    }

    #[inline]
    pub fn delete(&mut self, key: &ByteStr) -> io::Result<()> {
        unimplemented!();
    }
}
