import argparse
import json
import subprocess
from pathlib import Path

CMD_DIR = Path(__file__).parent / 'src' / 'commands'

SRC = """
use super::{{CommandInfo, RedisCommand}};
use crate::command::Command;
use crate::{{ByteString, Database, Response, Value}};

static INFO: CommandInfo = CommandInfo {{
    name: b"{name}",
    arity: {arity},
    flags: &[{flags}],
    first_key: {first_key},
    last_key: {last_key},
    step: {step},
}};

pub struct Cmd;

impl RedisCommand for Cmd {{
    fn info(&self) -> &'static CommandInfo {{ &INFO }}

    fn run(&self, _: &mut Database, _: Command) -> anyhow::Result<Response> {{
        todo!()
    }}
}}
""".lstrip()

parser = argparse.ArgumentParser()
parser.add_argument('cmds', nargs='*', help='instantiate one or more commands (none means all)')
opts = parser.parse_args()

res = subprocess.run(['redis-cli', '--json', 'command'], stdout=subprocess.PIPE, text=True, check=True)
command_info = sorted(json.loads(res.stdout), key=lambda cmd: cmd[0])
for name, arity, flags, first_key, last_key, step, *_ in command_info:
    if opts.cmds and name not in opts.cmds:
        continue
    print(f'instantiating command "{name}"')
    flags = '\n        '.join(f'b"{f}",' for f in flags)
    if flags:
        flags = f'\n        {flags}\n    '
    (CMD_DIR / f'{name}.rs').write_text(SRC.format(
        name=name,
        arity=arity,
        flags=flags,
        first_key=first_key,
        last_key=last_key,
        step=step,
    ))
