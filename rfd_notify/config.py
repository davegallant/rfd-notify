import yaml
from loguru import logger


class Config:
    def __init__(self, expressions):
        self.expressions = expressions

    def __repr__(self):
        return f"Config(expressions={self.expressions})"


def load_yaml_file(filename: str) -> Config:
    with open(filename, "r", encoding="utf-8") as file:
        try:
            data = yaml.safe_load(file)
        except yaml.YAMLError as err:
            logger.error(f"Error loading config file: {err}")
    return Config(**data)
