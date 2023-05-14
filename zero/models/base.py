"""
This module defines the base classes for different types of models.
"""

from abc import ABC, abstractmethod
from typing import Generic, TypeVar


T = TypeVar("T")


class BaseModel(ABC):
    """
    The base class for all models.
    """


class BaseLM(Generic[T], BaseModel, ABC):
    """
    The base class for all language models.
    """

    @abstractmethod
    async def prompt(self, prompt: T) -> T:
        """
        Prompts the model with the given prompt and returns the result.
        """
