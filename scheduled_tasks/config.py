from datetime import datetime
import enum
from typing import List, Optional
from pydantic import BaseModel
import toml
from datetime import datetime
import pathlib

class TomlModel(BaseModel):
    @classmethod
    def load(cls, file):
        with open(file, "r") as f:
            return cls.parse_obj(toml.load(f))

    def dump(self, file):
        with open(file, "w") as f:
            toml.dump(self.dict(), f)

class ScheduledTasksConfig(TomlModel):
    task_base: pathlib.Path = './tasks'
    token: str = 'REPLACE ME'
