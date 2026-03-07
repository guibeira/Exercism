pub fn abbreviate(phrase: &str) -> String {
    let words: Vec<String> = phrase
        .replace("-", " ")
        .replace("_", " ")
        .split(" ")
        .filter(|c| !c.is_empty())
        .map(|c| c.to_string())
        .map(|c| c.trim().to_string())
        .collect();
    
    let mut result = "".to_string();
    for word in words{
        let letters: Vec<String> = word.chars().map(| c| c.to_string()).collect();
        let first = letters.get(0).unwrap();
        result += &first.to_uppercase().to_string();
        if let Some(second) = letters.get(1){
            let mut second_leeters = second.chars();
            if !second_leeters.next().unwrap().is_uppercase(){
                for next_letter in &letters[1..]{
                    let char_check  = next_letter.chars().next().unwrap_or('a');
                    if char_check.is_uppercase(){
                        result+= &char_check.to_string();
                    }
                }
            }
        };
    }

    result
}
