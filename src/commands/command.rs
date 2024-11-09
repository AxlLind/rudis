use super::{CommandInfo, RedisCommand, COMMAND_LIST};
use crate::command::Command;
use crate::{ByteString, Database, Response};

static INFO: CommandInfo = CommandInfo {
    name: b"command",
    arity: 0,
    flags: &[],
    first_key: 1,
    last_key: 4,
    step: 5,
};

pub struct Cmd;

impl RedisCommand for Cmd {
    fn info(&self) -> &'static CommandInfo { &INFO }

    fn run(&self, _: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
        let subcommand = cmd.parse_partial_args::<Option<ByteString>>()?;
        let res = match subcommand.as_deref() {
            Some(b"COUNT") => Response::Number(COMMAND_LIST.len() as _),
            Some(b"DOCS") => anyhow::bail!("unimplemented"),
            Some(b"GETKEYS") => anyhow::bail!("unimplemented"),
            Some(b"GETKEYSANDFLAGS") => anyhow::bail!("unimplemented"),
            Some(b"INFO") => anyhow::bail!("unimplemented"),
            Some(b"LIST") => anyhow::bail!("unimplemented"),
            Some(_) => anyhow::bail!("invalid subcommand"),
            None => anyhow::bail!("unimplemented"),
        };
        Ok(res)
    }
}
