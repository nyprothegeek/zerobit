"""
Tests for the OpenAI models.
"""

from zero.models import OpenAILM


def test_openai_lm_has_right_defaults() -> None:
    """
    Tests that OpenAI language model has the right defaults.
    """

    model = OpenAILM(model="gpt-3.5-turbo")
    assert model.model == "gpt-3.5-turbo"
    assert model.temperature == 1.0
    assert model.max_tokens == 16
    assert model.top_p is None
    assert model.stop == "\n"


def test_openai_model_() -> None:
    """
    Tests thate OpenAI language model
    """

    # model = OpenAILM(model="gpt-3.5-turbo")
