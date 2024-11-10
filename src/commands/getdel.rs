use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"getdel",
    arity: 2,
    flags: &[
        b"write",
        b"fast",
    ],
    first_key: 1,
    last_key: 1,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let (src, dst) = cmd.parse_args::<(ByteString, ByteString)>()?;
    let value = match db.get(&src) {
        Some(v) => {
            let copy = v.clone();
            db.set(dst, copy);
            Response::Number(1)
        }
        None => Response::Number(0),
    };
    Ok(value)
}
