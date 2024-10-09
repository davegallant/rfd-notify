from json.decoder import JSONDecodeError
from typing import List, Any
import re
import shelve
import requests
from loguru import logger
from constants import API_BASE_URL, HEADERS

from config import Config
from notifications import send_notification
from models.topic import Topic
from models.post import Post


class RfdTopicsException(Exception):
    def __init__(self, message):
        self.message = message


def get_topic(topic_id: int) -> List[Post]:
    posts = []
    try:
        response = requests.get(
            f"{API_BASE_URL}/api/topics/{topic_id}/posts?per_page=1&page=1",
            headers=HEADERS,
            timeout=30,
        )
        if response.status_code != 200:
            raise RfdTopicsException(
                f"Received status code {response.status_code} when getting topic."
            )
        for post in response.json().get("posts"):
            posts.append(Post(**post))
    except JSONDecodeError as err:
        logger.error("Unable to decode topics. %s", err)
    return posts


def get_topics(forum_id: int, pages: int) -> List[Topic]:
    topics = []
    try:
        for page in range(1, pages + 1):
            response = requests.get(
                f"{API_BASE_URL}/api/topics?forum_id={forum_id}&per_page=40&page={page}",
                headers=HEADERS,
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


def look_for_matches(
    topics: List[Topic], config: Config, storage_path: str, apprise_url: Any
):
    with shelve.open(storage_path) as previous_matches:
        for topic in topics:
            found_match = False
            for expression in config.expressions:
                expression = expression.lower()
                topic_title = topic.title.lower()
                dealer_name = ""

                if topic.offer and topic.offer.dealer_name is not None:
                    dealer_name = topic.offer.dealer_name.lower()
                if re.search(expression, topic_title):
                    found_match = True
                    logger.debug(
                        f"Expression {expression} matched title '{dealer_name} - {topic.title}'"
                    )
                elif re.search(expression, dealer_name):
                    found_match = True
                    logger.debug(
                        f"Expression {expression} matched dealer '{dealer_name}' - '{topic.title}'"
                    )

                if not found_match:
                    continue

                if str(topic.topic_id) not in previous_matches:
                    posts = get_topic(topic.topic_id)
                    previous_matches[str(topic.topic_id)] = 1
                    send_notification(topic, posts, expression, apprise_url)
                else:
                    logger.debug(f"Already matched topic '{topic.title}'")
                break
