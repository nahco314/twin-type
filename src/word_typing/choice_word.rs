use crate::in_game::Level;
use crate::word_typing::wordbook::*;
use rand::seq::SliceRandom;

pub fn choice_word(level: Level, used_word_indexes: &Vec<usize>) -> (String, String, String) {
    let words: Vec<_> = match level {
        Level::Easy => EASY_WORDS.iter().collect(),
        Level::Medium => MEDIUM_WORDS.iter().collect(),
        Level::Hard => HARD_WORDS.iter().collect(),
        Level::ExtraHard => EXTRA_HARD_WORDS.iter().collect(),
    };

    let mut rng = rand::thread_rng();
    let mut nums: Vec<usize> = (0..words.len()).collect();
    nums.shuffle(&mut rng);

    for i in nums {
        if !used_word_indexes.contains(&i) {
            let (a, b, c) = words[i];
            return (a.to_string(), b.to_string(), c.to_string());
        }
    }

    panic!("No more words ><")
}
