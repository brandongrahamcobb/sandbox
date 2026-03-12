from contextlib import AsyncExitStack
from dataclasses import asdict
from llama import Llama
from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client
from typing import Optional
from memory import Memory, Message
from uuid import uuid4


class CustomMCPClient:
    def __init__(self):
        self.session: Optional[ClientSession] = None
        self.exit_stack = AsyncExitStack()
        self.llama = Llama()
        self.memory = Memory()

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
                message = Message(
                    content=query, identity=str(uuid4()), message_type="USER"
                )
                self.memory.add_message(message)
                new_context = self.memory.build_context()
                if query.lower() == "quit":
                    break
                response = await self.process_query(new_context)
                print("\n" + response)
                message = Message(
                    content=response, identity=str(uuid4()), message_type="AI"
                )
                self.memory.add_message(message)
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
                    "parameters": tool.inputSchema,
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
        final_text = []
        assistant_message_content = []
        for content in response.content:
            if content.type == "text":
                final_text.append(content.text)
                assistant_message_content.append(content)
            elif content.type == "tool_use":
                tool_name = content.name
                tool_args = content.input
                result = await self.session.call_tool(tool_name, tool_args)
                final_text.append(f"[Calling tool {tool_name} with args {tool_args}]")
                assistant_message_content.append(
                    f"[Tool call: {content.name} {content.input}]"
                )
                messages.append(
                    {"role": "assistant", "content": assistant_message_content[0]}
                )
                messages.append(
                    {
                        "role": "user",
                        "content": result.content[0].text,
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
                print(messages)
                response = self.llama.messages.create(
                    model="Qwen3.5-9B-Q4_K_M",
                    max_tokens=128000,
                    messages=messages,
                    tools=available_tools,
                )
                final_text.append(response.content[0].text)
        return "\n".join(final_text)
