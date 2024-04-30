from dataclasses import dataclass


@dataclass
class Word:
    word: str
    hiragana: str
    romaji: str
