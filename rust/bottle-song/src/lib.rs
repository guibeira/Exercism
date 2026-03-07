const NUMBER_WORDS: [&str; 11] = [
    "Zero", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
];
const WALL_MSG: &str = "And if one green bottle should accidentally fall,\n";

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut msg = String::new();
    for i in 0..take_down {
        let mut remain_bottles = start_bottles - i;
        for _index in 1..=2 {
            let phrase = if remain_bottles == 1 {
                "One green bottle hanging on the wall,\n".to_string()
            } else {
                let remain_word = NUMBER_WORDS[remain_bottles as usize];
                format!("{} green bottles hanging on the wall,\n", remain_word)
            };
            msg.push_str(phrase.as_str());
        }
        remain_bottles -= 1;
        msg.push_str(WALL_MSG);
        let remain_msg = if remain_bottles == 0 {
            "There'll be no green bottles hanging on the wall.\n".to_string()
        } else if remain_bottles == 1 {
            "There'll be one green bottle hanging on the wall.\n".to_string()
        } else {
            let remain_number_work: String = NUMBER_WORDS[remain_bottles as usize].to_lowercase();
            format!(
                "There'll be {} green bottles hanging on the wall.\n",
                remain_number_work
            )
        };
        msg.push_str(remain_msg.as_str());
        msg.push('\n');
    }
    msg
}
