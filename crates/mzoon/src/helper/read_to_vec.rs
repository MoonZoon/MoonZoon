use std::io::Read;
use anyhow::Result;

pub trait ReadToVec: Read {
    fn read_to_vec(&mut self) -> Result<Vec<u8>> {
        let mut vec = Vec::new();
        self.read_to_end(&mut vec)?;
        Ok(vec)
    }
}
impl<T: Read> ReadToVec for T {}
