from __future__ import annotations

import json
import os
from dataclasses import dataclass, field
from pathlib import Path
from typing import Dict

from prompts import DEFAULT_PROMPTS


@dataclass
class AppConfig:
    provider: str = "openai"
    model: str = "gpt-4.1-mini"
    ollama_endpoint: str = "http://localhost:11434"
    hotkeys: Dict[str, str] = field(
        default_factory=lambda: {
            "grammar_fix": "<ctrl>+<alt>+f",
            "professional_rewrite": "<ctrl>+<alt>+r",
            "summarize": "<ctrl>+<alt>+s",
            "translate_en": "<ctrl>+<alt>+t",
        }
    )
    prompts: Dict[str, str] = field(default_factory=lambda: DEFAULT_PROMPTS.copy())
    copy_wait_ms: int = 120
    paste_wait_ms: int = 120
    request_timeout_s: int = 20
    notifications_enabled: bool = True


DEFAULT_CONFIG_PATH = Path.home() / ".config" / "ai_text_transformer" / "config.json"


def _merge_dict(base: dict, override: dict) -> dict:
    merged = dict(base)
    for key, value in override.items():
        if isinstance(value, dict) and isinstance(merged.get(key), dict):
            merged[key] = _merge_dict(merged[key], value)
        else:
            merged[key] = value
    return merged


def load_config(path: Path | None = None) -> AppConfig:
    config_path = path or Path(os.getenv("AIT_CONFIG", DEFAULT_CONFIG_PATH))

    cfg = AppConfig()
    if not config_path.exists():
        return cfg

    with config_path.open("r", encoding="utf-8") as f:
        loaded = json.load(f)

    merged = _merge_dict(cfg.__dict__, loaded)
    return AppConfig(**merged)
