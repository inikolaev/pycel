from typing import Union

class CelInt:
    def __init__(self, value: int) -> None:
        self.value = value


class CelFloat:
    def __init__(self, value: float) -> None:
        self.value = value


class CelBool:
    def __init__(self, value: bool) -> None:
        self.value = value


class CelString:
    def __init__(self, value: str) -> None:
        self.value = value


class CelList:
    def __init__(self, value: list[Union[CelInt, CelFloat, CelBool, CelString, "CelList", "CelMap"]]) -> None:
        self.value = value


class CelMap:
    def __init__(self, value: dict[str, Union[CelInt, CelFloat, CelBool, CelString, CelList, "CelMap"]]) -> None:
        self.value = value