pub struct PascalsTriangle {
    rows:Vec<Vec<u32>>
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows:Vec<Vec<u32>> = Vec::new();
        for r in 0..row_count {
            let mut row:Vec<u32> = Vec::new();
            let prev_row = rows.last();
            for c in 0..=r {
                let val = if c == 0 || c == r {
                    1
                }else {
                    prev_row.unwrap()[(c-1)as usize] + prev_row.unwrap()[c as usize]
                };
                row.push(val);
            }
            rows.push(row);
        }
        Self { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}