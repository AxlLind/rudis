#!/usr/bin/python3
import json
import subprocess
from pathlib import Path

COMMANDS_DIR = Path(__file__).parent / '..' / 'rudis-core' / 'src' / 'commands'


def main() -> None:
    res = subprocess.run(
        ['redis-cli', '--json', 'command', 'info'],
        stdout=subprocess.PIPE,
        text=True,
        check=True,
    )
    all_commands = set(c[0] for c in json.loads(res.stdout))
    implemented_commands = set(p.with_suffix('').name for p in COMMANDS_DIR.glob('*.rs'))
    for c in sorted(all_commands - implemented_commands):
        print(c)


if __name__ == "__main__":
    main()
