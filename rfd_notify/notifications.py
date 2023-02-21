from typing import List
import apprise
from models.topic import Topic
from models.post import Post
from loguru import logger


def send_notification(
    topic: Topic, posts: List[Post], expression: str, servers: str
) -> None:
    apobj = apprise.Apprise()
    apobj.add(servers)

    subject = topic.title
    body = f"""\
    <b>Post time:</b> {topic.post_time}
    <br>
    <br>
    <b>Dealer:</b> {topic.offer.dealer_name}
    <br>
    <br>
    <b>Deal:</b> {topic.title}
    <br>
    <br>
    <b>Post:</b> {topic.web_path}\
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
