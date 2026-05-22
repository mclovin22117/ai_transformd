from __future__ import annotations

from contextlib import contextmanager
from typing import Iterator

import pyperclip


class ClipboardError(Exception):
    pass


class ClipboardManager:
    def read_text(self) -> str:
        try:
            return pyperclip.paste() or ""
        except Exception as exc:
            raise ClipboardError(f"Failed reading clipboard: {exc}") from exc

    def write_text(self, text: str) -> None:
        try:
            pyperclip.copy(text)
        except Exception as exc:
            raise ClipboardError(f"Failed writing clipboard: {exc}") from exc

    @contextmanager
    def preserve(self) -> Iterator[None]:
        original = self.read_text()
        try:
            yield
        finally:
            # Best effort restore. Never raise from finally to avoid masking root errors.
            try:
                self.write_text(original)
            except Exception:
                pass
