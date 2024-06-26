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

    extra_level_lst = []

    extras = """
晴れに晴れ、花よ咲け/はれにはれ、はなよさけ/ヨルシカ - 晴る

だから僕は音楽を辞めた/だからぼくはおんがくをやめた/ヨルシカ - だから僕は音楽を辞めた

負け犬にアンコールはいらない/まけいぬにあんこーるはいらない/ヨルシカ - 負け犬にアンコールはいらない

人生ごとマシンガン、消し飛ばしてもっと/じんせいごとましんがん、けしとばしてもっと/ヨルシカ - 夜紛い

淡い月に見とれてしまうから/あわいつきにみとれてしまうから/n-buna - 夜明けと蛍

夢を叶えるにもお金がいる/ゆめをかなえるにもおかねがいる/n-buna - 始発とカフカ

ずっと真夜中でいいのに。/ずっとまよなかでいいのに。/アーティスト名

このまま奪って 隠して 忘れたい/このままうばってかくしてわすれたい/ずっと真夜中でいいのに。 - 秒針を噛む

どうでもいいから置いてった　あいつら全員同窓会/どうでもいいからおいてったあいつらぜんいんどうそうかい/ずっと真夜中でいいのに。 - あいつら全員同窓会

啖呵切って 寝るふりして/たんかきってねるふりして/ずっと真夜中でいいのに。 - グラスとラムレーズン

君のすべてがあたしならいいのに/きみのすべてがあたしならいいのに/理芽 - 食虫植物

夏の歩幅に置いていかれないように/なつのほはばにおいていかれないように/笹川真生 - 滞る夜

くらい ちかい たかい夜/くらいちかいたかいよる/ぺんぎんの憂鬱 - 落ちる

いかないよって駄々をこねる/いかないよってだだをこねる/ぺんぎんの憂鬱 - SCR

緩やかに崩れ壊れてく/ゆるやかにくずれこわれてく/Reol - No Title

駆け出したいの 明日まで ひとっ飛び/かけだしたいのあしたまでひとっとび/Reol - drop pop candy

「もう一度どこかで会えたらいいな」って/もういちどどこかであえたらいいなって/秋山黄色 - 猿上がりシティーポップ

色水になってく 甘い甘いそれは/いろみずになってくあまいあまいそれは/おいしくるメロンパン - 色水

あなたの涙を呑んで　あなたの吐息を吸って/あなたのなみだをのんであなたのといきをすって/おいしくるメロンパン - look at the sea

ねぇミスターサンデー　待って、かまって/ねぇみすたーさんでーまってかまって/カラスは真っ白 - fake!fake!

時計の針が止まって見える現象のことだよ/とけいのはりがとまってみえるげんしょうのことだよ/きのこ帝国 - クロノスタシス

マーシャルの匂いで飛んじゃって大変さ/まーしゃるのにおいでとんじゃってたいへんさ/椎名林檎 - 丸の内サディスティック

太平洋 大西洋 ここ一体何平洋よ/たいへいようたいせいようここいったいなにへいようよ/相対性理論 - スマトラ警備隊

タクシー 飛ばしてよ九龍からニューヨークへ/たくしーとばしてよくーろんからにゅーよーくへ/相対性理論 - 気になるあの娘

三千万年前から 恋してるの/さんぜんまんねんまえからこいしてるの/相対性理論 - 三千万年

きっとね！秘密は多い方が　どうだろう！優しくなれるかも/きっとねひみつはおおいほうがどうだろうやさしくなれるかも/中村佳穂 - きっとね!

彗星ハネムーン　あなたとランデヴ/すいせいはねむーんあなたとらんでゔ/ナユタン星人 - 彗星ハネムーン

赫い髪の少女は早足の男に手を引かれ/あかいかみのしょうじょははやあしのおとこにてをひかれ/ナンバーガール - 透明少女

現代。冷凍都市に住む妄想人類諸君に告ぐ/げんだい。れいとうとしにすむもうそうじんるいしょくんにつぐ/ナンバーガール - Num-Ami-Dabutz

風 鋭くなって 都会の少女はにっこり笑う！！/かぜするどくなってとかいのしょうじょはにっこりわらう/ナンバーガール - 鉄風 鋭くなって

あなたのお母さんは鏡の向こうで笑っている/あなたのおかあさんはかがみのむこうでわらっている/ZAZEN BOYZ - 永遠少女

タイト、ト、ト、ト、トンと僕らは銀座でヘヘイヘイ/たいと、と、と、と、とんとぼくらはぎんざでへへいへい/ネクライトーキー - オシャレ大作戦

ゆるふわ樹海ガールは　今日も笑って元気/ゆるふわじゅかいがーるはきょうもわらってげんき/石風呂 - ゆるふわ樹海ガール

愛を 謳って 謳って 雲の上/あいをうたってうたってくものうえ/バルーン - シャルル

その振動は確かに花瓶に触れた/そのしんどうはたしかにかびんにふれた/バルーン - 花瓶に触れた

いつもその少女は右に曲ガール/いつもそのしょうじょはみぎにまがーる/はるふり - 右に曲ガール

嗚呼 テレキャスター・ストライプ/ああてれきゃすたーすとらいぷ/ポルカドットスティングレイ - テレキャスター・ストライプ

金曜午後からの高揚/きんようごごからのこうよう/ポルカドットスティングレイ - ばけものだらけの街

ポルカドットスティングレイ/ぽるかどっとすてぃんぐれい/アーティスト名

ちょっと病弱なセブンティーン/ちょっとびょうじゃくなせぶんてぃーん/米津玄師 - ゴーゴー幽霊船

上手く笑えないんだ どうしようもないまんま/うまくわらえないんだどうしようもないまんま/ハチ - ドーナツホール

もっと　騒げ怪獣の歌/もっとさわげかいじゅうのうた/Vaundy - 怪獣の花唄

ねぇ、どっかに置いてきたような/ねぇ、どっかにおいてきたような/Vaundy - 踊り子

間違いだって起こしちゃおうと誘う、坂道。/まちがいだっておこしちゃおうとさそう、さかみち。/wowaka - ローリンガール

「君の言うロジック、証明できたって何になるの？」/きみのいうろじっく、しょうめいできたってなにになるの/Oh No Darkness!! - フレア

此処に春が咲くまで眠らせて。/ここにはるがさくまでねむらせて。/文藝天国 - 宿命論とチューリップ

考えることに疲れたから　今、少しだけね泣いて/かんがえることにつかれたからいま、すこしだけねないて/ハク。 - 回転してから考える

平衡感覚を保つ機能ぶっ壊して/へいこうかんかくをたもつきのうぶっこわして/SAKANAMON - ミュージックプランクトン

ほら味方につけろよ　街の群れを/ほらみかたにつけろよまちのむれを/tricot - POOL

あたしだけのものになんて　いつまでもならないで/あたしだけのものになんていつまでもならないで/tricot - potage

僕はいつか貴方の恋人になりたい/ぼくはいつかあなたのこいびとになりたい/チョーキューメイ - 貴方の恋人になりたい

大人のあなたは煙草　私はシャボン玉/おとなのあなたはたばこわたしはしゃぼんだま/ラブリーサマーちゃん - あなたは煙草 私はシャボン

リッケンバッカーが歌う/りっけんばっかーがうたう/リーガルリリー - リッケンバッカー
"""

    for l in extras.splitlines():
        if l == "":
            continue

        ls = l.split("/")
        extra_level_lst.append(ls)

    res += f"pub const EXTRA_HARD_WORDS: [(&str, &str, &str); {len(extra_level_lst)}] = [\n"
    for word in extra_level_lst:
        res += f"    (\"{word[0]}\", \"{word[1]}\", \"{word[2]}\"),\n"
    res += "];\n\n"

    print(res)

    print("done.", file=sys.stderr)
    print("do `cargo fmt` for the generated file.", file=sys.stderr)


if __name__ == '__main__':
    main()
