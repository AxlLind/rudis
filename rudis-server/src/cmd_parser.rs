use rudis::{ByteString, Command};
use smol::io::{AsyncRead, AsyncReadExt};

pub struct Parser<R> {
    reader: R,
    peeked: Option<u8>,
}

impl<R: AsyncRead + Unpin> Parser<R> {
    pub fn new(reader: R) -> Self {
        Self { reader, peeked: None }
    }

    async fn read_byte(&mut self) -> anyhow::Result<u8> {
        let mut b = [0];
        self.reader.read_exact(&mut b).await?;
        Ok(b[0])
    }

    async fn peek(&mut self) -> Option<u8> {
        if self.peeked.is_none() {
            self.peeked = self.read_byte().await.ok();
        }
        self.peeked
    }

    async fn consume_byte(&mut self) -> anyhow::Result<u8> {
        if let Some(b) = self.peeked.take() {
            Ok(b)
        } else {
            self.read_byte().await
        }
    }

    async fn expect(&mut self, pat: &[u8]) -> anyhow::Result<()> {
        for &expected in pat {
            let b = self.consume_byte().await?;
            anyhow::ensure!(expected == b, "expected {:?}, got {:?}", expected as char, b as char);
        }
        Ok(())
    }

    async fn read_number(&mut self) -> anyhow::Result<usize> {
        let b = self.consume_byte().await?;
        anyhow::ensure!(b.is_ascii_digit(), "expected number, got {b}");
        let mut num = (b - b'0') as usize;
        while self.peek().await.is_some_and(|b| b.is_ascii_digit()) {
            let b = self.consume_byte().await?;
            num = num * 10 + (b - b'0') as usize;
        }
        Ok(num)
    }

    async fn read_bulk_string(&mut self) -> anyhow::Result<ByteString> {
        self.expect(b"$").await?;
        let len = self.read_number().await?;
        self.expect(b"\r\n").await?;

        let mut s = Vec::with_capacity(len);
        for _ in 0..len {
            s.push(self.consume_byte().await?);
        }
        self.expect(b"\r\n").await?;
        Ok(s)
    }

    pub async fn read_command(&mut self) -> anyhow::Result<Command> {
        self.expect(b"*").await?;
        let len = self.read_number().await?;
        self.expect(b"\r\n").await?;
        let mut args = Vec::with_capacity(len);
        for _ in 0..len {
            args.push(self.read_bulk_string().await?);
        }
        Command::new(args)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_array() {
        smol::block_on(async {
            let mut res = Parser::new(b"*2\r\n$3\r\nFOO\r\n$3\r\nbar\r\n".as_slice()).read_command().await.unwrap();
            assert_eq!(res.cmd(), "foo");
            assert_eq!(res.parse_args::<ByteString>().unwrap(), b"bar".to_vec());
        })
    }

    #[test]
    fn test_parse_empty_array() {
        smol::block_on(async {
            assert!(Parser::new(b"*0\r\n".as_slice()).read_command().await.is_err());
        })
    }

    #[test]
    fn test_parse_pipelined_arrays() {
        smol::block_on(async {
            let mut parser = Parser::new(b"*1\r\n$1\r\nA\r\n*3\r\n$4\r\nABCD\r\n$0\r\n\r\n$2\r\nxx\r\n".as_slice());
            let mut res = parser.read_command().await.unwrap();
            assert_eq!(res.cmd(), "a");
            assert!(res.parse_args::<Vec<ByteString>>().unwrap().is_empty());

            let mut res = parser.read_command().await.unwrap();
            assert_eq!(res.cmd(), "abcd");
            assert_eq!(res.parse_args::<(ByteString, ByteString)>().unwrap(), (b"".to_vec(), b"xx".to_vec()));

            assert!(parser.read_command().await.is_err())
        })
    }
}
