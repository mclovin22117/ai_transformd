from __future__ import annotations

import shutil
import subprocess


class Notifier:
    def __init__(self, enabled: bool = True) -> None:
        self.enabled = enabled
        self._notify_send = shutil.which("notify-send")

    def info(self, title: str, message: str) -> None:
        self._send(title, message)

    def error(self, title: str, message: str) -> None:
        self._send(title, message, urgency="critical")

    def _send(self, title: str, message: str, urgency: str = "normal") -> None:
        if not self.enabled:
            return
        if self._notify_send:
            subprocess.run(
                [self._notify_send, "-u", urgency, title, message],
                check=False,
            )
        else:
            print(f"[{title}] {message}")
