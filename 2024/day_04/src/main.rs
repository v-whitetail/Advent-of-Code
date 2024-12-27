fn main() {

    let input = std::fs::read_to_string("input.txt").unwrap();

    let cross_word: CrossWord = input.as_str().try_into().unwrap();

    println!("Part 1: {:#?}", cross_word.get_all_xmas().len());
    println!("Part 2: {:#?}", cross_word.get_all_cross_mas().len());

}


#[derive(Debug)]
enum Direction {
    E, NE, N, NW, W, SW, S, SE
}

impl Direction {

    fn next(&self, index: [usize; 2]) -> [usize; 2] {

        match self {
            Self::E  => [index[0]    , index[1] + 1],
            Self::NE => [index[0] - 1, index[1] + 1],
            Self::N  => [index[0] - 1, index[1]    ],
            Self::NW => [index[0] - 1, index[1] - 1],
            Self::W  => [index[0]    , index[1] - 1],
            Self::SW => [index[0] + 1, index[1] - 1],
            Self::S  => [index[0] + 1, index[1]    ],
            Self::SE => [index[0] + 1, index[1] + 1],
        }
    }
}


#[derive(Default, Debug)]
struct CrossWord<'a> {
    rows: usize,
    cols: usize,
    bytes: &'a [u8],
    x_indices: Vec<[usize; 2]>,
    a_indices: Vec<[usize; 2]>,
}

#[derive(Debug)]
struct SingleLineErr;

impl<'a> TryFrom<&'a str> for CrossWord<'a> {

    type Error = SingleLineErr;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        
        let rows = value.trim().lines().count();
        let cols = value.find('\n').ok_or_else(|| SingleLineErr)?;
        let bytes = value.as_bytes();

        let mut x_indices = Vec::new();
        let mut a_indices = Vec::new();

        for (abs_index, byte) in bytes.iter().enumerate() {

            if *byte == b'X' {

                x_indices.push([abs_index / (cols + 1), abs_index % (cols + 1)])

            }

            if *byte == b'A' {

                a_indices.push([abs_index / (cols + 1), abs_index % (cols + 1)])

            }
        }

        Ok(Self{rows, cols, bytes, x_indices, a_indices})
    }

}

impl<'a> CrossWord<'a> {

    fn get(&self, index: [usize; 2]) -> Option<&'a u8> {

        let abs_index = (self.rows + 1)*index[0] + index[1];

        self.bytes.get(abs_index)

    }
    fn xmas_directions(&self, index: [usize;2]) -> [Option<Direction>; 8] {

        let n_margin = index[0] > 2;
        let w_margin = index[1] > 2;
        let s_margin = index[0] + 3 < self.rows;
        let e_margin = index[1] + 3 < self.cols;
        let ne_margin = n_margin && e_margin;
        let nw_margin = n_margin && w_margin;
        let sw_margin = s_margin && w_margin;
        let se_margin = s_margin && e_margin;

        [
            e_margin.then_some(Direction::E),
            ne_margin.then_some(Direction::NE),
            n_margin.then_some(Direction::N),
            nw_margin.then_some(Direction::NW),
            w_margin.then_some(Direction::W),
            sw_margin.then_some(Direction::SW),
            s_margin.then_some(Direction::S),
            se_margin.then_some(Direction::SE),
        ]
    }

    fn has_cross_margin(&self, index: [usize;2]) -> bool {

        let n_margin = index[0] > 0;
        let w_margin = index[1] > 0;
        let s_margin = index[0] + 1 < self.rows;
        let e_margin = index[1] + 1 < self.cols;
        
        return n_margin && w_margin && s_margin && e_margin

    }

    fn get_all_xmas(&self) -> Vec<&[usize; 2]> {

        let mut all_xmas = Vec::new();

        for x_index in &self.x_indices {

            let valid_directions = self.xmas_directions(*x_index);

            for direction in valid_directions.into_iter().filter_map(|dir| dir) {

                let m_index = direction.next(*x_index);
                if self.get(m_index) != Some(&b'M') {
                    continue
                };

                let a_index = direction.next(m_index);
                if self.get(a_index) != Some(&b'A') {
                    continue
                };

                let s_index = direction.next(a_index);
                if self.get(s_index) != Some(&b'S') {
                    continue
                };

                all_xmas.push(x_index);

            }
        }

        return all_xmas

    }

    fn get_all_cross_mas(&self) -> Vec<&[usize; 2]> {

        let mut all_cross_mas = Vec::new();

        for a_index in &self.a_indices {

            if !self.has_cross_margin(*a_index) {
                continue
            };

            let ne = self.get(Direction::NE.next(*a_index));
            let nw = self.get(Direction::NW.next(*a_index));
            let sw = self.get(Direction::SW.next(*a_index));
            let se = self.get(Direction::SE.next(*a_index));

            match (ne, nw, se, sw) {
                (Some(b'M'), Some(b'S'), Some(b'M'), Some(b'S')) => all_cross_mas.push(a_index),
                (Some(b'M'), Some(b'M'), Some(b'S'), Some(b'S')) => all_cross_mas.push(a_index),
                (Some(b'S'), Some(b'S'), Some(b'M'), Some(b'M')) => all_cross_mas.push(a_index),
                (Some(b'S'), Some(b'M'), Some(b'S'), Some(b'M')) => all_cross_mas.push(a_index),
                _ => {},
            };

        }

        return all_cross_mas
    }
}

#[test]
fn test_part_1() {
    let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
    let cross_word: CrossWord = input.try_into().unwrap();
    assert_eq!(cross_word.get_all_xmas().len(), 18);
}

#[test]
fn input_part_1() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let cross_word: CrossWord = input.as_str().try_into().unwrap();
    assert_ne!(cross_word.get_all_xmas().len(), 941);
    // 941 caused by extra whitespace at bottom of file
    // error was fixed by adding .trim() to row count
    assert_eq!(cross_word.get_all_xmas().len(), 2536);
}

#[test]
fn test_part_2() {
    let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
    let cross_word: CrossWord = input.try_into().unwrap();
    dbg!(cross_word.get_all_cross_mas());
    assert_ne!(cross_word.get_all_cross_mas().len(), 5);
    assert_ne!(cross_word.get_all_cross_mas().len(), 25);
    assert_eq!(cross_word.get_all_cross_mas().len(), 9);
}

#[test]
fn input_part_2() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let cross_word: CrossWord = input.as_str().try_into().unwrap();
    assert_eq!(cross_word.get_all_cross_mas().len(), 1875);
}
