from typing import List
from datetime import datetime, timezone

import apprise
from models.topic import Topic
from models.post import Post
from loguru import logger
from constants import API_BASE_URL


def send_notification(
    topic: Topic, posts: List[Post], expression: str, servers: str
) -> None:
    if servers is None:
        logger.warning("APPRISE_URL is not set. Will not send notification")
        return

    apobj = apprise.Apprise()
    apobj.add(servers)

    if topic.offer:
        dealer_name = topic.offer.dealer_name
    else:
        dealer_name = ""

    subject = topic.title
    body = f"""\
    <b>Post age:</b> {datetime.now(timezone.utc) - datetime.fromisoformat(topic.post_time)}
    <br>
    <br>
    <b>Dealer:</b> {dealer_name}
    <br>
    <br>
    <b>Deal:</b> {topic.title}
    <br>
    <br>
    <b>Post:</b> {API_BASE_URL}{topic.web_path}\
    <br>
    <br>
    <b>Body:</b> {posts[0].body}
    <br>
    <br>
    <b>Matched by expression:</b> {expression}
    """

    logger.debug("Sending notification")

    apobj.notify(
        body=body,
        title=subject,
    )
