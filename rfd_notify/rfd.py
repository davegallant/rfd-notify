from json.decoder import JSONDecodeError
from typing import List, Dict
import re
import requests
from loguru import logger
from constants import API_BASE_URL

from config import Config


class RfdTopicsException(Exception):
    def __init__(self, message):
        self.message = message


class Offer:
    def __init__(self, dealer_name, url):
        self.dealer_name = dealer_name
        self.url = url

    def __repr__(self):
        return f"Offer({self.url})"


class Topic:
    # pylint: disable=unused-argument
    # pylint: disable=too-many-arguments
    def __init__(
        self,
        topic_id: int,
        title: str,
        post_time: str,
        web_path: str,
        offer: Offer,
        **kwargs,
    ):
        self.topic_id = topic_id
        self.title = title
        self.post_time = post_time
        self.web_path = web_path
        self.offer = offer

    def __repr__(self):
        return f"Topic({self.title})"


def get_topics(forum_id: int, pages: int) -> List[Dict]:
    topics = []
    try:
        for page in range(1, pages + 1):
            response = requests.get(
                f"{API_BASE_URL}/api/topics?forum_id={forum_id}&per_page=40&page={page}",
                timeout=30,
            )
            if response.status_code != 200:
                raise RfdTopicsException(
                    f"Received status code {response.status_code} when getting topics."
                )
            for topic in response.json().get("topics"):
                topics.append(Topic(**topic))
    except JSONDecodeError as err:
        logger.error("Unable to decode topics. %s", err)
    return topics


def match_topics(topics: List[Topic], config: Config):
    found_match = False
    for topic in topics:
        for expression in config.expressions:
            if re.search(expression, topic.title):
                found_match = True
                logger.debug(f"Expression {expression} matched title '{topic.title}'")
            elif re.search(expression, topic.offer["dealer_name"]):
                found_match = True
                logger.debug(
                    f"Expression {expression} matched dealer '{topic.offer.dealer_name}'"
                )
            if not found_match:
                continue
            # Put into db
