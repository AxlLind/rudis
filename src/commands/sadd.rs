use super::CommandInfo;
use crate::cmd_parser::Command;
use crate::{ByteString, Database, Response, Value};

pub static INFO: CommandInfo = CommandInfo {
    name: b"sadd",
    arity: -3,
    flags: &[
        b"write",
        b"denyoom",
        b"fast",
    ],
    first_key: 1,
    last_key: 1,
    step: 1,
};

pub fn run(db: &mut Database, mut cmd: Command) -> anyhow::Result<Response> {
    let (key, elems) = cmd.parse_args::<(ByteString, Vec<ByteString>)>()?;
    anyhow::ensure!(!elems.is_empty(), "expected SADD member [member ...]");
    match db.get_set(&key)? {
        Some(set) => {
            let prelen = set.len();
            set.extend(elems);
            Ok(Response::Number((set.len() - prelen) as _))
        }
        None => {
            let len = elems.len();
            db.set(key, Value::Set(elems.into_iter().collect()));
            Ok(Response::Number(len as _))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::redis_test;

    redis_test! {
        test_sadd
        "sadd x 1 2 3" => 3;
        "sadd x 1 2 3" => 0;
        "sadd x 3 4" => 1;
    }
}
