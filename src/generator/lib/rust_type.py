from typing import Optional, List
from copy import deepcopy


class RustType:
    def __init__(self, name: str, members: Optional[List["RustType"]] = None):
        self.name = name
        self.members = members

    def serde_replace_inner_ty(self, from_to):
        if self.members is None:
            return False

        changed = False
        for i, member in enumerate(self.members):
            if member in from_to:
                self.members[i] = from_to[member]
                changed = True
            else:
                # serde_as fails to compile if type definition includes
                # types without custom serialization
                if not member.serde_replace_inner_ty(from_to):
                    self.members[i] = Base("_")
        return changed

    def serde_as(self) -> "RustType":
        copied = deepcopy(self)
        from_to = {
            Vec(Base("u8")): Base("::client::serde::urlsafe_base64::Wrapper"),
            Base("client::chrono::Duration"): Base("::client::serde::duration::Wrapper"),
            Base("i64"): Base("::client::serde_with::DisplayFromStr"),
            Base("u64"): Base("::client::serde_with::DisplayFromStr"),
        }

        changed = copied.serde_replace_inner_ty(from_to)

        return copied, changed

    def __str__(self):
        if self.members:
            return f"{self.name}<{', '.join(str(m) for m in self.members)}>"
        return self.name

    def __eq__(self, other):
        if not isinstance(other, RustType):
            return False
        return self.name == other.name and self.members == other.members

    def __hash__(self):
        if self.members:
            return hash((self.name, *[(i, v) for i, v in enumerate(self.members)]))
        return hash((self.name, None))

class Option(RustType):
    def __init__(self, member):
        super().__init__("Option", [member])


class Box(RustType):
    def __init__(self, member):
        super().__init__("Box", [member])


class Vec(RustType):
    def __init__(self, member):
        super().__init__("Vec", [member])


class HashMap(RustType):
    def __init__(self, key, value):
        super().__init__("HashMap", [key, value])


class Base(RustType):
    def __init__(self, name):
        super().__init__(name)