from __future__ import annotations

from providers.ollama_provider import OllamaProvider
from providers.openai_provider import OpenAIProvider


class LLMClient:
    def __init__(self, provider: str, model: str, timeout_s: int, ollama_endpoint: str) -> None:
        provider_name = provider.lower()
        if provider_name == "openai":
            self.provider = OpenAIProvider(model=model, timeout_s=timeout_s)
        elif provider_name == "ollama":
            self.provider = OllamaProvider(model=model, endpoint=ollama_endpoint, timeout_s=timeout_s)
        else:
            raise ValueError(f"Unsupported provider: {provider}")

    async def transform(self, text: str, prompt: str) -> str:
        return await self.provider.transform(text=text, prompt=prompt)
