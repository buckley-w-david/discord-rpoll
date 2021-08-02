import asyncio
import concurrent.futures
import json
import pathlib
import pprint
import uuid

import discord
from discord.ext import commands, tasks
from crontab import CronSlices, CronTab, CronItem

from scheduled_tasks.config import ScheduledTasksConfig


def create_bot(config: ScheduledTasksConfig):
    bot = commands.Bot("!")

    class ScheduledTasks(commands.Cog):
        def __init__(self, bot: commands.Bot, config: ScheduledTasksConfig):
            self.bot = bot
            self.config = config

        @commands.command()
        @commands.is_owner()
        async def info(self, ctx, *, member: discord.Member = None):
            """Get some info"""
            member =  member or ctx.author
            for role in member.roles:
                print(role.name)
                print(role.id)
                print(role.mention)

            await ctx.send(pprint.pformat(vars(ctx), indent=4))

        @commands.command()
        async def schedule(self, ctx):
            await ctx.send(f"Schedule?")

            def valid_cron(message):
                return CronSlices.is_valid(message.content) and message.author,id == ctx.author.id

            def valid_msg(message):
                return message.author,id == ctx.author.id

            msg = await self.bot.wait_for("message", check=valid_cron)
            schedule = msg.content

            await ctx.send("Message?")
            msg = await self.bot.wait_for("message", chech=valid_msg)
            await msg.add_reaction("✅")

            job_id = uuid.uuid4()
            job_dir = self.config.task_base / str(job_id)
            job_dir.mkdir(parents=True, exist_ok=True)
            with open(job_dir / 'content.txt', 'w') as f:
                f.write(msg.content)

            for attachment in msg.attachments:
                with open(job_dir / attachment.filename, 'wb') as f:
                    await attachment.save(f)
            files = " ".join(f'"{(job_dir / attachment.filename).resolve()}"' for attachment in msg.attachments)

            with CronTab(user=True) as cron:
                line = f'{schedule} {self.config.send} "{ctx.message.channel.id}" "{(job_dir / "content.txt").resolve()}" {files}'
                job = CronItem.from_line(line, cron=cron)
                cron.append(job)

    bot.add_cog(ScheduledTasks(bot, config))
    return bot
