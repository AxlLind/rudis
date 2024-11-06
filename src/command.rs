use std::fmt::Display;
use std::io::Read;
use std::collections::VecDeque;
use anyhow::Context;

use crate::{escape_bytes, ByteString};

pub trait FromArgs : Sized {
    fn from_args(cmd: &mut Command) -> anyhow::Result<Self>;
}

impl FromArgs for ByteString {
    fn from_args(cmd: &mut Command) -> anyhow::Result<Self> {
        cmd.pop_arg().ok_or(anyhow::anyhow!("too few arguments"))
    }
}

impl FromArgs for i64 {
    fn from_args(cmd: &mut Command) -> anyhow::Result<Self> {
        let v = cmd.pop_arg().ok_or(anyhow::anyhow!("too few arguments"))?;
        std::str::from_utf8(&v)
            .map_err(|_| anyhow::anyhow!("tried to parse number, got non-utf8 value"))?
            .parse()
            .map_err(|_| anyhow::anyhow!("tried to parse number, got non-numeric value"))
    }
}

impl FromArgs for Vec<ByteString> {
    fn from_args(cmd: &mut Command) -> anyhow::Result<Self> {
        Ok(std::mem::take(&mut cmd.args).into())
    }
}

impl<T: FromArgs> FromArgs for Option<T> {
    fn from_args(cmd: &mut Command) -> anyhow::Result<Self> {
        if !cmd.has_more() {
            return Ok(None);
        }
        T::from_args(cmd).map(|v| Some(v))
    }
}

impl<T1: FromArgs, T2: FromArgs> FromArgs for (T1, T2) {
    fn from_args(cmd: &mut Command) -> anyhow::Result<Self> {
        let t1 = T1::from_args(cmd)?;
        let t2 = T2::from_args(cmd)?;
        Ok((t1, t2))
    }
}

impl<T1: FromArgs, T2: FromArgs, T3: FromArgs> FromArgs for (T1, T2, T3) {
    fn from_args(cmd: &mut Command) -> anyhow::Result<Self> {
        let t1 = T1::from_args(cmd)?;
        let t2 = T2::from_args(cmd)?;
        let t3 = T3::from_args(cmd)?;
        Ok((t1, t2, t3))
    }
}

impl<T1: FromArgs, T2: FromArgs, T3: FromArgs, T4: FromArgs> FromArgs for (T1, T2, T3, T4) {
    fn from_args(cmd: &mut Command) -> anyhow::Result<Self> {
        let t1 = T1::from_args(cmd)?;
        let t2 = T2::from_args(cmd)?;
        let t3 = T3::from_args(cmd)?;
        let t4 = T4::from_args(cmd)?;
        Ok((t1, t2, t3, t4))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    cmd: String,
    args: VecDeque<ByteString>,
}

impl Command {
    pub fn new(args: Vec<ByteString>) -> anyhow::Result<Self> {
        let mut args = VecDeque::from(args);
        let arg1 = args.pop_front().context("expected non-empty command")?;
        let mut cmd = String::from_utf8(arg1).map_err(|_| anyhow::anyhow!("non-utf8 command"))?;
        cmd.make_ascii_lowercase();
        Ok(Self { cmd, args })
    }

    pub fn has_more(&self) -> bool {
        !self.args.is_empty()
    }

    pub fn cmd(&self) -> &str {
        &self.cmd
    }

    pub fn pop_arg(&mut self) -> Option<ByteString> {
        self.args.pop_front()
    }

    pub fn parse_partial_args<T: FromArgs>(&mut self) -> anyhow::Result<T> {
        T::from_args(self)
    }

    pub fn parse_args<T: FromArgs>(&mut self) -> anyhow::Result<T> {
        let res = self.parse_partial_args::<T>()?;
        anyhow::ensure!(self.args.is_empty(), "Too many arguments to {}", self.cmd());
        Ok(res)
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cmd())?;
        for arg in &self.args {
            write!(f, " {}", escape_bytes(arg))?;
        }
        Ok(())
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

    fn read_bulk_string(&mut self) -> anyhow::Result<ByteString> {
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
        self.expect(b"\r\n")?;
        let args = (0..len).map(|_| self.read_bulk_string()).collect::<anyhow::Result<_>>()?;
        Command::new(args)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_array() {
        let mut res = Parser::new(b"*2\r\n$3\r\nFOO\r\n$3\r\nbar\r\n".as_slice()).read_command().unwrap();
        assert_eq!(res.cmd, "foo");
        assert_eq!(res.parse_args::<ByteString>().unwrap(), b"bar".to_vec());
    }

    #[test]
    fn test_parse_empty_array() {
        assert!(Parser::new(b"*0\r\n".as_slice()).read_command().is_err());
    }

    #[test]
    fn test_parse_pipelined_arrays() {
        let mut parser = Parser::new(b"*1\r\n$1\r\nA\r\n*3\r\n$4\r\nABCD\r\n$0\r\n\r\n$2\r\nxx\r\n".as_slice());
        let mut res = parser.read_command().unwrap();
        assert_eq!(res.cmd, "a");
        assert!(res.parse_args::<Vec<ByteString>>().unwrap().is_empty());

        let mut res = parser.read_command().unwrap();
        assert_eq!(res.cmd, "abcd");
        assert_eq!(res.parse_args::<(ByteString, ByteString)>().unwrap(), (b"".to_vec(), b"xx".to_vec()));

        assert!(parser.read_command().is_err())
    }
}
