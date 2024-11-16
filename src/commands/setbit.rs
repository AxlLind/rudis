use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response, Value};

pub static INFO: CommandInfo = CommandInfo {
    name: b"setbit",
    arity: 4,
    flags: &[
        b"write",
        b"denyoom",
    ],
    first_key: 1,
    last_key: 1,
    step: 1,
};

fn set_bit(v: &mut Vec<u8>, offset: i64, value: i64) -> u8 {
    let i = (offset >> 3) as usize;
    let j = 7 - (offset & 0x7) as usize;
    if v.len() <= i {
        v.resize(i + 1, 0);
    }
    let bit = (v[i] >> j) & 1;
    if value == 0 {
        v[i] &= !(1 << j);
    } else {
        v[i] |= 1 << j;
    }
    bit
}

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let (key, offset, value) = cmd.parse_args::<(ByteString, i64, i64)>()?;
    anyhow::ensure!(offset >= 0, "offset cannot be negative");
    anyhow::ensure!(offset < u32::MAX as _, "offset larger than 2^32");
    anyhow::ensure!(value == 0 || value == 1, "invalid value");
    let bit = match db.get_str(&key)? {
        Some(s) => set_bit(s, offset, value),
        None => {
            let mut s = Vec::new();
            let bit = set_bit(&mut s, offset, value);
            db.set(key, Value::String(s));
            bit
        }
    };
    Ok(Response::Number(bit as _))
}

#[cfg(test)]
crate::command_test! {
    "setbit x 15 1" => 0;
    "strlen x"      => 2;
    "getbit x 15"   => 1;
    "setbit x 15 0" => 1;
    "getbit x 15"   => 0;
    "set x a"       => "OK";
    "setbit x 6 1"  => 0;
    "get x"         => "c";
}
