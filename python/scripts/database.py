import asyncpg
import os

from dotenv import load_dotenv


class CustomDatabase:

    def __init__(
        self,
        *,
        database: str | None = None,
        host: str | None = None,
        password: str | None = None,
        user: str | None = None
    ):
        load_dotenv()
        self._db_pool: asyncpg.Pool
        self.__database = database or os.environ["POSTGRES_DB"]
        self.__host = host or os.environ["POSTGRES_HOST"]
        self.__password = password or os.environ["POSTGRES_PWD"]
        self.__user = user or os.environ["POSTGRES_USER"]

    async def __aenter__(self):
        self._db_pool = await asyncpg.create_pool(
            database=self.__database,
            host=self.__host,
            password=self.__password,
            user=self.__user,
        )
        return self._db_pool

    async def __aexit__(self, exc_type, exc_val, exc_tb):
        await self._db_pool.close()
