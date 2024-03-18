use std::collections::HashMap;
use anyhow;

mod command;
use command::Parser;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    String(Vec<u8>),
    Hash(HashMap<Vec<u8>, Vec<u8>>),
    Nil,
}

fn execute_command(state: &mut HashMap<Vec<u8>, Value>, cmd: Vec<Vec<u8>>) -> anyhow::Result<Value> {
    let value = match cmd[0].as_slice() {
        b"GET" => {
            let [_, key] = cmd.try_into().map_err(|_| anyhow::anyhow!("expected GET key"))?;
            state.get(&key).cloned().unwrap_or(Value::Nil)
        }
        b"SET" => {
            let [_, key, value] = cmd.try_into().map_err(|_| anyhow::anyhow!("expected SET key value"))?;
            state.insert(key, Value::String(value));
            Value::String(b"OK".to_vec())
        }
        _ => anyhow::bail!("Unrecognized command: {:?}", cmd[0]),
    };
    Ok(value)
}

fn main() -> anyhow::Result<()> {
    let mut parser = Parser::new(b"*2\r\n$4\r\nLLEN\r\n$6\r\nmylist\r\n".as_slice());
    let mut state = HashMap::new();
    let cmd = parser.read_string_array()?;
    let _res = execute_command(&mut state, cmd)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_set() {
        let mut state = HashMap::new();
        let res = execute_command(&mut state, vec![b"GET".to_vec(), b"a".to_vec()]).unwrap();
        assert_eq!(res, Value::Nil);

        let res = execute_command(&mut state, vec![b"SET".to_vec(), b"a".to_vec(), b"b".to_vec()]).unwrap();
        assert_eq!(res, Value::String(b"OK".to_vec()));

        let res = execute_command(&mut state, vec![b"GET".to_vec(), b"a".to_vec()]).unwrap();
        assert_eq!(res, Value::String(b"b".to_vec()));

        let res = execute_command(&mut state, vec![b"SET".to_vec(), b"a".to_vec(), b"c".to_vec()]).unwrap();
        assert_eq!(res, Value::String(b"OK".to_vec()));

        let res = execute_command(&mut state, vec![b"GET".to_vec(), b"a".to_vec()]).unwrap();
        assert_eq!(res, Value::String(b"c".to_vec()));
    }
}
