use super::{CommandInfo, RedisCommand};
use crate::command::Command;
use crate::{ByteString, Database, Response};

static INFO: CommandInfo = CommandInfo {
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

pub struct Cmd;

impl RedisCommand for Cmd {
    fn info(&self) -> &'static CommandInfo { &INFO }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
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
}
