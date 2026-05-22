DEFAULT_PROMPTS = {
    "grammar_fix": "Fix grammar and spelling only. Preserve meaning and tone.",
    "professional_rewrite": "Rewrite professionally while preserving intent.",
    "summarize": "Summarize concisely.",
    "translate_en": "Translate to English.",
}


def get_prompt(command_name: str, prompt_map: dict[str, str]) -> str:
    return prompt_map.get(command_name, DEFAULT_PROMPTS["grammar_fix"])
