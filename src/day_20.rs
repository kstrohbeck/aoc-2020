use self::{
    pixels::{Pixels, Rect},
    symmetry::Symmetry,
};

mod index_iter;
mod pixels;
mod symmetry;

pub fn star_1(data: String) {}

pub fn star_2(data: String) {}

/*
use nom::{IResult, branch::alt, bytes::complete::tag, combinator::value, character::complete::{space0, space1, line_ending, multispace0, multispace1}, sequence::terminated, multi::many1};
use crate::utils::u64_;
use std::collections::HashMap;

pub fn star_1(data: String) {
    let tiles = parse(&data);

    let mut edge_map = HashMap::new();
    for tile in &tiles {
        for edge in tile.edges().into_iter() {
            *edge_map.entry(*edge).or_insert(0) += 1;
        }

        let flipped = tile.flipped();
        for edge in flipped.edges().into_iter() {
            *edge_map.entry(*edge).or_insert(0) += 1;
        }
    }

    let mut count = 1;
    for tile in tiles {
        let mut shared_edges = 0;
        for edge in tile.edges().into_iter() {
            if edge_map[edge] > 1 {
                shared_edges += 1;
            }
        }
        if shared_edges < 3 {
            count *= tile.id;
        }
    }

    println!("{}", count);
}

pub fn star_2(data: String) {
    let tiles = parse(&data);
    let mut tile_info = TileInfo::new(tiles);
    let mut rows = Vec::with_capacity(tile_info.side_len);
    let mut left = tile_info.find_top_left_tile();
    for i in 0..tile_info.side_len {
        rows.push(tile_info.find_row_given_first_tile(left.clone()));
        if i < tile_info.side_len - 1 {
            left = tile_info.find_correct_down_tile(&left);
        }
    }
    let mut big_array = tiles_to_big_array(&rows);
    println!("Before: {}", count_waves(&big_array));
    erase_sea_monsters(&mut big_array);
    println!("After: {}", count_waves(&big_array));
}

fn erase_sea_monsters(big_array: &mut [Vec<bool>]) {
    let base = vec![
        str_to_vec("                  # "),
        str_to_vec("#    ##    ##    ###"),
        str_to_vec(" #  #  #  #  #  #   "),
    ];

    let base_a = rot_cw(&base);

    let base_b = flip_horiz(&base);
    let base_c = rot_cw(&base_b);

    let flip = flip(&base);
    let flip_a = rot_cw(&flip);

    let flip_b = flip_horiz(&flip);
    let flip_c = rot_cw(&flip_b);

    erase_pattern(big_array, &base);
    erase_pattern(big_array, &base_a);
    erase_pattern(big_array, &base_b);
    erase_pattern(big_array, &base_c);
    erase_pattern(big_array, &flip);
    erase_pattern(big_array, &flip_a);
    erase_pattern(big_array, &flip_b);
    erase_pattern(big_array, &flip_c);
}

fn str_to_vec(s: &str) -> Vec<bool> {
    s.chars().map(|c| c == '#').collect()
}

fn rot_cw(pattern: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let mut rows = Vec::new();
    for c in 0..pattern[0].len() {
        let mut row = Vec::new();
        for r in 0..pattern.len() {
            row.push(pattern[r][c]);
        }
        rows.push(row);
    }
    rows
}

fn flip(pattern: &[Vec<bool>]) -> Vec<Vec<bool>> {
    pattern.iter().rev().cloned().collect()
}

fn flip_horiz(pattern: &[Vec<bool>]) -> Vec<Vec<bool>> {
    pattern.iter().map(|r| r.iter().rev().copied().collect()).collect()
}

fn erase_pattern(big_array: &mut [Vec<bool>], pattern: &[Vec<bool>]) {
    let h = pattern.len();
    let w = pattern[0].len();
    for r in 0..big_array.len() - h {
        'outer: for c in 0..big_array[r].len() - w {
            for i in 0..h {
                for j in 0..w {
                    if pattern[i][j] && !big_array[r+i][c+j] {
                        continue 'outer;
                    }
                }
            }

            for i in 0..h {
                for j in 0..w {
                    if pattern[i][j] {
                        big_array[r+i][c+j] = false;
                    }
                }
            }
        }
    }
}

fn count_waves(big_array: &[Vec<bool>]) -> usize {
    big_array.iter().map(|r| r.iter().filter(|c| **c).count()).sum()
}

fn tiles_to_big_array(rows: &[Vec<Tile>]) -> Vec<Vec<bool>> {
    let mut big_array = Vec::with_capacity(8 * rows.len());
    for row in rows {
        for i in 1..9 {
            let mut flat_row = Vec::with_capacity(8 * row.len());
            for tile in row {
                for j in 1..9 {
                    flat_row.push(tile.pixels[i][j]);
                }
            }
            big_array.push(flat_row);
        }
    }
    big_array
}

fn pretty_print_big_array(arr: &[Vec<bool>]) {
    for row in arr {
        for c in row {
            if *c {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

#[derive(Debug, Clone)]
struct TileInfo {
    tiles: Vec<Tile>,
    edge_map: HashMap<u64, u64>,
    side_len: usize,
}

impl TileInfo {
    fn new(tiles: Vec<Tile>) -> Self {
        let mut edge_map = HashMap::new();
        for tile in &tiles {
            for edge in tile.edges().into_iter() {
                *edge_map.entry(*edge).or_insert(0) += 1;
            }

            let flipped = tile.flipped();
            for edge in flipped.edges().into_iter() {
                *edge_map.entry(*edge).or_insert(0) += 1;
            }
        }
        let side_len = (tiles.len() as f64).sqrt() as usize;
        Self { tiles, edge_map, side_len }
    }

    fn find_top_left_tile(&mut self) -> Tile {
        let (i, top_left) = self.tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| t.edges().iter().filter(|e| self.edge_map[e] > 1).count() < 3)
            .next()
            .unwrap();
        let top_left = top_left.clone();
        self.tiles.remove(i);

        let [n, e, s, w] = top_left.edges();

        let n_c = self.edge_map[&n];
        let e_c = self.edge_map[&e];
        let s_c = self.edge_map[&s];
        let w_c = self.edge_map[&w];

        let top_left = if n_c > 1 && e_c > 1 {
            top_left.rot_cw()
        } else if e_c > 1 && s_c > 1 {
            top_left.clone()
        } else if s_c > 1 && w_c > 1 {
            top_left.rot_ccw()
        } else {
            top_left.rot_180()
        };

        top_left
    }

    fn find_correct_right_tile(&mut self, tile: &Tile) -> Tile {
        let (i, t) = self.tiles.iter().enumerate().filter_map(|(i, t)| t.align_to_right_of(tile).map(|t| (i, t))).next().unwrap();
        let t = t.clone();
        self.tiles.remove(i);
        t
    }

    fn find_correct_down_tile(&mut self, tile: &Tile) -> Tile {
        let (i, t) = self.tiles.iter().enumerate().filter_map(|(i, t)| t.align_to_bottom_of(tile).map(|t| (i, t))).next().unwrap();
        let t = t.clone();
        self.tiles.remove(i);
        t
    }

    fn find_row_given_first_tile(&mut self, tile: Tile) -> Vec<Tile> {
        let mut tiles = Vec::with_capacity(self.side_len);
        tiles.push(tile);
        for _ in 1..self.side_len {
            let tile = self.find_correct_right_tile(&tiles[tiles.len() - 1]);
            tiles.push(tile);
        }
        tiles
    }
}

fn parse(data: &str) -> Vec<Tile> {
    tiles(data).unwrap().1
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Tile {
    id: u64,
    pixels: [[bool; 10]; 10],
}

impl Tile {
    fn edges(&self) -> [u64; 4] {
        let north = bools_to_u64(self.pixels[0].iter().copied());
        let east = bools_to_u64(self.pixels.iter().map(|r| r[9]));
        let south = bools_to_u64(self.pixels[9].iter().rev().copied());
        let west = bools_to_u64(self.pixels.iter().rev().map(|r| r[0]));
        [north, east, south, west]
    }

    fn flipped(&self) -> Tile {
        Self {
            id: self.id,
            pixels: [
                self.pixels[9],
                self.pixels[8],
                self.pixels[7],
                self.pixels[6],
                self.pixels[5],
                self.pixels[4],
                self.pixels[3],
                self.pixels[2],
                self.pixels[1],
                self.pixels[0],
            ]
        }
    }

    #[rustfmt::skip]
    fn rot_cw(&self) -> Tile {
        let p = &self.pixels;
        Self {
            id: self.id,
            pixels: [
                [p[9][0], p[8][0], p[7][0], p[6][0], p[5][0], p[4][0], p[3][0], p[2][0], p[1][0], p[0][0]],
                [p[9][1], p[8][1], p[7][1], p[6][1], p[5][1], p[4][1], p[3][1], p[2][1], p[1][1], p[0][1]],
                [p[9][2], p[8][2], p[7][2], p[6][2], p[5][2], p[4][2], p[3][2], p[2][2], p[1][2], p[0][2]],
                [p[9][3], p[8][3], p[7][3], p[6][3], p[5][3], p[4][3], p[3][3], p[2][3], p[1][3], p[0][3]],
                [p[9][4], p[8][4], p[7][4], p[6][4], p[5][4], p[4][4], p[3][4], p[2][4], p[1][4], p[0][4]],
                [p[9][5], p[8][5], p[7][5], p[6][5], p[5][5], p[4][5], p[3][5], p[2][5], p[1][5], p[0][5]],
                [p[9][6], p[8][6], p[7][6], p[6][6], p[5][6], p[4][6], p[3][6], p[2][6], p[1][6], p[0][6]],
                [p[9][7], p[8][7], p[7][7], p[6][7], p[5][7], p[4][7], p[3][7], p[2][7], p[1][7], p[0][7]],
                [p[9][8], p[8][8], p[7][8], p[6][8], p[5][8], p[4][8], p[3][8], p[2][8], p[1][8], p[0][8]],
                [p[9][9], p[8][9], p[7][9], p[6][9], p[5][9], p[4][9], p[3][9], p[2][9], p[1][9], p[0][9]],
            ]
        }
    }

    #[rustfmt::skip]
    fn rot_ccw(&self) -> Tile {
        let p = &self.pixels;
        Self {
            id: self.id,
            pixels: [
                [p[0][9], p[1][9], p[2][9], p[3][9], p[4][9], p[5][9], p[6][9], p[7][9], p[8][9], p[9][9]],
                [p[0][8], p[1][8], p[2][8], p[3][8], p[4][8], p[5][8], p[6][8], p[7][8], p[8][8], p[9][8]],
                [p[0][7], p[1][7], p[2][7], p[3][7], p[4][7], p[5][7], p[6][7], p[7][7], p[8][7], p[9][7]],
                [p[0][6], p[1][6], p[2][6], p[3][6], p[4][6], p[5][6], p[6][6], p[7][6], p[8][6], p[9][6]],
                [p[0][5], p[1][5], p[2][5], p[3][5], p[4][5], p[5][5], p[6][5], p[7][5], p[8][5], p[9][5]],
                [p[0][4], p[1][4], p[2][4], p[3][4], p[4][4], p[5][4], p[6][4], p[7][4], p[8][4], p[9][4]],
                [p[0][3], p[1][3], p[2][3], p[3][3], p[4][3], p[5][3], p[6][3], p[7][3], p[8][3], p[9][3]],
                [p[0][2], p[1][2], p[2][2], p[3][2], p[4][2], p[5][2], p[6][2], p[7][2], p[8][2], p[9][2]],
                [p[0][1], p[1][1], p[2][1], p[3][1], p[4][1], p[5][1], p[6][1], p[7][1], p[8][1], p[9][1]],
                [p[0][0], p[1][0], p[2][0], p[3][0], p[4][0], p[5][0], p[6][0], p[7][0], p[8][0], p[9][0]],
            ]
        }
    }

    fn rot_180(&self) -> Tile {
        self.rot_ccw().rot_ccw()
    }

    fn align_to_right_of(&self, to_left: &Self) -> Option<Self> {
        let right_edge = to_left.edges()[1];
        self.with_left_edge_matching(right_edge)
    }

    fn with_left_edge_matching(&self, edge: u64) -> Option<Self> {
        fn matching_idxs(tile: &Tile, edge: u64) -> Option<usize> {
            tile.edges().iter().enumerate().filter(|(_, e)| do_edges_match(edge, **e)).map(|(i, _)| i).next()
        }

        fn do_edges_match(a: u64, mut b: u64) -> bool {
            let mut rev = 0;
            for _ in 0..10 {
                rev <<= 1;
                if b & 1 > 0 {
                    rev += 1;
                }
                b >>= 1;
            }
            a == rev
        }

        if let Some(idx) = matching_idxs(self, edge) {
            let tile = match idx {
                0 => self.rot_ccw(),  // north
                1 => self.rot_180(),  // east
                2 => self.rot_cw(),  // south
                3 => self.clone(),  // west
                _ => return None,
            };
            return Some(tile)
        }

        let flipped = self.flipped();
        if let Some(idx) = matching_idxs(&flipped, edge) {
            let tile = match idx {
                0 => flipped.rot_ccw(),  // north
                1 => flipped.rot_180(),  // east
                2 => flipped.rot_cw(),  // south
                3 => flipped.clone(),  // west
                _ => return None,
            };
            return Some(tile)
        }

        None
    }

    fn align_to_bottom_of(&self, to_top: &Self) -> Option<Self> {
        let bottom_edge = to_top.edges()[2];
        self.with_top_edge_matching(bottom_edge)
    }

    fn with_top_edge_matching(&self, edge: u64) -> Option<Self> {
        fn matching_idxs(tile: &Tile, edge: u64) -> Option<usize> {
            tile.edges().iter().enumerate().filter(|(_, e)| do_edges_match(edge, **e)).map(|(i, _)| i).next()
        }

        fn do_edges_match(a: u64, mut b: u64) -> bool {
            let mut rev = 0;
            for _ in 0..10 {
                rev <<= 1;
                if b & 1 > 0 {
                    rev += 1;
                }
                b >>= 1;
            }
            a == rev
        }

        if let Some(idx) = matching_idxs(self, edge) {
            let tile = match idx {
                0 => self.clone(),  // north
                1 => self.rot_ccw(),  // east
                2 => self.rot_180(),  //south
                3 => self.rot_cw(),  // west
                _ => return None,
            };
            return Some(tile)
        }

        let flipped = self.flipped();
        if let Some(idx) = matching_idxs(&flipped, edge) {
            let tile = match idx {
                0 => flipped.clone(),  // north
                1 => flipped.rot_ccw(),  // east
                2 => flipped.rot_180(),  //south
                3 => flipped.rot_cw(),  // west
                _ => return None,
            };
            return Some(tile)
        }

        None
    }

    fn pretty_print(&self) {
        println!("Tile {}:", self.id);
        for line in &self.pixels {
            for c in line {
                if *c {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }
}

fn bools_to_u64<I>(i: I) -> u64
where
    I: Iterator<Item = bool>
{
    let mut num = 0;
    for b in i {
        num <<= 1;
        if b {
            num += 1;
        }
    }
    num
}

fn tiles(input: &str) -> IResult<&str, Vec<Tile>> {
    many1(tile)(input)
}

fn tile(input: &str) -> IResult<&str, Tile> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("Tile")(input)?;
    let (input, _) = space1(input)?;
    let (input, id) = u64_(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = line_ending(input)?;
    let (input, a) = tile_row(input)?;
    let (input, b) = tile_row(input)?;
    let (input, c) = tile_row(input)?;
    let (input, d) = tile_row(input)?;
    let (input, e) = tile_row(input)?;
    let (input, f) = tile_row(input)?;
    let (input, g) = tile_row(input)?;
    let (input, h) = tile_row(input)?;
    let (input, i) = tile_row(input)?;
    let (input, j) = tile_row(input)?;
    let pixels = [a, b, c, d, e, f, g, h, i, j];
    let (input, _) = multispace0(input)?;
    Ok((input, Tile { id, pixels }))
}

fn tile_row(input: &str) -> IResult<&str, [bool; 10]> {
    let (input, a) = pixel(input)?;
    let (input, b) = pixel(input)?;
    let (input, c) = pixel(input)?;
    let (input, d) = pixel(input)?;
    let (input, e) = pixel(input)?;
    let (input, f) = pixel(input)?;
    let (input, g) = pixel(input)?;
    let (input, h) = pixel(input)?;
    let (input, i) = pixel(input)?;
    let (input, j) = pixel(input)?;
    let row = [a, b, c, d, e, f, g, h, i, j];
    let (input, _) = line_ending(input)?;
    Ok((input, row))
}

fn pixel(input: &str) -> IResult<&str, bool> {
    alt((
        value(true, tag("#")),
        value(false, tag(".")),
    ))(input)
}

#[cfg(tests)]
mod test {
    use super::bools_to_u64;

    #[test]
    fn bools_to_u64_is_correct() {
        let bools = [false, false, true, true, false, true, false, false, true, false];
        assert_eq!(bools_to_u64(bools.iter().copied()), 210);
    }
}

*/
