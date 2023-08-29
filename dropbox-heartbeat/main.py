import datetime
import logging
import os
import sys
import time
from fnmatch import fnmatch
from typing import Optional
from urllib.parse import urljoin

import dropbox
import requests
from dropbox.files import ListFolderResult
from requests.adapters import HTTPAdapter, Retry

logger = logging.getLogger()


def checkpath_and_heartbeat(
    dbx: dropbox.Dropbox,
    path: str,
    heartbeat_url: str,
    ttl: int,
):
    if path.endswith("/"):
        dirname = path
        xp = "*"
    else:
        dirname, xp = os.path.split(path)

    now = time.time()
    latest = 0
    result: ListFolderResult = dbx.files_list_folder(dirname)  # type: ignore
    for entry in result.entries:
        logger.debug("entry: %s", entry.path_display)
        if not fnmatch(os.path.basename(entry.path_display), xp):
            continue
        latest = max(latest, entry.server_modified.replace(tzinfo=datetime.timezone.utc).timestamp())

    logger.debug("now=%s, latest=%s, result=%s", now, latest, latest + ttl > now)
    if latest + ttl > now:
        requests.get(heartbeat_url, timeout=10)
    else:
        requests.get(heartbeat_url + "/fail", timeout=10)


def main(
    path: str,
    dropbox_token: Optional[str],
    dropbox_token_envvar: Optional[str],
    heartbeat_url: str,
    ttl: int,
    verbose: bool,
):
    logger.addHandler(logging.StreamHandler(sys.stdout))
    if verbose:
        logger.setLevel(logging.DEBUG)

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
    checkpath_and_heartbeat(dbx, path=path, heartbeat_url=heartbeat_url, ttl=ttl)


def __entry_point():
    import argparse

    parser = argparse.ArgumentParser(
        description="",  # プログラムの説明
    )
    parser.add_argument("-p", "--path", required=True)
    parser.add_argument("-t", "--dropbox-token")
    parser.add_argument("-e", "--dropbox-token-envvar")
    parser.add_argument("-u", "--heartbeat-url", required=True)
    parser.add_argument("--ttl", type=int, required=True)
    parser.add_argument("--verbose", action="store_true")
    main(**dict(parser.parse_args()._get_kwargs()))


if __name__ == "__main__":
    __entry_point()
