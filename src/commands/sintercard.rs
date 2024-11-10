use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response};

pub static INFO: CommandInfo = CommandInfo {
    name: b"sintercard",
    arity: -3,
    flags: &[
        b"readonly",
        b"movablekeys",
    ],
    first_key: 0,
    last_key: 0,
    step: 0,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let (numkeys, key, keys) = cmd.parse_args::<(i64, ByteString, Vec<ByteString>)>()?;
    anyhow::ensure!(numkeys as usize == keys.len() + 1, "expected numkeys to be equal to number of keys");
    let Some(mut set) = db.get_set(&key)?.cloned() else { return Ok(Response::Number(0)) };
    for k in keys {
        let Some(s) = db.get_set(&k)? else { continue };
        set.retain(|e| s.contains(e));
    }
    Ok(Response::Number(set.len() as _))
}

#[cfg(test)]
mod tests {
    use crate::redis_test;

    redis_test! {
        test_sintercard
        "sadd x 1 2 3"       => 3;
        "sadd y 2 3 4"       => 3;
        "sadd z 3 4 5"       => 3;
        "sintercard 1 x"     => 3;
        "sintercard 2 x z"   => 1;
        "sintercard 3 x y z" => 1;
        "sintercard 2 x y"   => 2;
        "sintercard 2 y x"   => 2;
        "sintercard 1 q"     => 0;
        "sintercard 2 q x"   => 0;
    }
}
