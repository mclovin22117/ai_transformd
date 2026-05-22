from __future__ import annotations

import shutil
import subprocess
import time


class ReplacerError(Exception):
    pass


class Replacer:
    """Input simulation wrapper for X11 using xdotool."""

    def __init__(self, copy_wait_ms: int = 120, paste_wait_ms: int = 120) -> None:
        self.copy_wait_ms = copy_wait_ms
        self.paste_wait_ms = paste_wait_ms
        self._xdotool = shutil.which("xdotool")
        if not self._xdotool:
            raise ReplacerError("xdotool is required for X11 MVP but was not found in PATH")

    def trigger_copy(self) -> None:
        subprocess.run(
            [self._xdotool, "key", "--clearmodifiers", "ctrl+c"],
            check=False,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )
        time.sleep(self.copy_wait_ms / 1000)

    def trigger_paste(self) -> None:
        subprocess.run(
            [self._xdotool, "key", "--clearmodifiers", "ctrl+v"],
            check=False,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )
        time.sleep(self.paste_wait_ms / 1000)
