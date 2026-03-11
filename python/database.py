from dotenv import load_dotenv
import asyncpg
import os
import pytest
from aiohttp import web

def test_main():
   main()

class Database:

    def __init__(self, *, database: str | None = None, host: str | None = None, password: str | None = None, user: str | None = None):
        self.__database = database or os.environ["POSTGRES_DB"]
        self.__host = host or os.environ["POSTGRES_HOST"]
        self.__password = password or os.environ["POSTGRES_PWD"]
        self.__user = user or os.environ["POSTGRES_USER"]


    async def start(self, app):
        app['pool'] = await asyncpg.create_pool(database=self.__database, host=self.__host, password=self.__password, user=self.__user)
        yield
        await app['pool'].close()

def main():
   load_dotenv()
   db = Database()
   app = web.Application()
   app.cleanup_ctx.append(db.start)
   web.run_app(app)

