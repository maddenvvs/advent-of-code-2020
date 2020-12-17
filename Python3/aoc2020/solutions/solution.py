from abc import ABC, abstractmethod


class Solution(ABC):

    @abstractmethod
    def first_task(self, input_text: str) -> str:
        pass

    @abstractmethod
    def second_task(self, input_text: str) -> str:
        pass
