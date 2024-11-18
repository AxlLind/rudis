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
                    .map(|(_, info)| info.as_response()).unwrap_or_default())
                    .collect(),
                None => COMMAND_LIST.iter().map(|&(_, info)| info.as_response()).collect(),
            })
        },
        Some(b"list") => Response::string_array(COMMAND_LIST.iter().map(|(_, info)| info.name.to_vec())),
        Some(_) => anyhow::bail!("invalid subcommand"),
        None => Response::Array(COMMAND_LIST.iter().map(|&(_, info)| info.as_response()).collect()),
    };
    Ok(res)
}

#[cfg(test)]
crate::command_test! {
    "command count" => COMMAND_LIST.len() as i64;
}
