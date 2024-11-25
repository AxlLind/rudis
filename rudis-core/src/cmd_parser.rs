use std::fmt::Display;
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

impl<T: FromArgs> FromArgs for Vec<T> {
    fn from_args(cmd: &mut Command) -> anyhow::Result<Self> {
        let mut v = Vec::new();
        while cmd.has_more() {
            v.push(T::from_args(cmd)?);
        }
        Ok(v)
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
