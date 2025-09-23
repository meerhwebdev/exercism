pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack  = String::new();
    let brackets_open = vec!['[', '{', '('];
    let brackets_close = vec![']', '}', ')'];
    for ch in string.chars() {
        if brackets_open.contains(&ch)  {
            stack.push(ch);
        } else if brackets_close.contains(&ch) {
            let index = brackets_close.iter().position(|&element| &element == &ch);
            return match index {
                Some(i) => {
                    if stack.ends_with(brackets_open[i]) {
                        stack.pop();
                        continue;
                    }else {
                        return false
                    }
                },
                None => false
            };
        }
         else {
            continue;
        }
    }
    stack.is_empty()
}