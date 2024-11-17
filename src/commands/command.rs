use super::{CommandInfo, COMMANDS, COMMAND_LIST};
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"command",
    arity: -1,
    flags: &[
        b"loading",
        b"stale",
    ],
    first_key: 0,
    last_key: 0,
    step: 0,
};

fn info_response(info: &CommandInfo) -> Response {
    Response::Array(vec![
        Response::BulkString(info.name.to_vec()),
        Response::Number(info.arity),
        Response::string_array(info.flags.iter().map(|s| s.to_vec()).collect()),
        Response::Number(info.first_key),
        Response::Number(info.last_key),
        Response::Number(info.step),
    ])
}

pub fn run(_: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let subcommand = cmd.parse_partial_args::<Option<ByteString>>()?;
    let res = match subcommand.as_deref() {
        Some(b"COUNT") => Response::Number(COMMAND_LIST.len() as _),
        Some(b"DOCS") => anyhow::bail!("unimplemented"),
        Some(b"GETKEYS") => anyhow::bail!("unimplemented"),
        Some(b"GETKEYSANDFLAGS") => anyhow::bail!("unimplemented"),
        Some(b"INFO") => {
            let cmds = cmd.parse_args::<Option<Vec<ByteString>>>()?;
            Response::Array(match cmds {
                Some(cmds) => cmds.iter().map(|c| COMMANDS[c.as_slice()].1).map(info_response).collect(),
                None => COMMAND_LIST.iter().map(|&(_, info)| info).map(info_response).collect(),
            })
        },
        Some(b"LIST") => anyhow::bail!("unimplemented"),
        Some(_) => anyhow::bail!("invalid subcommand"),
        None => Response::Array(COMMAND_LIST.iter().map(|&(_, info)| info_response(info)).collect()),
    };
    Ok(res)
}

#[cfg(test)]
crate::command_test! {
    "command COUNT" => COMMAND_LIST.len() as i64;
}
