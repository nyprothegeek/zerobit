"""
This module defines the class for accessing public OpenAI models.
"""


from typing import Literal, Optional
from .base import BaseLM


class OpenAILM(BaseLM[str]):
    """
    A class for accessing public OpenAI models.
    """

    # TODO(appcypher): Add more models.
    Model = Literal["ada", "babbage", "curie", "davinci", "gpt-3.5-turbo"]

    def __init__(
        self,
        model: Model,
        max_tokens: Optional[int] = 16,
        top_p: Optional[float] = None,
        temperature: Optional[float] = 1.0,
        stop: Optional[str] = "\n",
    ):
        """
        Initializes a new instance of the OpenAILM class.
        """

        self.model = model
        self.temperature = temperature
        self.max_tokens = max_tokens
        self.top_p = top_p
        self.stop = stop

    async def prompt(self, prompt: str) -> str:
        """
        Prompts the model with the given prompt and returns the result.
        """

        return prompt
