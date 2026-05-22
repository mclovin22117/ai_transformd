from __future__ import annotations

import httpx


class OllamaProvider:
    def __init__(self, model: str, endpoint: str = "http://localhost:11434", timeout_s: int = 20) -> None:
        self.model = model
        self.endpoint = endpoint.rstrip("/")
        self.timeout_s = timeout_s

    async def transform(self, text: str, prompt: str) -> str:
        payload = {
            "model": self.model,
            "prompt": f"{prompt}\n\nText:\n{text}",
            "stream": False,
        }
        async with httpx.AsyncClient(timeout=self.timeout_s) as client:
            response = await client.post(f"{self.endpoint}/api/generate", json=payload)
            response.raise_for_status()
            body = response.json()
            result = body.get("response", "").strip()
            if not result:
                raise RuntimeError("Ollama returned empty response")
            return result
