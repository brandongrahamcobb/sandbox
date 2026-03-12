from database import CustomDatabase
from dotenv import load_dotenv
from mcp_client import CustomMCPClient
from web_client import CustomWebClient
import asyncpg
import asyncio
import aiohttp
import mcp


class CustomBot:
    def __init__(self, pool: asyncpg.Pool, mcp: CustomMCPClient, web: CustomWebClient):
        self.pool = pool
        self.mcp = mcp
        self.web = web

    async def __aenter__(self):
        print("CustomBot initialized")
        return self

    async def __aexit__(self, exc_type, exc_val, exc_tb):
        print("CustomBot killed")

    async def repl(self):
        await self.mcp.chat_loop()


async def main():
    async with CustomWebClient() as web_client, CustomMCPClient() as mcp_client, CustomDatabase() as pool:
        async with CustomBot(pool=pool, mcp=mcp_client, web=web_client) as bot:
            await bot.repl()


if __name__ == "__main__":
    import sys

    load_dotenv()
    asyncio.run(main())
