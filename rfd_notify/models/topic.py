from .offer import Offer


class Topic:
    # pylint: disable=unused-argument
    # pylint: disable=too-many-arguments
    def __init__(
        self,
        topic_id: int,
        title: str,
        post_time: str,
        web_path: str,
        offer: dict,
        **kwargs,
    ):
        self.topic_id = topic_id
        self.title = title
        self.post_time = post_time
        self.web_path = web_path
        if offer:
            self.offer = Offer(**offer)

    def __repr__(self):
        return f"Topic({self.title})"
