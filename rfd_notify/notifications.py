import apprise

def send_notification(servers: str) -> None:
    apobj = apprise.Apprise()
    apobj.add(servers)

    subject = title
    body = f"""\
    <b>Date:</b> {}
    <br>
    <br>
    <b>Dealer:</b> {}
    <br>
    <br>
    <b>Deal:</b> {}
    <br>
    <br>
    <b>Post:</b> {}\
    <br>
    <br>
    <b>Body:</b> {}
    <br>
    <br>
    <b>Matched by expression:</b> {}
    """

    apobj.notify(
        body=body,
        title=subject,
    )
