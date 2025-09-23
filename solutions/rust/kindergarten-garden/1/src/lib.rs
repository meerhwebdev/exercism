fn child_index(child_name:&str)-> usize {
    let list = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph", "Kincaid", "Larry"
    ];
    list.iter().position(|child| child == &child_name).unwrap()
}

fn decode_plant_list(plant_code: &str)-> Vec<&'static str> {
    let mut list  = vec![];
    for code in plant_code.chars() {
        match code {
            'G'=> list.push("grass"),
            'C'=> list.push("clover"),
            'R'=> list.push("radishes"),
            'V'=> list.push("violets"),
            _ => panic!("Unknown plant type!")
        }
    }
    list
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let mut planted= vec![];
    let planted_lines:Vec<&str> = diagram.lines().collect();
    let index= child_index(student);
    let child_index = if index !=0 {
        index + index
    }else {
        0
    };
    for line in planted_lines {
        dbg!(line.trim(), &child_index);
        planted.extend(decode_plant_list(&line.trim()[child_index..child_index + 2]));
    }
    planted
}