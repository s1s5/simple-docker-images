import hashlib
import json
import time
from typing import Optional

import click
import requests
import watchfiles
from graphql import build_client_schema, get_introspection_query
from graphql.utilities.print_schema import print_schema
from requests.adapters import HTTPAdapter, Retry


def output_schema_str(
    session: requests.Session, url: str, output: Optional[str], olddigest: Optional[str] = None
):
    res = session.post(
        url,
        headers={"Content-Type": "application/json"},
        data=json.dumps({"query": get_introspection_query()}),
    )
    schema_str = print_schema(build_client_schema(res.json()["data"]))
    digest = hashlib.sha256(schema_str.encode("utf-8")).hexdigest()
    if digest != olddigest:
        if output:
            with open(output, "rb") as fp:
                fp.write(schema_str.encode("utf-8"))
        else:
            print(schema_str)

    return digest


@click.command()
@click.option("--url", help="graphql endpoint")
@click.option("--output", help="output path")
@click.option("--watch", help="watch path. ',' separated path list")
@click.option("--watch-delay-ms", default=1000, help="delay after file changes.")
@click.option("--yield-on-timeout", default=False, is_flag=True)
def dump_schema(url: str, output: str, watch: str, watch_delay_ms: int, yield_on_timeout: bool):
    session = requests.Session()
    adapter = HTTPAdapter(
        pool_connections=1,
        pool_maxsize=1,
        max_retries=Retry(total=30, backoff_factor=0.5, status_forcelist=[500, 502, 503, 504]),
    )
    session.mount("http://", adapter)
    session.mount("https://", adapter)

    digest = output_schema_str(session, url, output)

    if watch:
        for _ in watchfiles.watch(*watch.split(","), yield_on_timeout=yield_on_timeout):
            if watch_delay_ms:
                time.sleep(watch_delay_ms / 1000)
            digest = output_schema_str(session, url, output, olddigest=digest)


if __name__ == "__main__":
    dump_schema()
