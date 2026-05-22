from __future__ import annotations

from typing import Callable, Dict

from pynput import keyboard


class HotkeyManager:
    def __init__(self, hotkey_map: Dict[str, str], on_command: Callable[[str], None]) -> None:
        self.hotkey_map = hotkey_map
        self.on_command = on_command
        self._listener: keyboard.GlobalHotKeys | None = None

    def start(self) -> None:
        bindings = {
            combo: (lambda cmd=cmd_name: self.on_command(cmd))
            for cmd_name, combo in self.hotkey_map.items()
        }
        self._listener = keyboard.GlobalHotKeys(bindings)
        self._listener.start()

    def join(self) -> None:
        if self._listener is not None:
            self._listener.join()

    def stop(self) -> None:
        if self._listener is not None:
            self._listener.stop()
