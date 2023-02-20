import argparse

from config import load_yaml_file
from rfd import get_topics
from loguru import logger


def main() -> None:
    parser = argparse.ArgumentParser(description="Process some configuration.")

    parser.add_argument(
        "-c", "--config", type=str, required=True, help="path to configuration file"
    )

    args = parser.parse_args()

    config_path = args.config

    config = load_yaml_file(config_path)

    topics = get_topics(9, 2)

    logger.debug(f"config: {config}")
    # logger.debug(topics)


main()
