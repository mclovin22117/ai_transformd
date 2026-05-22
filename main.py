from __future__ import annotations

import asyncio
import signal
from typing import Optional

from clipboard_manager import ClipboardError, ClipboardManager
from config import load_config
from hotkeys import HotkeyManager
from llm_client import LLMClient
from notifier import Notifier
from prompts import get_prompt
from replacer import Replacer, ReplacerError


class TransformApp:
    def __init__(self) -> None:
        self.config = load_config()
        self.notifier = Notifier(enabled=self.config.notifications_enabled)
        self.clipboard = ClipboardManager()
        self.replacer = Replacer(
            copy_wait_ms=self.config.copy_wait_ms,
            paste_wait_ms=self.config.paste_wait_ms,
        )
        self.client = LLMClient(
            provider=self.config.provider,
            model=self.config.model,
            timeout_s=self.config.request_timeout_s,
            ollama_endpoint=self.config.ollama_endpoint,
        )

        self._queue: asyncio.Queue[str] = asyncio.Queue()
        self._stop_event = asyncio.Event()
        self._loop: Optional[asyncio.AbstractEventLoop] = None
        self._hotkeys = HotkeyManager(
            hotkey_map=self.config.hotkeys,
            on_command=self.enqueue_command,
        )

    def enqueue_command(self, command_name: str) -> None:
        if self._loop is None:
            return
        self._loop.call_soon_threadsafe(self._queue.put_nowait, command_name)

    async def handle_command(self, command_name: str) -> None:
        # MVP target: grammar fix hotkey command. Other commands are configured and ready.
        prompt = get_prompt(command_name, self.config.prompts)

        self.notifier.info("AI Transformer", "Processing selection...")

        try:
            with self.clipboard.preserve():
                self.replacer.trigger_copy()
                selected = self.clipboard.read_text().strip()
                if not selected:
                    self.notifier.error("AI Transformer", "No text selected")
                    return

                transformed = await self.client.transform(selected, prompt)
                transformed = transformed.strip()
                if not transformed:
                    self.notifier.error("AI Transformer", "Received empty model response")
                    return

                self.clipboard.write_text(transformed)
                self.replacer.trigger_paste()

            self.notifier.info("AI Transformer", "Completed")
        except ClipboardError as exc:
            self.notifier.error("AI Transformer", f"Clipboard error: {exc}")
        except TimeoutError:
            self.notifier.error("AI Transformer", "Request timed out")
        except Exception as exc:
            self.notifier.error("AI Transformer", f"Error: {exc}")

    async def worker(self) -> None:
        while not self._stop_event.is_set():
            command_name = await self._queue.get()
            await self.handle_command(command_name)

    async def run(self) -> None:
        self._loop = asyncio.get_running_loop()
        self.notifier.info("AI Transformer", "Running. Select text and press a hotkey.")
        self._hotkeys.start()
        try:
            await self.worker()
        finally:
            self._hotkeys.stop()


async def _async_main() -> None:
    try:
        app = TransformApp()
    except ReplacerError as exc:
        print(f"Startup error: {exc}")
        return

    loop = asyncio.get_running_loop()

    stop_called = False

    def _shutdown(*_: object) -> None:
        nonlocal stop_called
        if stop_called:
            return
        stop_called = True
        for task in asyncio.all_tasks(loop):
            if task is not asyncio.current_task(loop):
                task.cancel()

    loop.add_signal_handler(signal.SIGINT, _shutdown)
    loop.add_signal_handler(signal.SIGTERM, _shutdown)

    try:
        await app.run()
    except asyncio.CancelledError:
        pass


def main() -> None:
    asyncio.run(_async_main())


if __name__ == "__main__":
    main()
