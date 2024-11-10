use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"sunion",
    arity: -2,
    flags: &[
        b"readonly",
    ],
    first_key: 1,
    last_key: -1,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let (key, keys) = cmd.parse_args::<(ByteString, Vec<ByteString>)>()?;
    let Some(mut set) = db.get_set(&key)?.cloned() else { return Ok(Response::Array(Vec::new())) };
    for k in keys {
        let Some(s) = db.get_set(&k)? else { continue };
        for m in s.iter() {
            if !set.contains(m) {
                set.insert(m.clone());
            }
        }
    }
    let mut elems = set.into_iter().collect::<Vec<_>>();
    elems.sort();
    Ok(Response::Array(elems))
}
