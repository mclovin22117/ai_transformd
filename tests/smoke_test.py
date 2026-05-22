from __future__ import annotations

import os
import sys
import unittest

sys.path.insert(0, os.path.dirname(os.path.dirname(__file__)))

from config import AppConfig
from prompts import DEFAULT_PROMPTS, get_prompt


class SmokeTest(unittest.TestCase):
    def test_default_config_has_expected_hotkey(self) -> None:
        cfg = AppConfig()
        self.assertEqual(cfg.hotkeys["grammar_fix"], "<ctrl>+<alt>+f")

    def test_prompt_lookup(self) -> None:
        prompt = get_prompt("grammar_fix", DEFAULT_PROMPTS)
        self.assertIn("Fix grammar", prompt)


if __name__ == "__main__":
    unittest.main()
