use crate::challenge::DailyChallenge;
use std::fs;
use std::path::Path;

#[derive(PartialEq, Clone, Copy)]
enum Pixel {
    Empty,
    Galaxy,
}

impl TryFrom<char> for Pixel {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Pixel::Empty),
            '#' => Ok(Pixel::Galaxy),
            _ => Err("Invalid pixel"),
        }
    }
}

impl From<Pixel> for char {
    fn from(value: Pixel) -> Self {
        match value {
            Pixel::Empty => '.',
            Pixel::Galaxy => '#',
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn distance(a: &Coord, b: &Coord) -> usize {
        a.row.abs_diff(b.row) + a.col.abs_diff(b.col)
    }
}

struct Universe {
    width: usize,
    height: usize,
    galaxies: Vec<Coord>,
}

impl Universe {
    fn new(pixels: &Vec<Vec<Pixel>>) -> Self {
        let height = pixels.len();
        let width = pixels.first().map(Vec::len).unwrap_or(0);

        let mut galaxies: Vec<Coord> = Vec::new();
        for (r, pixel_row) in pixels.iter().enumerate() {
            for (c, &pixel) in pixel_row.iter().enumerate() {
                if pixel == Pixel::Galaxy {
                    galaxies.push(Coord::new(r, c));
                }
            }
        }

        Self {
            width,
            height,
            galaxies,
        }
    }

    fn expand(&self, factor: usize) -> Universe {
        let galaxy_rows: Vec<usize> = self.galaxies.iter().map(|c| c.row).collect();
        let galaxy_cols: Vec<usize> = self.galaxies.iter().map(|c| c.col).collect();

        let is_empty_row = |r: usize| -> bool { (r < self.height) && (!galaxy_rows.contains(&r)) };

        let is_empty_col = |c: usize| -> bool { (c < self.width) && (!galaxy_cols.contains(&c)) };

        let mut new_galaxies = Vec::new();

        let mut empty_rows = 0;
        let mut empty_cols = 0;
        for row in 0..self.height {
            if is_empty_row(row) {
                empty_rows += 1;
                continue;
            }

            empty_cols = 0;
            for col in 0..self.width {
                if is_empty_col(col) {
                    empty_cols += 1;
                    continue;
                }

                if self.galaxies.contains(&Coord::new(row, col)) {
                    let row_offset = empty_rows * (factor - 1);
                    let col_offset = empty_cols * (factor - 1);
                    new_galaxies.push(Coord::new(row + row_offset, col + col_offset))
                }
            }
        }

        Universe {
            width: self.width + empty_cols * (factor - 1),
            height: self.height + empty_rows * (factor - 1),
            galaxies: new_galaxies,
        }
    }

    fn from_file(file: &Path) -> Result<Self, String> {
        let contents = fs::read_to_string(file).map_err(|e| e.to_string())?;

        let mut pixels: Vec<Vec<Pixel>> = Vec::new();

        for line in contents.lines() {
            let mut row: Vec<Pixel> = Vec::new();

            for c in line.chars() {
                let tile = Pixel::try_from(c)?;
                row.push(tile);
            }
            pixels.push(row);
        }

        Ok(Universe::new(&pixels))
    }

    // #[allow(dead_code)]
    // fn print_masked(&self, mask: impl Fn(Coord) -> bool) {
    //     for (row, pixel_row) in self.pixels.iter().enumerate() {
    //         for (col, pixel) in pixel_row.iter().enumerate() {
    //             let coord = Coord::new(row, col);
    //             let c = if mask(coord) {
    //                 (*pixel).try_into().unwrap()
    //             } else {
    //                 '_'
    //             };
    //             print!("{c}");
    //         }
    //         print!("\n");
    //     }
    //     print!("\n");
    // }
}

fn galaxy_distance_sum(universe: &Universe, expand_factor: usize) -> u64 {
    let expanded = universe.expand(expand_factor);
    let mut distance_sum = 0;
    for (i, galaxy1) in expanded.galaxies.iter().enumerate() {
        for galaxy2 in expanded.galaxies[(i + 1)..].iter() {
            distance_sum += Coord::distance(galaxy1, galaxy2) as u64;
        }
    }

    distance_sum
}

#[derive(Default)]
pub struct Day11;

impl DailyChallenge for Day11 {
    fn part1(&self, file: &Path) -> u64 {
        let universe = Universe::from_file(file).unwrap();
        galaxy_distance_sum(&universe, 2)
    }

    fn part2(&self, file: &Path) -> u64 {
        let universe = Universe::from_file(file).unwrap();
        galaxy_distance_sum(&universe, 1000000)
    }
}

#[test]
fn test_from_file() {
    let universe = Universe::from_file(Path::new("data/11.sample")).expect("Test file missing");
    assert_eq!(universe.width, 10);
    assert_eq!(universe.height, 10);
    assert_eq!(
        universe.galaxies,
        vec![
            Coord::new(0, 3),
            Coord::new(1, 7),
            Coord::new(2, 0),
            Coord::new(4, 6),
            Coord::new(5, 1),
            Coord::new(6, 9),
            Coord::new(8, 7),
            Coord::new(9, 0),
            Coord::new(9, 4)
        ]
    );
}

#[test]
fn test_expand() {
    let universe = Universe::from_file(Path::new("data/11.sample")).expect("Test file missing");
    let expanded = universe.expand(2);

    assert_eq!(expanded.width, 13);
    assert_eq!(expanded.height, 12);
}

#[test]
fn test_part1() {
    assert_eq!(Day11.part1(Path::new("data/11.sample")), 374)
}

#[test]
fn test_part2() {
    let universe = Universe::from_file(Path::new("data/11.sample")).expect("Test file missing");
    assert_eq!(galaxy_distance_sum(&universe, 10), 1030);
    assert_eq!(galaxy_distance_sum(&universe, 100), 8410);
}

// #[test]
// fn test_part2_larger() {
//     assert_eq!(Day11.part2(Path::new("data/11.sample3")), 11)
// }
