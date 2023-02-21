class Offer:
    # pylint: disable=unused-argument
    def __init__(self, dealer_name, url, **kwargs):
        self.dealer_name = dealer_name
        self.url = url

    def __repr__(self):
        return f"Offer({self.url})"
