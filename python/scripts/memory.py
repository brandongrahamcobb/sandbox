MESSAGE_TYPES = ["SYSTEM", "USER", "TOOL", "AI"]


class Memory:
    def __init__(self):
        self.messages = {}

    def add_message(self, message: Message):
        self.messages[message.identity] = (
            f"{message.message_type} (Ref. ID:{message.identity}): {message.content}"
        )

    def delete_message(self, identity: str):
        del self.messages[identity]

    def build_context(self):
        return "\n".join(self.messages.values())


class Message:
    def __init__(self, content: str, identity: str, message_type: str):
        self._content = content
        self._identity = identity
        self._message_type = message_type

    @property
    def content(self):
        return self._content

    @property
    def identity(self):
        return self._identity

    @property
    def message_type(self):
        return self._message_type

    @message_type.setter
    def message_type(self, nmt: str):
        if nmt in MESSAGE_TYPES:
            self._message_type = nmt
        else:
            raise ValueError(f"Message type must be one of {", ".join(MESSAGE_TYPES)}.")
