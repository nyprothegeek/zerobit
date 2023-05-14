"""
This module defines the class for accessing public Anthropic AI models.
"""

from .base import BaseLM


class AnthropicLM(BaseLM[str]):
    """
    A class for accessing public Anthropic AI models.
    """

    def __init__(self, temperature: float):
        """
        Initializes a new instance of the AnthropicLM class.
        """
