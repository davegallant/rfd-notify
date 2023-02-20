from typing import Any, Optional
import yaml
from loguru import logger


def load_yaml_file(filename: str) -> Optional[Any]:
    with open(filename, "r", encoding="utf-8") as file:
        try:
            data = yaml.safe_load(file)
        except yaml.YAMLError as err:
            logger.error(f"Error loading config file: {err}")
            return None
    return data
