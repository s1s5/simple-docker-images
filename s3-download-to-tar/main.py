import contextlib
import datetime
import os
import sys
import tarfile
import tempfile
import time
import traceback
import urllib.parse
import urllib.request
from concurrent.futures import ThreadPoolExecutor
from queue import Queue
from typing import IO, Optional, Tuple

import boto3

MAX_RETRY = 3
QueueItem = Optional[Tuple[str, IO[bytes]]] | StopIteration


def create_tar(dst_file: IO[bytes], q: Queue[QueueItem]):
    with tarfile.open(fileobj=dst_file, mode="w|") as t:
        while True:
            v = q.get()
            if v is None:
                continue
            elif isinstance(v, StopIteration):
                return
            path, ntf = v
            with contextlib.closing(ntf), open(ntf.name, "rb") as fp:
                tf = tarfile.TarInfo(path)

                fp.seek(0, os.SEEK_END)
                tf.size = fp.tell()
                fp.seek(0, os.SEEK_SET)

                t.addfile(tf, fileobj=fp)


def download_file(s3, bucket: str, key: str, dst_path: str, q: Queue[QueueItem]):
    ntf = None
    result: QueueItem = None
    try:
        ntf = tempfile.NamedTemporaryFile()
        for try_cnt in range(1, MAX_RETRY + 1):
            try:
                s3.download_file(bucket, key, ntf.name)
                break
            except Exception:
                if try_cnt == MAX_RETRY:
                    raise
                time.sleep(3 + try_cnt * 3)
                continue

        result = (dst_path, ntf)
    except Exception:
        if ntf:
            ntf.close()
        traceback.print_exc()
        result = None
    finally:
        q.put(result)


def main(
    base_url: str,
    tar_path_prefix: str,
    aws_access_key_id: Optional[str],
    aws_secret_access_key: Optional[str],
    region_name: Optional[str],
    endpoint_url: Optional[str],
    timedelta_hours: int,
    dst_file: IO[bytes],
):
    parsed = urllib.parse.urlparse(base_url)
    bucket = parsed.hostname
    prefix = parsed.path.lstrip("/")

    if not bucket:
        raise ValueError("bucket must be set")

    if (not aws_access_key_id) and parsed.username:
        aws_access_key_id = parsed.username

    if (not aws_secret_access_key) and parsed.password:
        aws_secret_access_key = parsed.password

    qm = urllib.parse.parse_qs(parsed.query)
    if (not region_name) and "region_name" in qm and isinstance(qm["region_name"], str):
        region_name = qm["region_name"]

    if (
        (not endpoint_url)
        and "endpoint_url" in qm
        and isinstance(qm["endpoint_url"], str)
    ):
        endpoint_url = qm["endpoint_url"]

    threshold = datetime.datetime.now(tz=datetime.timezone.utc) - datetime.timedelta(
        hours=timedelta_hours
    )

    session_kwargs = {}
    for field in ["aws_access_key_id", "aws_secret_access_key", "region_name"]:
        if locals()[field]:
            session_kwargs[field] = locals()[field]
    session = boto3.Session(**session_kwargs)

    client_kwargs = {}
    for field in ["endpoint_url"]:
        if locals()[field]:
            client_kwargs[field] = locals()[field]
    s3 = session.client("s3", **client_kwargs)

    object_response_paginator = s3.get_paginator("list_objects")

    q: Queue[QueueItem] = Queue()
    with ThreadPoolExecutor(max_workers=32) as executor:
        executor.submit(create_tar, dst_file, q)

        download_tasks = []
        for object_response_itr in object_response_paginator.paginate(
            Bucket=bucket, Prefix=prefix
        ):
            for obj in object_response_itr.get("Contents", []):
                last_modified = obj["LastModified"]
                if last_modified < threshold:
                    continue

                dst_path = tar_path_prefix + os.path.relpath(obj["Key"], start=prefix)

                download_tasks.append(
                    executor.submit(download_file, s3, bucket, obj["Key"], dst_path, q)
                )
        # wait all download complete
        try:
            for i in download_tasks:
                i.result()
        finally:
            q.put(StopIteration())


def __entry_point():
    import argparse

    parser = argparse.ArgumentParser(
        description="",  # プログラムの説明
    )
    parser.add_argument("--base-url")
    parser.add_argument("--tar-path-prefix", default="")
    parser.add_argument("--aws-access-key-id")
    parser.add_argument("--aws-secret-access-key")
    parser.add_argument("--region-name")
    parser.add_argument("--endpoint-url")
    parser.add_argument("--timedelta-hours", type=int, default=100 * 365 * 24)
    parser.add_argument(
        "-d", "--dst-file", type=argparse.FileType("wb"), default=sys.stdout.buffer
    )

    main(**dict(parser.parse_args()._get_kwargs()))


if __name__ == "__main__":
    __entry_point()
