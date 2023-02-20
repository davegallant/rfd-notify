import apprise
from rfd import Topic


def send_notification(topic: Topic, expression: str, servers: str) -> None:
    apobj = apprise.Apprise()
    apobj.add(servers)

    subject = topic.title
    body = f"""\
    <b>Date:</b> {topic.date}
    <br>
    <br>
    <b>Dealer:</b> {topic.dealer_name}
    <br>
    <br>
    <b>Deal:</b> {topic.title}
    <br>
    <br>
    <b>Post:</b> {topic.url}\
    <br>
    <br>
    <b>Body:</b> {topic.body}
    <br>
    <br>
    <b>Matched by expression:</b> {expression}
    """

    apobj.notify(
        body=body,
        title=subject,
    )
