pub struct VarIntIterator<'a> {
    source: &'a [i8],
    index: usize,
}

impl<'a> VarIntIterator<'a> {
    pub fn new(source: &'a [i8]) -> Self {
        VarIntIterator { source, index: 0 }
    }
}

impl<'a> Iterator for VarIntIterator<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut value = 0u32;
        let mut bits_read = 0;
        let mut bytes_consumed = 0;

        loop {
            if self.index >= self.source.len() {
                return if bytes_consumed == 0 {
                    None
                } else {
                    panic!("Ran out of bytes while reading VarInt");
                };
            }

            let next_byte = self.source[self.index] as u8;
            self.index += 1;
            bytes_consumed += 1;

            value |= (next_byte as u32 & 0x7F) << bits_read;
            bits_read += 7;

            if (next_byte & 0x80) == 0 {
                return Some(value);
            }

            if bits_read > 35 {
                panic!("VarInt too big (max 5 bytes)");
            }
        }
    }
}
