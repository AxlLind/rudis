use super::CommandInfo;
use crate::command::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"lolwut",
    arity: -1,
    flags: &[
        b"readonly",
        b"fast",
    ],
    first_key: 0,
    last_key: 0,
    step: 0,
};

const RUDIS_LOGO: &str = r"
__________          .___.__
\______   \__ __  __| _/|__| ______
 |       _/  |  \/ __ | |  |/  ___/
 |    |   \  |  / /_/ | |  |\___ \
 |____|_  /____/\____ | |__/____  >
        \/           \/         \/
";

fn get_reply() -> String {
    format!("{}\n{}\n", RUDIS_LOGO.trim(), env!("CARGO_PKG_VERSION"))
}

pub fn run(_: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let _ = cmd.parse_args::<Option<(ByteString, ByteString)>>()?;
    Ok(Response::BulkString(get_reply().into_bytes()))
}

#[cfg(test)]
crate::command_test! {
    "lolwut" => get_reply().as_str();
    "lolwut VERSION 0.1.0" => get_reply().as_str();
}
