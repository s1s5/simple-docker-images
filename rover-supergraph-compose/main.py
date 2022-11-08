import os
import subprocess

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

def dump_supergraph(config: str, output: str):
    p = subprocess.Popen(
        ["rover", "supergraph", "compose", "--config", config],
        stdout=subprocess.PIPE
    )
    p.wait()

    stdout, _ = p.communicate()

    with open(output, "wb") as fp:
        fp.write(stdout)


@click.command()
@click.option("--config", help="config path")
@click.option("--output", help="output path")
def auto_reload(config: str, output: str):
    dump_supergraph(config, output)

    while True:
        paths = parse_file(config)

        for changes in watchfiles.watch(*paths):
            changed_paths = [path for _, path in changes]
            if config in changed_paths:
                break
            dump_supergraph(config, output)
            


if __name__ == "__main__":
    auto_reload()
