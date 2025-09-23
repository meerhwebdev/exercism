pub fn annotate(garden: &[&str]) -> Vec<String> {
    let rows = garden.len();
    let cols = if rows != 0 {
        garden[0].len()
    } else {
        0
    };
    let mut output = vec![vec!['0'; cols]; rows];
    for r in 0..rows {
        let row = garden[r].as_bytes();
        for c in 0..cols {
            if row[c] == b'*' {
                output[r][c] = '*';
                for dr in [-1i32, 0, 1] {
                    for dc in [-1i32, 0, 1] {
                        if dr == 0 && dc == 0 { continue; }
                        let nr = r as i32 + dr;
                        let nc = c as i32 + dc;
                        if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                            let (nr, nc) = (nr as usize, nc as usize);
                            if output[nr][nc] != '*' {
                                 output[nr][nc] = (output[nr][nc] as u8  + 1) as char
                            }
                        }
                    }
                }
            }
        }

    }
    // map the output
    output.into_iter().map(|row| row.into_iter().map(|c| if c == '0' { ' '} else { c } ).collect()).collect()
}