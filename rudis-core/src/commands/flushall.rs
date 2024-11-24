use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"flushall",
    arity: -1,
    flags: &[
        b"write",
    ],
    first_key: 0,
    last_key: 0,
    step: 0,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let arg = cmd.parse_args::<Option<ByteString>>()?;
    match arg.as_deref() {
        Some(b"SYNC") | None => {
            db.clear();
            Ok(Response::SimpleString(b"OK".to_vec()))
        },
        Some(b"ASYNC") => anyhow::bail!("async flush not implemented"),
        _ => anyhow::bail!("invalid argument"),
    }
}

// TODO: test multiple databases
#[cfg(test)]
crate::command_test! {
    "set x 1"      => "OK";
    "set y 1"      => "OK";
    "set z 1"      => "OK";
    "dbsize"       => 3;
    "flushall"     => "OK";
    "dbsize"       => 0;
    "exists x y z" => 0;
}
