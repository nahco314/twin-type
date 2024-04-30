import sys

from tools.evaluation import eval_word
from tools.get_words import get_words
from tools.word import Word

import random


def get_good_words_per_level() -> dict[str, list[Word]]:
    all_words = get_words()
    words_per_level = {"easy": [], "medium": [], "hard": []}

    for word in all_words:
        if len(word.romaji) <= 6:
            words_per_level["easy"].append(word)
        elif len(word.romaji) <= 10:
            words_per_level["medium"].append(word)
        else:
            words_per_level["hard"].append(word)

    res = {}

    random.seed(0)

    for level in ("easy", "medium", "hard"):
        res[level] = words_per_level[level].copy()
        random.shuffle(res[level])  # タイブレークが五十音順になると嫌なので事前にシャッフルしておく
        res[level].sort(key=eval_word, reverse=True)
        while len(res[level]) > 200:
            res[level].pop()

    return res


def main() -> None:
    words = get_good_words_per_level()

    res = ""

    for level in ("easy", "medium", "hard"):
        res += f"pub const {level.upper()}_WORDS: [(&str, &str, &str); {len(words[level])}] = [\n"
        for word in words[level]:
            res += f"    (\"{word.word}\", \"{word.hiragana}\", \"{word.romaji}\"),\n"
        res += "];\n\n"

    print(res)

    print("done.", file=sys.stderr)
    print("do `cargo fmt` for the generated file.", file=sys.stderr)


if __name__ == '__main__':
    main()
