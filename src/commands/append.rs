use super::{CommandInfo, RedisCommand};
use crate::command::Command;
use crate::{ByteString, Database, Response, Value};

static INFO: CommandInfo = CommandInfo {
    name: b"append",
    arity: 0,
    flags: &[],
    first_key: 1,
    last_key: 4,
    step: 5,
};

pub struct AppendCommand;

impl RedisCommand for AppendCommand {
    fn info(&self) -> &'static CommandInfo { &INFO }

    fn run(&self, db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let (key, value) = cmd.parse_args::<(ByteString, ByteString)>()?;
        let len = match db.get_str(&key)? {
            Some(v) => {
                v.extend(value);
                v.len()
            }
            None => {
                let len = value.len();
                db.set(key, Value::String(value));
                len
            }
        };
        Ok(Response::Number(len as _))
    }
}
