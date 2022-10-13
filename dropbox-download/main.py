import contextlib
import os
import sys
from typing import IO, Optional

import dropbox
import dropbox.files
import requests
from requests.adapters import HTTPAdapter, Retry

CHUNK_SIZE = 4 << 20


def download_from_dropbox(dbx: dropbox.Dropbox, dbx_path: str, dst_fp: IO[bytes]):
    _, http_res = dbx.files_download(dbx_path)  # type: ignore
    with contextlib.closing(dst_fp), contextlib.closing(http_res):
        for chunk in http_res.iter_content(CHUNK_SIZE):
            dst_fp.write(chunk)


def main(
    dst_file: IO[bytes],
    dropbox_token: Optional[str],
    dropbox_token_envvar: Optional[str],
    src_path: str,
):
    session = requests.Session()
    adapter = HTTPAdapter(
        pool_connections=8,
        pool_maxsize=32,
        max_retries=Retry(
            total=10, backoff_factor=0.5, status_forcelist=[500, 502, 503, 504]
        ),
    )

    session.mount("http://", adapter)
    session.mount("https://", adapter)

    dropbox_token = dropbox_token or os.environ.get(dropbox_token_envvar or "") or ""

    dbx = dropbox.Dropbox(dropbox_token, session=session)
    download_from_dropbox(dbx, src_path, dst_file)


def __entry_point():
    import argparse

    parser = argparse.ArgumentParser(
        description="",  # プログラムの説明
    )
    parser.add_argument("-s", "--src-path")
    parser.add_argument(
        "-d", "--dst-file", type=argparse.FileType("wb"), default=sys.stdout.buffer
    )
    parser.add_argument("-t", "--dropbox-token")
    parser.add_argument("-e", "--dropbox-token-envvar")
    main(**dict(parser.parse_args()._get_kwargs()))


if __name__ == "__main__":
    __entry_point()
