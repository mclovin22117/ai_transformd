# Ubuntu AI Text Transformer (Python MVP)

Lightweight Ubuntu desktop utility that transforms selected text via global hotkey.

MVP target (Stage 1):
- Ubuntu X11
- Global hotkey: `Ctrl+Alt+F`
- Workflow: select text -> hotkey -> copy -> OpenAI transform -> paste replacement
- Clipboard is preserved and restored

## Architecture (Stage 1)

Core modules:
- `main.py`: app controller and async worker loop
- `hotkeys.py`: global hotkey listener (`pynput`)
- `clipboard_manager.py`: clipboard read/write and preserve/restore wrapper
- `replacer.py`: X11 input simulation using `xdotool`
- `llm_client.py`: provider abstraction entry point
- `prompts.py`: default prompt map
- `notifier.py`: desktop notifications (`notify-send`)
- `config.py`: JSON config loading with defaults
- `providers/openai_provider.py`: OpenAI SDK provider
- `providers/ollama_provider.py`: optional local Ollama provider

Privacy posture:
- No keylogging
- No global text interception
- Only selected text at explicit hotkey trigger is processed
- Clipboard content is restored after replacement
- App does not persist user text

## Linux Dependencies (Ubuntu)

```bash
sudo apt update
sudo apt install -y xdotool xclip wl-clipboard libnotify-bin
```

Notes:
- MVP is X11-first (`xdotool` relies on X11)
- Wayland support is planned separately

## Python Setup

```bash
cd /home/babayaga/Documents/ai_text_transformation
python3 -m venv .venv
source .venv/bin/activate
pip install -U pip
pip install -r requirements.txt
```

## Config

Copy and edit config:

```bash
mkdir -p ~/.config/ai_text_transformer
cp config.example.json ~/.config/ai_text_transformer/config.json
```

Required for OpenAI provider:

```bash
export OPENAI_API_KEY="your_key_here"
```

## Run

```bash
cd /home/babayaga/Documents/ai_text_transformation
source .venv/bin/activate
python main.py
```

Then:
1. Select text in any app
2. Press `Ctrl+Alt+F`
3. Selected text is replaced with transformed output

## Test Checklist (MVP)

1. Select a sentence with grammar issues in a text field
2. Press `Ctrl+Alt+F`
3. Confirm desktop notification shows "Processing"
4. Confirm selection is replaced
5. Confirm your previous clipboard is restored afterward

Failure handling you can validate:
- Empty selection -> "No text selected"
- Invalid/absent key -> error notification
- API/network failures -> error notification
