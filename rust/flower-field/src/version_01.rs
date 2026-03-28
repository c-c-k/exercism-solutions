const FLOWER: u8 = b'*';
const SPACE: u8 = b' ';
const ZERO: u8 = b'0';

pub fn annotate(garden: &[&str]) -> Vec<String> {
    let height: usize = garden.len();
    let width = garden.first().unwrap_or(&"").len();
    // let mut ret: Vec<Vec<u8>> = vec![vec![b' '; width]; height];
    // let mut ret: Vec<Vec<u8>> = vec![vec![b'0'; width]; height];
    let mut ret: Vec<Vec<u8>> = garden
        .iter()
        .map(|row| {
            row.as_bytes()
                .iter()
                .map(|cell| if *cell == FLOWER { FLOWER } else { ZERO })
                .collect()
        })
        .collect();

    for (y, row) in garden.iter().enumerate() {
        for (x, cell) in row.as_bytes().iter().enumerate() {
            if *cell == FLOWER {
                for i in y.saturating_sub(1)..height.min(y + 2) {
                    for j in x.saturating_sub(1)..width.min(x + 2) {
                        if ret[i][j] != FLOWER {
                            ret[i][j] += 1
                        }
                    }
                }
            }
        }
    }

    ret.iter()
        .map(|bytes| unsafe {
            String::from_utf8_unchecked(
                bytes
                    .iter()
                    .map(|cell| if *cell == ZERO { SPACE } else { *cell })
                    .collect(),
            )
        })
        .collect()
}
