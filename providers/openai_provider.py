from __future__ import annotations

import os
from typing import Optional

from openai import AsyncOpenAI


class OpenAIProvider:
    def __init__(
        self,
        model: str,
        timeout_s: int,
        api_key: Optional[str] = None,
    ) -> None:
        key = api_key or os.getenv("OPENAI_API_KEY")
        if not key:
            raise ValueError("OPENAI_API_KEY is not set")

        self.model = model
        self.client = AsyncOpenAI(api_key=key, timeout=timeout_s)

    async def transform(self, text: str, prompt: str) -> str:
        response = await self.client.responses.create(
            model=self.model,
            input=[
                {"role": "system", "content": prompt},
                {"role": "user", "content": text},
            ],
        )

        output_text = getattr(response, "output_text", None)
        if output_text:
            return output_text.strip()

        # Fallback parsing for SDK/version differences
        try:
            content = response.output[0].content[0].text  # type: ignore[attr-defined]
            return content.strip()
        except Exception as exc:
            raise RuntimeError("OpenAI response did not contain output text") from exc
