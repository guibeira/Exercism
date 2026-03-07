use std::collections::VecDeque;

static CLOSE_BRACKETS: [char;3] = ['}', ']', ')'];
static OPEN_BRACKETS: [char;3] = ['{','[', '('];

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut queue = VecDeque::new(); 

    for c in string.chars(){
        if CLOSE_BRACKETS.contains(&c) {
            if queue.len() > 0{
                let first = queue.pop_front().unwrap();      
                if first == '{' && c == '}'{
                    continue;
                }
                if first == '[' && c == ']'{
                    continue;
                }
                if first == '(' && c == ')'{
                    continue;
                }
                return false;
            }else{
                return false;
            }
        }else{
            if OPEN_BRACKETS.contains(&c) {
                queue.push_front(c);
            }
        }
    }
    queue.len() == 0
}
