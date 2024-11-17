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
        Response::string_array(info.flags.iter().map(|s| s.to_vec())),
        Response::Number(info.first_key),
        Response::Number(info.last_key),
        Response::Number(info.step),
    ])
}

pub fn run(_: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let mut subcommand = cmd.parse_partial_args::<Option<ByteString>>()?;
    if let Some(cmd) = &mut subcommand {
        cmd.make_ascii_lowercase();
    }
    let res = match subcommand.as_deref() {
        Some(b"count") => Response::Number(COMMAND_LIST.len() as _),
        Some(b"docs") => anyhow::bail!("unimplemented"),
        Some(b"getkeys") => anyhow::bail!("unimplemented"),
        Some(b"getkeysandflags") => anyhow::bail!("unimplemented"),
        Some(b"info") => {
            let cmds = cmd.parse_args::<Option<Vec<ByteString>>>()?;
            Response::Array(match cmds {
                Some(cmds) => cmds.iter()
                    .map(|c| COMMANDS.get(c.as_slice())
                    .map(|(_, info)| info_response(info)).unwrap_or_default())
                    .collect(),
                None => COMMAND_LIST.iter().map(|&(_, info)| info_response(info)).collect(),
            })
        },
        Some(b"list") => anyhow::bail!("unimplemented"),
        Some(_) => anyhow::bail!("invalid subcommand"),
        None => Response::Array(COMMAND_LIST.iter().map(|&(_, info)| info_response(info)).collect()),
    };
    Ok(res)
}

#[cfg(test)]
crate::command_test! {
    "command count" => COMMAND_LIST.len() as i64;
}
