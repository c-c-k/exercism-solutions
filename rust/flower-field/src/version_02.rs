const FLOWER: u8 = b'*';
const SPACE: u8 = b' ';
const ZERO: u8 = b'0';

pub fn annotate(garden: &[&str]) -> Vec<String> {
    garden
        .iter()
        .enumerate()
        .map(|(row, cells)| unsafe {
            String::from_utf8_unchecked(
                cells
                    .as_bytes()
                    .iter()
                    .enumerate()
                    .map(|(column, cell)| {
                        if *cell == FLOWER {
                            FLOWER
                        } else {
                            let mut count = ZERO;
                            for adjacent_cells in
                                garden.iter().take(row + 2).skip(row.saturating_sub(1))
                            {
                                for adjacent_cell in adjacent_cells
                                    .as_bytes()
                                    .iter()
                                    .take(column + 2)
                                    .skip(column.saturating_sub(1))
                                {
                                    if *adjacent_cell == FLOWER {
                                        count += 1
                                    }
                                }
                            }
                            if count > ZERO { count } else { SPACE }
                        }
                    })
                    .collect(),
            )
        })
        .collect()
}
