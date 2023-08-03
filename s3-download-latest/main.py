import logging
import datetime
import sys
import urllib.parse
import urllib.request
from typing import IO, Optional

import boto3

logger = logging.getLogger(__name__)


def main(
    base_url: str,
    aws_access_key_id: Optional[str],
    aws_secret_access_key: Optional[str],
    region_name: Optional[str],
    endpoint_url: Optional[str],
    dst_file: IO[bytes],
):
    logging.basicConfig(
        level=logging.INFO,
        handlers=[logging.StreamHandler(sys.stdout)],
        format="%(asctime)s [%(levelname)s] %(name)s / %(message)s"
    )

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

    latest = None
    last_modified = datetime.datetime.now().astimezone(datetime.timezone.utc) - datetime.timedelta(days=20 * 365)
    for object_response_itr in object_response_paginator.paginate(
        Bucket=bucket, Prefix=prefix
    ):
        for obj in object_response_itr.get("Contents", []):
            if obj["LastModified"] > last_modified:
                last_modified = obj["LastModified"]
                latest = obj["Key"]

    logger.info("latest: %s, @%s", latest, last_modified)

    s3.download_fileobj(bucket, latest, dst_file)


def __entry_point():
    import argparse

    parser = argparse.ArgumentParser(
        description="",  # プログラムの説明
    )
    parser.add_argument("base_url")
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
