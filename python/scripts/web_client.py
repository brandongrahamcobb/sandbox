from contextlib import AsyncExitStack
from typing import Optional
import aiohttp


class CustomWebClient:
    def __init__(self):
        self.session: Optional[aiohttp.ClientSession] = None
        self.exit_stack = AsyncExitStack()

    async def __aenter__(self):
        return self

    async def __aexit__(self, exc_type, exc_val, exc_tb):
        await self.cleanup()

    async def cleanup(self):
        await self.exit_stack.aclose()
