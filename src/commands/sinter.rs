use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"sinter",
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
        set.retain(|e| s.contains(e));
    }
    let mut elems = set.into_iter().collect::<Vec<_>>();
    elems.sort();
    Ok(Response::Array(elems))
}

#[cfg(test)]
mod tests {
    use crate::redis_test;

    redis_test! {
        test_sinter
        "sadd x 1 2 3" => 3;
        "sadd y 2 3 4" => 3;
        "sadd z 3 4 5" => 3;
        "sinter x"     => ["1", "2", "3"];
        "sinter x z"   => ["3"];
        "sinter x y z" => ["3"];
        "sinter x y"   => ["2", "3"];
        "sinter y x"   => ["2", "3"];
        "sinter q"     => [];
    }
}
