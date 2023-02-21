from json.decoder import JSONDecodeError
from typing import List, Dict
import re
import shelve
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


def look_for_matches(topics: List[Topic], config: Config, storage_path: str):
    found_match = False

    with shelve.open(storage_path) as previous_matches:
        for topic in topics:
            logger.debug(topic)
            for expression in config.expressions:
                expression = expression.lower()
                topic_title = topic.title.lower()
                dealer_name = topic.offer["dealer_name"].lower()
                if re.search(expression, topic_title):
                    found_match = True
                    logger.debug(
                        f"Expression {expression} matched title '{topic.title}'"
                    )
                elif re.search(expression, dealer_name):
                    found_match = True
                    logger.debug(
                        f"Expression {expression} matched dealer '{dealer_name}' - '{topic.title}'"
                    )
                if not found_match:
                    continue
                if str(topic.topic_id) not in previous_matches:
                    previous_matches[str(topic.topic_id)] = 1
                    # Send notifications
                else:
                    logger.debug(f"Already matched topic with title '{topic.title}'")
                break
