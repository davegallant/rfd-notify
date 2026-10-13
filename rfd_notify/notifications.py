from datetime import datetime, timezone
import re

import apprise
import humanize
from models.topic import Topic
from loguru import logger
from constants import API_BASE_URL


def escape_markdown(value: str) -> str:
    """Keep forum titles from changing the notification's Markdown structure."""
    return re.sub(r"([\\`*_{}\[\]()#+.!|>-])", r"\\\1", value)


def send_notification(topic: Topic, expression: str, servers: str) -> None:
    if servers is None:
        logger.warning("APPRISE_URL is not set. Will not send notification")
        return

    apobj = apprise.Apprise()
    apobj.add(servers)

    subject = "RFD deal match"
    age = humanize.naturaltime(
        datetime.now(timezone.utc) - datetime.fromisoformat(topic.post_time)
    )
    body = (
        f"**[{escape_markdown(topic.title)}]({API_BASE_URL}{topic.web_path})"
        '{:target="_blank"}**\n\n'
        f"⌚ {age} · Matched: {escape_markdown(expression)}"
    )

    logger.debug("Sending notification")

    apobj.notify(
        body=body,
        title=subject,
        body_format=apprise.NotifyFormat.MARKDOWN,
    )
