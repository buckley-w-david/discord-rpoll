import logging
from typing import Optional, List, Protocol
from pathlib import Path

import discord
import typer


from scheduled_tasks.bot import create_bot
from scheduled_tasks.config import ScheduledTasksConfig

# TODO configurable log level
# Should also probably log to a file since this is a TUI application
logging.basicConfig(level=logging.CRITICAL)
logger = logging.getLogger(__name__)

app = typer.Typer()

@app.command()
def touch(config_file: Path):
    if not config_file.exists():
        config_file.parent.mkdir(parents=True, exist_ok=True)
        config = ScheduledTasksConfig()  # Creates a config structure with default values
        config.dump(config_file)

@app.command()
def run(config_file: Path = Path('tasks.toml')):
    touch(config_file)
    config = ScheduledTasksConfig.load(config_file)

    bot = create_bot(config)
    bot.run(config.token)

@app.command()
def send(channel_id: int, content: Path, attachments: List[Path], config_file: Path = Path('./tasks.toml')):
    touch(config_file)
    config = ScheduledTasksConfig.load(config_file)
    client = discord.Client()

    @client.event
    async def on_ready():
        channel = client.get_channel(channel_id)
        with open(content, 'r') as f:
            message_content = f.read()
        files = []
        for attachment in attachments:
            files.append(discord.File(str(attachment), attachment.name))
        await channel.send(message_content, files=files)
        await client.close()

    client.run(config.token)

if __name__ == "__main__":
    app()
