import yaml
from typing import Any, Optional
from loguru import logger


def load_yaml_file(filename: str) -> Optional[Any]:
    with open(filename, "r") as file:
        try:
            data = yaml.safe_load(file)
        except yaml.YAMLError as e:
            logger.error(f"Error loading config file: {e}")
            return None
    return data
