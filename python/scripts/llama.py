from dataclasses import asdict, dataclass, field
from metadata import Metadata, MetadataContainer, MetadataHolder
from typing import Any
import json
import requests


def json_to_dict(content: str):
    dictionary = json.loads(content)
    return dictionary


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
        if id_value:
            id_metadata = Metadata(value=id_value)
            id_holder = MetadataHolder(object=id_metadata)
            self.__holders.update({id_key: id_holder})

    def json_to_total_tokens(self, json: dict[str, Any]):
        total_tokens_key = "total_tokens"
        total_tokens_value = json.get(total_tokens_key, None)
        if total_tokens_value:
            total_tokens_metadata = Metadata(value=total_tokens_value)
            total_tokens_holder = MetadataHolder(object=total_tokens_metadata)
            self.__holders.update({total_tokens_key: total_tokens_holder})

    def json_to_model(self, json: dict[str, Any]):
        model_key = "model"
        model_value = json.get(model_key, None)
        if model_value:
            model_metadata = Metadata(value=model_value)
            model_holder = MetadataHolder(object=model_metadata)
            self.__holders.update({model_key: model_holder})

    def json_to_created(self, json: dict[str, Any]):
        created_key = "created"
        created_value = json.get(created_key, None)
        if created_value:
            created_metadata = Metadata(value=created_value)
            created_holder = MetadataHolder(object=created_metadata)
            self.__holders.update({created_key: created_holder})

    def json_to_usage(self, json: dict[str, Any]):
        usage_key = "usage"
        usage_value = json.get(usage_key, None)
        if usage_value:
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
        if role_value:
            role_metadata = Metadata(value=role_value)
            role_holder = MetadataHolder(object=role_metadata)
            self.__holders.update({role_key: role_holder})
        content_key = "content"
        content_value = message_value.get(content_key, None)
        if content_value:
            content_metadata = Metadata(value=content_value)
            content_holder = MetadataHolder(object=content_metadata)
            self.__holders.update({content_key: content_holder})
        tool_calls_key = "tool_calls"
        tool_calls_value = message_value.get(tool_calls_key, None)
        type_key = "type"
        if tool_calls_value:
            type_metadata = Metadata(value="tool_use")
        else:
            type_metadata = Metadata(value="text")
        type_holder = MetadataHolder(object=type_metadata)
        self.__holders.update({type_key: type_holder})

    def json_to_finish_reason(self, json: dict[str, Any]):
        finish_reason_key = "finish_reason"
        finish_reason_value = json.get(finish_reason_key, None)
        if finish_reason_value:
            finish_reason_metadata = Metadata(value=finish_reason_value)
            finish_reason_holder = MetadataHolder(object=finish_reason_metadata)
            self.__holders.update({finish_reason_key: finish_reason_holder})


@dataclass(frozen=True)
class Content:
    id: str
    type: str
    input: dict[str, str] = field(default_factory=dict)
    name: str | None = None
    text: str | None = None


@dataclass(frozen=True)
class Response:
    content: list[Content]


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
        contents = []
        query = self.build_query(
            model=model, max_tokens=max_tokens, messages=messages, tools=tools
        )
        x = requests.post("http://localhost:8080/v1/chat/completions", query)
        json_dict = json_to_dict(x.content)
        id_key = "id"
        id = json_dict.get(id_key, None)
        choices_key = "choices"
        choices_value = json_dict.get(choices_key, None)
        message_key = "message"
        message_value = choices_value[0].get(message_key, None)
        if tool_calls := message_value.get("tool_calls", None):
            for tool_call in tool_calls:
                name = tool_call.get("function", None).get("name", None)
                args = json.loads(
                    tool_call.get("function", None).get("arguments", None)
                )
                contents.append(Content(type="tool_use", id=id, name=name, input=args))
        if content := message_value.get("content", None):
            contents.append(Content(type="text", id=id, text=content))
        # llama = Llama()
        # container = llama.load(json=json_dict)
        #
        return Response(contents)  # asdict(container)["holders"]
