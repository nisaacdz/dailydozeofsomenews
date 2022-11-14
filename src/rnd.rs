use rand::Rng;

fn get_word(len: usize) -> String {
    let mut word = String::new();

    for _ in 0..len {
        let ch: u8 = rand::thread_rng().gen_range(97..123);
        word.push(ch as char);
    }

    word
}

pub fn write_word() -> String {
    let mut word = String::new();

    let ch: u8 = rand::thread_rng().gen_range(65..91);
    word.push(ch as char);
    word.push_str(get_word(3).as_str());

    word
}

pub fn write_sentence(mut len: usize) -> String {
    let mut sentence = String::new();

    let ch: u8 = rand::thread_rng().gen_range(65..91);
    sentence.push(ch as char);
    sentence.push_str(get_word(3).as_str());
    len -= 1;

    for _ in 0..len {
        sentence.push(' ');
        sentence.push_str(get_word(5).as_str());
    }

    sentence
}
