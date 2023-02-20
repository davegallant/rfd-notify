import argparse

from config import load_yaml_file


def main() -> None:
    parser = argparse.ArgumentParser(description="Process some configuration.")

    parser.add_argument(
        "-c", "--config", type=str, required=True, help="path to configuration file"
    )

    args = parser.parse_args()

    config_path = args.config

    config = load_yaml_file(config_path)
