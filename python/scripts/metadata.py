from dataclasses import dataclass, field
from typing import Any, TypeVar, Generic

T = TypeVar("T")


class MetadataType(Generic[T]):
    pass


@dataclass(frozen=True)
class Metadata(MetadataType[T]):
    value: Any


@dataclass(frozen=True)
class MetadataHolder:
    object: Metadata


@dataclass(frozen=True)
class MetadataContainer:
    holders: dict[str, MetadataHolder] = field(default_factory=dict)
