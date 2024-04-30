from tools.word import Word


KB_SHARING = """\
12345|67890-
QWERT|YUIOP
ASDFG|HJKL
ZXCVB|NM,.
"""


LEFT_CHARS = []
RIGHT_CHARS = []
for line in KB_SHARING.splitlines():
    l, _, r = line.partition("|")
    LEFT_CHARS.extend(l.lower())
    RIGHT_CHARS.extend(r.lower())


def eval_word(word: Word) -> float:
    left_cnt = 0
    right_cnt = 0
    lr_lst = []
    for c in word.romaji:
        if c in LEFT_CHARS:
            left_cnt += 1
            lr_lst.append(0)
        elif c in RIGHT_CHARS:
            right_cnt += 1
            lr_lst.append(1)
        else:
            assert False, (c, word)

    border_cnt = 0
    for i in range(len(lr_lst) - 1):
        if lr_lst[i] != lr_lst[i + 1]:
            border_cnt += 1

    return border_cnt - abs(left_cnt - right_cnt)
