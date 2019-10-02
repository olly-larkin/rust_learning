pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows = Vec::new();
        let mut i = 0;
        let mut prev_row = Vec::new();
        rows.resize_with(self.row_count as usize, || {
            i += 1;
            let mut j = 0;
            let mut r = Vec::new();
            r.resize_with(i, || {
                j += 1;
                if j == 1 || j == i {
                    1
                } else {
                    prev_row[j - 2] + prev_row[j - 1]
                }
            });
            prev_row = r.clone();
            r
        });
        rows
    }
}
