from database import Database
from dataclasses import asdict, dataclass
from contextlib import AsyncExitStack
from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client
from metadata import Metadata, MetadataContainer, MetadataHolder
from typing import Any, Optional
import asyncio
import json
import os
import requests


def json_to_dict(content: str):
    dictionary = json.loads(content)
    return dictionary


class Messages:

    def build_query(
        self, model: str, max_tokens: int, messages: list[dict[str, str]], tools=list
    ):
        data = {}
        data["messages"] = messages
        data["max_tokens"] = max_tokens
        data["model"] = model
        data["tools"] = tools
        return json.dumps(data)

    def create(
        self,
        model: str = "Qwen3.5-9B-Q4_K_M",
        max_tokens: int = 128000,
        messages: list[dict[str, str]] = [{}],
        tools: list = [],
    ):
        query = self.build_query(
            model=model, max_tokens=max_tokens, messages=messages, tools=tools
        )
        x = requests.post("http://localhost:8080/v1/chat/completions", query)
        json_dict = json_to_dict(x.content)
        llama = Llama()
        container = llama.load(json=json_dict)
        return asdict(container)["holders"]["content"]["object"]["value"]


class Llama:

    def __init__(self):
        self.__holders = {}
        self.messages = Messages()

    def load(self, json: dict[str, Any]):
        self.json_to_id(json)
        self.json_to_total_tokens(json)
        self.json_to_model(json)
        self.json_to_created(json)
        self.json_to_usage(json)
        self.json_to_choices(json)
        self.json_to_finish_reason(json)
        return MetadataContainer(holders=self.__holders)

    def json_to_id(self, json: dict[str, Any]):
        id_key = "id"
        id_value = json.get(id_key, None)
        id_metadata = Metadata(value=id_value)
        id_holder = MetadataHolder(object=id_metadata)
        self.__holders.update({id_key: id_holder})

    def json_to_total_tokens(self, json: dict[str, Any]):
        total_tokens_key = "total_tokens"
        total_tokens_value = json.get(total_tokens_key, None)
        total_tokens_metadata = Metadata(value=total_tokens_value)
        total_tokens_holder = MetadataHolder(object=total_tokens_metadata)
        self.__holders.update({total_tokens_key: total_tokens_holder})

    def json_to_model(self, json: dict[str, Any]):
        model_key = "model"
        model_value = json.get(model_key, None)
        model_metadata = Metadata(value=model_value)
        model_holder = MetadataHolder(object=model_metadata)
        self.__holders.update({model_key: model_holder})

    def json_to_created(self, json: dict[str, Any]):
        created_key = "created"
        created_value = json.get(created_key, None)
        created_metadata = Metadata(value=created_value)
        created_holder = MetadataHolder(object=created_metadata)
        self.__holders.update({created_key: created_holder})

    def json_to_usage(self, json: dict[str, Any]):
        usage_key = "usage"
        usage_value = json.get(usage_key, None)
        usage_metadata = Metadata(value=usage_value)
        usage_holder = MetadataHolder(object=usage_metadata)
        self.__holders.update({usage_key: usage_holder})

    def json_to_choices(self, json: dict[str, Any]):
        choices_key = "choices"
        choices_value = json.get(choices_key, None)
        message_key = "message"
        message_value = choices_value[0].get(message_key, None)
        role_key = "role"
        role_value = message_value.get(role_key, None)
        role_metadata = Metadata(value=role_value)
        role_holder = MetadataHolder(object=role_metadata)
        content_key = "content"
        content_value = message_value.get(content_key, None)
        content_metadata = Metadata(value=content_value)
        content_holder = MetadataHolder(object=content_metadata)
        self.__holders.update({role_key: role_holder, content_key: content_holder})

    def json_to_finish_reason(self, json: dict[str, Any]):
        finish_reason_key = "finish_reason"
        finish_reason_value = json.get(finish_reason_key, None)
        finish_reason_metadata = Metadata(value=finish_reason_value)
        finish_reason_dict = {"object": finish_reason_metadata}
        finish_reason_holder = MetadataHolder(object=finish_reason_metadata)
        self.__holders.update({finish_reason_key: finish_reason_holder})


class MCPClient:
    def __init__(self):
        self.session: Optional[ClientSession] = None
        self.exit_stack = AsyncExitStack()
        self.llama = Llama()

    async def connect_to_server(self, server_script_path: str):
        is_python = server_script_path.endswith(".py")
        if not is_python:
            raise ValueError("Server script must be a .py file.")
        command = "python"
        server_params = StdioServerParameters(
            command=command, args=[server_script_path], env=None
        )
        stdio_transport = await self.exit_stack.enter_async_context(
            stdio_client(server_params)
        )
        self.stdio, self.write = stdio_transport
        self.session = await self.exit_stack.enter_async_context(
            ClientSession(self.stdio, self.write)
        )
        await self.session.initialize()
        response = await self.session.list_tools()
        tools = response.tools
        print("\nConnected to server with tools:", [tool.name for tool in tools])

    async def chat_loop(self):
        print("\nMCP Client Started!")
        print("Type your queries or 'quit' to exit.")
        while True:
            try:
                query = input("\nQuery: ").strip()
                if query.lower() == "quit":
                    break
                response = await self.process_query(query)
                print("\n" + response)
            except Exception as e:
                import traceback

                traceback.print_exc()
                print(f"\nError: {str(e)}")

    async def cleanup(self):
        await self.exit_stack.aclose()

    async def process_query(self, query: str):
        messages = [{"role": "user", "content": query}]
        response = await self.session.list_tools()
        available_tools = [
            {
                "type": "function",
                "function": {
                    "name": tool.name,
                    "description": tool.description,
                    "input_schema": tool.inputSchema,
                },
            }
            for tool in response.tools
        ]
        response = self.llama.messages.create(
            model="Qwen3.5-9B-Q4_K_M",
            max_tokens=128000,
            messages=messages,
            tools=available_tools,
        )
        return response


@dataclass(frozen=True)
class DatabasePool:
    database: str = "nutrition"
    host: str = "127.0.0.1"
    password: str = os.environ["POSTGRES_PWD"]
    user: str = "postgres"


class Nutrition:
    def __init__(self):
        db_pool = DatabasePool()
        self.db = Database(**asdict(db_pool))

    async def main(self):
        if len(sys.argv) < 2:
            print("Usage: python client.py server.py")
            sys.exit(1)
        client = MCPClient()
        try:
            await client.connect_to_server(sys.argv[1])
            await client.chat_loop()
        finally:
            await client.cleanup()


if __name__ == "__main__":
    import sys

    nutrition = Nutrition()
    asyncio.run(nutrition.main())
