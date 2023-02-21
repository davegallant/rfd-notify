import argparse
import os

from config import load_yaml_file
from rfd import get_topics, look_for_matches
from loguru import logger


def main() -> None:
    parser = argparse.ArgumentParser(description="Process some configuration.")

    parser.add_argument(
        "-c", "--config", type=str, required=True, help="path to configuration file"
    )

    parser.add_argument(
        "-s",
        "--storage-path",
        type=str,
        required=False,
        default="previous_matches",
        help="path to persistent storage",
    )

    args = parser.parse_args()

    config_path = args.config

    apprise_url = os.getenv("APPRISE_URL")

    config = load_yaml_file(config_path)

    topics = get_topics(forum_id=9, pages=2)

    logger.debug(f"config: {config}")

    look_for_matches(topics, config, args.storage_path, apprise_url)


main()
