use std::io::Read;
use std::collections::VecDeque;
use anyhow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    cmd: Vec<u8>,
    args: VecDeque<Vec<u8>>,
}

impl Command {
    pub fn new(args: Vec<Vec<u8>>) -> Self {
        let mut args = VecDeque::from(args);
        Self { cmd: args.pop_front().unwrap(), args }
    }

    pub fn cmd(&self) -> &[u8] {
        &self.cmd
    }

    pub fn nargs(&self) -> usize {
        self.args.len()
    }

    pub fn pop_args_inexact<const N: usize>(&mut self) -> Option<[Vec<u8>; N]> {
        let mut args = std::array::from_fn(|_| Vec::new());
        for i in 0..N {
            args[i] = self.args.pop_front()?;
        }
        Some(args)
    }

    pub fn pop_args<const N: usize>(&mut self, expected_args: &str) -> anyhow::Result<[Vec<u8>; N]> {
        let args = self.pop_args_inexact();
        let res = if self.args.is_empty() {args} else {None};
        res.ok_or_else(|| anyhow::anyhow!("expected {} {}", crate::escape_bytes(self.cmd()), expected_args))
    }

    pub fn rest(self) -> impl Iterator<Item=Vec<u8>> {
        self.args.into_iter()
    }
}

pub struct Parser<R> {
    reader: R,
    peeked: Option<u8>,
}

impl<R: Read> Parser<R> {
    pub fn new(reader: R) -> Self {
        Self { reader, peeked: None }
    }

    fn read_byte(&mut self) -> anyhow::Result<u8> {
        let mut b = [0];
        self.reader.read_exact(&mut b)?;
        Ok(b[0])
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
        anyhow::ensure!(b.is_ascii_digit(), "expected number, got {b}");
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

    pub fn read_command(&mut self) -> anyhow::Result<Command> {
        self.expect(b"*")?;
        let len = self.read_number()?;
        anyhow::ensure!(len > 0, "no command given");
        self.expect(b"\r\n")?;
        let args = (0..len).map(|_| self.read_bulk_string()).collect::<anyhow::Result<_>>()?;
        Ok(Command::new(args))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_array() {
        let mut res = Parser::new(b"*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n".as_slice()).read_command().unwrap();
        assert_eq!(res.cmd, b"foo");
        assert_eq!(res.pop_args("").unwrap(), [b"bar"]);
        assert_eq!(res.nargs(), 0);
    }

    #[test]
    fn test_parse_empty_array() {
        assert!(Parser::new(b"*0\r\n".as_slice()).read_command().is_err());
    }

    #[test]
    fn test_parse_pipelined_arrays() {
        let mut parser = Parser::new(b"*1\r\n$1\r\na\r\n*3\r\n$4\r\nabcd\r\n$0\r\n\r\n$2\r\nxx\r\n".as_slice());
        let res = parser.read_command().unwrap();
        assert_eq!(res.cmd, b"a");
        assert_eq!(res.nargs(), 0);

        let mut res = parser.read_command().unwrap();
        assert_eq!(res.cmd, b"abcd");
        assert_eq!(res.pop_args("").unwrap(), [b"".to_vec(), b"xx".to_vec()]);
        assert_eq!(res.nargs(), 0);
    }
}
