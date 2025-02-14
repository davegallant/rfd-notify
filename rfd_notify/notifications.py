from datetime import datetime, timezone

import apprise
from models.topic import Topic
from loguru import logger
from constants import API_BASE_URL


def send_notification(topic: Topic, expression: str, servers: str) -> None:
    if servers is None:
        logger.warning("APPRISE_URL is not set. Will not send notification")
        return

    apobj = apprise.Apprise()
    apobj.add(servers)

    subject = topic.title
    body = f"""\
{API_BASE_URL}{topic.web_path}
Age: {datetime.now(timezone.utc) - datetime.fromisoformat(topic.post_time)}
Matched by expression: {expression}
"""

    logger.debug("Sending notification")

    apobj.notify(
        body=body,
        title=subject,
    )
