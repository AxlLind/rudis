use std::io::Read;
use anyhow;

struct Parser<R> {
    reader: R,
    peeked: Option<u8>,
}

impl<R: Read> Parser<R> {
    fn new(reader: R) -> Self {
        Self { reader, peeked: None }
    }

    fn read_byte(&mut self) -> anyhow::Result<u8> {
        let mut b = 0;
        self.reader.read_exact(std::slice::from_mut(&mut b))?;
        Ok(b)
    }

    fn peek(&mut self) -> Option<u8> {
        if self.peeked.is_none() {
            self.peeked = self.read_byte().ok();
        }
        self.peeked
    }

    fn consume_byte(&mut self) -> anyhow::Result<u8> {
        if let Some(b) = self.peeked.take() {
            Ok(b)
        } else {
            self.read_byte()
        }
    }

    fn expect(&mut self, pat: &[u8]) -> anyhow::Result<()> {
        for &expected in pat {
            let b = self.consume_byte()?;
            anyhow::ensure!(expected == b, "expected {expected}, got {b}");
        }
        Ok(())
    }

    fn read_number(&mut self) -> anyhow::Result<usize> {
        let b = self.consume_byte()?;
        anyhow::ensure!(b.is_ascii_digit(), "expected number, got {:?}", b);
        let mut num = (b - b'0') as usize;
        while self.peek().is_some_and(|b| b.is_ascii_digit()) {
            let b = self.consume_byte()?;
            num = num * 10 + (b - b'0') as usize;
        }
        Ok(num)
    }

    fn read_bulk_string(&mut self) -> anyhow::Result<Vec<u8>> {
        self.expect(b"$")?;
        let len = self.read_number()?;
        self.expect(b"\r\n")?;

        let vec = (0..len).map(|_| self.consume_byte()).collect::<Result<Vec<_>,_>>()?;
        self.expect(b"\r\n")?;
        Ok(vec)
    }

    fn read_string_array(&mut self) -> anyhow::Result<Vec<Vec<u8>>> {
        self.expect(b"*")?;
        let len = self.read_number()?;
        self.expect(b"\r\n")?;
        (0..len).map(|_| self.read_bulk_string()).collect()
    }
}

fn main() -> anyhow::Result<()> {
    let mut parser = Parser::new(b"*2\r\n$4\r\nLLEN\r\n$6\r\nmylist\r\n".as_slice());
    let args = parser.read_string_array()?;
    match args[0].as_slice() {
        b"LLEN" => {
            println!("LLEN {:?}", args[1]);
        },
        cmd => anyhow::bail!("unknown command {:?}", cmd),
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_array() {
        let res = Parser::new(b"*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n".as_slice()).read_string_array().unwrap();
        assert_eq!(&res, &[b"foo", b"bar"]);
    }

    #[test]
    fn test_parse_empty_array() {
        let res = Parser::new(b"*0\r\n".as_slice()).read_string_array().unwrap();
        assert!(res.is_empty());
    }

    #[test]
    fn test_parse_pipelined_arrays() {
        let mut parser = Parser::new(b"*1\r\n$1\r\na\r\n*3\r\n$4\r\nabcd\r\n$0\r\n\r\n$2\r\nxx\r\n".as_slice());
        let res = parser.read_string_array().unwrap();
        assert_eq!(res, &[b"a"]);
        let res = parser.read_string_array().unwrap();
        assert_eq!(res, &[b"abcd".as_slice(), b"".as_slice(), b"xx".as_slice()]);
    }
}
