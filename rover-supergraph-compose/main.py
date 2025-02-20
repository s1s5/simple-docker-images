import hashlib
import sys
import os
import subprocess
from typing import Optional

import click
import watchfiles
import yaml


def parse_file(config_filename: str):
    result = [config_filename]
    config = yaml.safe_load(open(config_filename, "r").read())
    for _, value in config.get("subgraphs", {}).items():
        schema_config = value.get("schema", {})
        if "file" in schema_config:
            path = schema_config["file"]
            if not os.path.isabs(path):
                path = os.path.normpath(os.path.join(os.path.dirname(config_filename), path))
            result.append(path)
    return result


def dump_supergraph(config: str, output: str, olddigest: Optional[str]):
    r = subprocess.run(
        ["rover", "supergraph", "compose", "--config", config],
        capture_output=True, text=False
    )
    print(r.stderr.decode("utf-8"), file=sys.stderr)
    if r.returncode == 0:
        digest = hashlib.sha256(r.stdout).hexdigest()

        if digest != olddigest:
            print(f"digest changed {output} update")
            with open(output, "wb") as fp:
                fp.write(r.stdout)
        else:
            print(f"no changes found for {output}")
        returncode = 0
    else:
        # エラー発生時の処理
        print("something wrong not updated")
        digest = olddigest or ""
    return r.returncode, digest


@click.command()
@click.option("--config", help="config path")
@click.option("--output", help="output path")
@click.option("--poll-ms", help="poll ms", type=int, default=0)
def auto_reload(config: str, output: str, poll_ms: int):
    kwargs = {}
    if poll_ms > 0:
        kwargs["rust_timeout"] = poll_ms
        kwargs["yield_on_timeout"] = True

    if os.path.exists(output):
        with open(output, "rb") as fp:
            digest = hashlib.sha256(fp.read()).hexdigest()
    else:
        digest = None

    while True:
        _code, digest = dump_supergraph(config, output, olddigest=digest)

        paths = parse_file(config)
        print(f"watching {paths}")

        for changes in watchfiles.watch(*paths, **kwargs):
            changed_paths = [path for _, path in changes]
            print(f"found changes {changed_paths}. reload={config not in changed_paths}")
            if config in changed_paths:
                break
            _code, digest = dump_supergraph(config, output, olddigest=digest)


if __name__ == "__main__":
    auto_reload()
