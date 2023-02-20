from json.decoder import JSONDecodeError
from typing import List, Dict
import requests
from loguru import logger
from constants import API_BASE_URL


class Topic:
    def __init__(self, title, dealer_name, url, external_url, views):
        self.dealer_name = dealer_name
        self.title = title
        self.url = url
        self.external_url = external_url

    def __repr__(self):
        return f"Topic({self.title})"


def get_topics(forum_id: int, pages: int) -> List[Dict]:
    topics = []
    try:
        for page in range(1, pages + 1):
            response = requests.get(
                f"{API_BASE_URL}/api/topics?forum_id={forum_id}&per_page=40&page={page}"
            )
            if response.status_code != 200:
                raise Exception(
                    f"When collecting topics, received a status code: {response.status_code}"
                )
            topics += response.json().get("topics")
    except JSONDecodeError as err:
        logger.error("Unable to decode topics. %s", err)
    return topics
