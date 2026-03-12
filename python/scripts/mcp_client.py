from contextlib import AsyncExitStack
from llama import Llama
from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client
from typing import Optional


class CustomMCPClient:
    def __init__(self):
        self.session: Optional[ClientSession] = None
        self.exit_stack = AsyncExitStack()
        self.llama = Llama()

    async def __aenter__(self):
        await self.connect_to_server("server.py")
        return self

    async def __aexit__(self, exc_type, exc_val, exc_tb):
        await self.cleanup()

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
