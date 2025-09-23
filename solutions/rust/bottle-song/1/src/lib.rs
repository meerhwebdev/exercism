pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut poem = String::from("");
    let mut remaining_bottles = start_bottles;
    for _ in 0..take_down {
        let lyric = format!("{} green {} hanging on the wall,
{} green {} hanging on the wall,
And if one green bottle should accidentally fall,
There'll be {} green {} hanging on the wall.

"
        ,num_to_word(remaining_bottles),
        if remaining_bottles == 1 {"bottle"} else {"bottles"}, 
        num_to_word(remaining_bottles), 
        if remaining_bottles == 1 {"bottle"} else {"bottles"}, 
        num_to_word(remaining_bottles-1).to_lowercase(), 
        if remaining_bottles - 1 == 1 {"bottle"} else {"bottles"} 
    );
        remaining_bottles -= 1;
        poem.push_str(&lyric);
    }
    poem
}

fn num_to_word(n: u32) -> &'static str {
    match n {
        0 => "no",
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        10 => "Ten",
        _ => "unknown"
    }
}