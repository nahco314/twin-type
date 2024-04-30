import urllib.request
import csv

from successive_romaji import parse_hiragana, parse_hiragana_with_buf, try_read

from tools.word import Word


def get_words() -> list[Word]:
    csv_url = "http://nihongo-net.jp/voc_list_kiso.csv"
    req = urllib.request.Request(csv_url)
    with urllib.request.urlopen(req) as res:
        csv_text = res.read().decode("shift-jis")

    csv_reader = csv.reader(csv_text.splitlines())
    words = []

    for row in csv_reader:
        _, num, word, hiragana, type_, _ = row

        if "動詞" in type_ or "名詞" in type_:
            try:
                romaji = "".join(map(lambda x: x[1], parse_hiragana(hiragana)))
            except ValueError:
                continue

            words.append(Word(word, hiragana, romaji))

    return words


if __name__ == '__main__':
    # print(get_words())
    print(parse_hiragana_with_buf("あんにん", ""))
