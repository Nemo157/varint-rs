use std::io;

trait ReadHelper {
    fn read_u8(&mut self) -> io::Result<u8>;
}

pub trait ReadVarInt {
    fn read_u64_varint(&mut self) -> io::Result<u64>;
}

impl<R: io::Read> ReadHelper for R {
    fn read_u8(&mut self) -> io::Result<u8> {
        let mut buffer = [0];
        try!(self.read_exact(&mut buffer));
        Ok(buffer[0])
    }
}

impl<R: io::Read> ReadVarInt for R {
    fn read_u64_varint(&mut self) -> io::Result<u64> {
        let mut result = 0;
        let mut offset = 0;

        loop {
            let current = try!(self.read_u8());
            result = result + (((current & 0x7F) as u64) << offset);
            if current & 0x80 == 0 {
                return Ok(result);
            }
            offset += 7;
            if offset == 63 {
                let last = try!(self.read_u8());
                if last == 0x01 {
                    return Ok(result + (1 << offset));
                } else {
                    return Err(io::Error::new(
                            io::ErrorKind::Other,
                            "varint exceeded 64 bits long"));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use { ReadVarInt };

    #[test]
    fn zero() {
        let mut bytes: &[u8] = &[0];
        assert_eq!(bytes.read_u64_varint().unwrap(), 0);
    }

    #[test]
    fn one() {
        let mut bytes: &[u8] = &[1];
        assert_eq!(bytes.read_u64_varint().unwrap(), 1);
    }

    #[test]
    fn some() {
        let mut bytes: &[u8] = &[0xAC, 0x02];
        assert_eq!(bytes.read_u64_varint().unwrap(), 0x12C);
    }

    #[test]
    fn many() {
        let mut bytes: &[u8] = &[0xB5, 0xFF, 0xAC, 0x02];
        assert_eq!(bytes.read_u64_varint().unwrap(), 0x4B3FB5);
    }

    #[test]
    fn half() {
        let mut bytes: &[u8] = &[
            0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF,
            0x7F,
        ];
        assert_eq!(bytes.read_u64_varint().unwrap(), 0x7FFFFFFFFFFFFFFF);
    }

    #[test]
    fn all() {
        let mut bytes: &[u8] = &[
            0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0x01,
        ];
        assert_eq!(bytes.read_u64_varint().unwrap(), 0xFFFFFFFFFFFFFFFF);
    }

    #[test]
    fn too_many() {
        let mut bytes: &[u8] = &[
            0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0x02,
        ];
        assert!(bytes.read_u64_varint().is_err());
    }
}
