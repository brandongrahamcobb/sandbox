from contextlib import AsyncExitStack
from dotenv import load_dotenv
from llama import Llama
from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client
from typing import Optional
import os


class CustomMCPClient:
    def __init__(self):
        load_dotenv()
        self.session: Optional[ClientSession] = None
        self.exit_stack = AsyncExitStack()
        self.server_script_path = os.environ["MCP_SERVER_PATH"]
        self.llama = Llama()
        self.messages = []

    async def __aenter__(self):
        await self.connect_to_server(self.server_script_path)
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
        self.messages.append({"role": "user", "content": query})
        response = await self.session.list_tools()
        available_tools = [
            {
                "type": "function",
                "function": {
                    "name": tool.name,
                    "description": tool.description,
                    "parameters": tool.inputSchema,
                },
            }
            for tool in response.tools
        ]
        response = self.llama.messages.create(
            model="Qwen3.5-9B-Q4_K_M",
            max_tokens=128000,
            messages=self.messages,
            tools=available_tools,
        )
        print(response)
        final_text = []
        assistant_message_content = []
        for content in response.content:
            if content.type == "text":
                final_text.append(content.text)
                assistant_message_content.append(content.text)
                self.messages.append(
                    {"role": "assistant", "content": assistant_message_content[-1]}
                )
            elif content.type == "tool_use":
                tool_name = content.name
                tool_args = content.input
                result = await self.session.call_tool(tool_name, tool_args)
                final_text.append(f"[Calling tool {tool_name} with args {tool_args}]")
                assistant_message_content.append(
                    f"[Tool call: {content.name} {content.input}]"
                )
                self.messages.append(
                    {"role": "assistant", "content": assistant_message_content[-1]}
                )
                self.messages.append(
                    {
                        "role": "user",
                        "content": result.content[-1].text,
                        # "content": "test",
                        # [
                        #     {
                        #         "type": "tool_result",
                        #         "tool_use_id": content.id,
                        #         "content": [entry.text for entry in result.content],
                        #     }
                        # ],
                    }
                )
                response = self.llama.messages.create(
                    model="Qwen3.5-9B-Q4_K_M",
                    max_tokens=128000,
                    messages=self.messages,
                    tools=available_tools,
                )
                print(response)
                final_text.append(response.content[0].text)
        return "\n".join(final_text)
