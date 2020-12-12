use std::convert::TryFrom;

pub fn star_1(data: String) {
    let mut old = parse(&data);
    let mut new = old.clone();
    loop {
        step_1(&old, &mut new);
        if old == new {
            break;
        }
        std::mem::swap(&mut old, &mut new);
    }
    let occupied = new
        .iter()
        .map(|r| {
            r.iter()
                .map(|c| if c.is_occupied() { 1 } else { 0 })
                .sum::<u32>()
        })
        .sum::<u32>();
    println!("{}", occupied);
}

pub fn star_2(data: String) {
    let mut old = parse(&data);
    let mut new = old.clone();
    loop {
        step_2(&old, &mut new);
        if old == new {
            break;
        }
        std::mem::swap(&mut old, &mut new);
    }
    let occupied = new
        .iter()
        .map(|r| {
            r.iter()
                .map(|c| if c.is_occupied() { 1 } else { 0 })
                .sum::<u32>()
        })
        .sum::<u32>();
    println!("{}", occupied);
}

fn step_1(old: &Vec<Vec<Cell>>, new: &mut Vec<Vec<Cell>>) {
    for row in 0..old.len() {
        for col in 0..old[row].len() {
            if old[row][col] == Cell::Floor {
                new[row][col] = Cell::Floor;
                continue;
            }

            let mut neighbor_sum = 0;

            if row > 0 {
                if col > 0 {
                    neighbor_sum += if old[row - 1][col - 1].is_occupied() {
                        1
                    } else {
                        0
                    };
                }
                neighbor_sum += if old[row - 1][col].is_occupied() {
                    1
                } else {
                    0
                };
                if col < old[row].len() - 1 {
                    neighbor_sum += if old[row - 1][col + 1].is_occupied() {
                        1
                    } else {
                        0
                    };
                }
            }
            if col > 0 {
                neighbor_sum += if old[row][col - 1].is_occupied() {
                    1
                } else {
                    0
                };
            }
            if col < old[row].len() - 1 {
                neighbor_sum += if old[row][col + 1].is_occupied() {
                    1
                } else {
                    0
                };
            }
            if row < old.len() - 1 {
                if col > 0 {
                    neighbor_sum += if old[row + 1][col - 1].is_occupied() {
                        1
                    } else {
                        0
                    };
                }
                neighbor_sum += if old[row + 1][col].is_occupied() {
                    1
                } else {
                    0
                };
                if col < old[row].len() - 1 {
                    neighbor_sum += if old[row + 1][col + 1].is_occupied() {
                        1
                    } else {
                        0
                    };
                }
            }

            if old[row][col] == Cell::Empty && neighbor_sum == 0 {
                new[row][col] = Cell::Occupied;
            } else if old[row][col] == Cell::Occupied && neighbor_sum >= 4 {
                new[row][col] = Cell::Empty;
            } else {
                new[row][col] = old[row][col];
            }
        }
    }
}

fn step_2(old: &Vec<Vec<Cell>>, new: &mut Vec<Vec<Cell>>) {
    for row in 0..old.len() {
        for col in 0..old[row].len() {
            if old[row][col] == Cell::Floor {
                new[row][col] = Cell::Floor;
                continue;
            }

            let ul = raycast(&old, row, col, Direction::Prev, Direction::Prev);
            let u = raycast(&old, row, col, Direction::Prev, Direction::Same);
            let ur = raycast(&old, row, col, Direction::Prev, Direction::Next);
            let l = raycast(&old, row, col, Direction::Same, Direction::Prev);
            let r = raycast(&old, row, col, Direction::Same, Direction::Next);
            let dl = raycast(&old, row, col, Direction::Next, Direction::Prev);
            let d = raycast(&old, row, col, Direction::Next, Direction::Same);
            let dr = raycast(&old, row, col, Direction::Next, Direction::Next);
            let neighbor_sum = [ul, u, ur, l, r, dl, d, dr]
                .iter()
                .map(|o| if *o { 1 } else { 0 })
                .sum::<u32>();

            if old[row][col] == Cell::Empty && neighbor_sum == 0 {
                new[row][col] = Cell::Occupied;
            } else if old[row][col] == Cell::Occupied && neighbor_sum >= 5 {
                new[row][col] = Cell::Empty;
            } else {
                new[row][col] = old[row][col];
            }
        }
    }
}

fn raycast(
    cells: &Vec<Vec<Cell>>,
    row: usize,
    col: usize,
    row_dir: Direction,
    col_dir: Direction,
) -> bool {
    let mut r = row;
    let mut c = col;
    loop {
        if (r == 0 && row_dir == Direction::Prev)
            || (r == cells.len() - 1 && row_dir == Direction::Next)
            || (c == 0 && col_dir == Direction::Prev)
            || (c == cells[r].len() - 1 && col_dir == Direction::Next)
        {
            return false;
        }
        r = match row_dir {
            Direction::Prev => r - 1,
            Direction::Same => r,
            Direction::Next => r + 1,
        };
        c = match col_dir {
            Direction::Prev => c - 1,
            Direction::Same => c,
            Direction::Next => c + 1,
        };
        match cells[r][c] {
            Cell::Floor => {}
            Cell::Empty => return false,
            Cell::Occupied => return true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Prev,
    Same,
    Next,
}

fn parse(data: &str) -> Vec<Vec<Cell>> {
    data.lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(parse_row)
        .collect()
}

fn parse_row(row: &str) -> Vec<Cell> {
    row.chars()
        .map(Cell::try_from)
        .map(Result::unwrap)
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Floor,
    Empty,
    Occupied,
}

impl Cell {
    fn is_occupied(&self) -> bool {
        *self == Cell::Occupied
    }
}

impl TryFrom<char> for Cell {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Self::Floor),
            'L' => Ok(Self::Empty),
            '#' => Ok(Self::Occupied),
            _ => Err(()),
        }
    }
}
