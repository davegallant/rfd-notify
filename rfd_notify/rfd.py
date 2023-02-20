import requests
from constants import API_BASE_URL
from json.decoder import JSONDecodeError
from loguru import logger

from typing import List, Dict


class Thread:
    def __init__(self, title, dealer_name, url, external_url, views):
        self.dealer_name = dealer_name
        self.title = title
        self.url = url
        self.external_url = external_url
        self.views = views

    def __repr__(self):
        return f"Thread({self.title})"


def get_threads(forum_id: int, pages: int) -> List[Dict]:
    threads = []
    try:
        for page in range(1, pages + 1):
            response = requests.get(
                f"{API_BASE_URL}/api/topics?forum_id={forum_id}&per_page=40&page={page}"
            )
            if response.status_code != 200:
                raise Exception(
                    f"When collecting threads, received a status code: {response.status_code}"
                )
            threads += response.json().get("topics")
    except JSONDecodeError as err:
        logger.error("Unable to decode threads. %s", err)
    return threads
