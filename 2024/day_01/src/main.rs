fn main() {

    let input = std::fs::read_to_string("src/input.txt").unwrap();

    let mut list = input.parse::<List>().unwrap();
    list.sort();

    println!("Part 1: {}", list.total());
    println!("Part 2: {}", list.similarity());
}

#[derive(Default, Debug)]
struct List{
    left: Vec<u32>,
    right: Vec<u32>,
}

impl List {
    fn sort(&mut self) {
        self.left.sort();
        self.right.sort();
    }
    fn total(&self) -> u32 {
        self.left.iter().zip(self.right.iter())
            .fold(0, |total, (left, right)| total + left.abs_diff(*right))
    }
    fn similarity(&self) -> u32 {
        self.left.iter()
            .fold(
                0,
                |total, left| total + left * scan_right_list(left, &self.right)
                )
    }
}

#[derive(Default, Debug)]
struct ParseError;

impl std::str::FromStr for List {

    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut list = Self::default();

        for line in s.lines() {

            if 1 < line.len() {

                let (lhs, rhs) = line.split_at(line.find(" ").unwrap() + 1);

                list.left.push(lhs.trim().parse::<u32>().map_err(|_| ParseError)?);
                list.right.push(rhs.trim().parse::<u32>().map_err(|_| ParseError)?);
            }
        }
        Ok(list)
    }
}

fn scan_right_list(left: &u32, right: &Vec<u32>) -> u32 {

    if let Ok(start_index) = right.binary_search(left) {

        let mut lower_bound = start_index;
        let mut upper_bound = start_index;

        while 0 < lower_bound && right[lower_bound - 1] == right[start_index] {
            lower_bound -= 1;
        };
        while upper_bound < right.len() && right[upper_bound + 1] == right[start_index] {
            upper_bound += 1;
        };

        return (upper_bound - lower_bound + 1) as u32;

    } else {
        return 0;
    }
}

#[test]
fn test_part_1() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3
";
    let mut list = input.parse::<List>().unwrap();
    list.sort();
    assert_eq!(list.total(), 11);
}

#[test]
fn input_part_1() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let mut list = input.parse::<List>().unwrap();
    list.sort();
    assert_eq!(list.total(), 1603498);
}

#[test]
fn test_part_2() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3
";
    let mut list = input.parse::<List>().unwrap();
    list.sort();
    assert_eq!(list.similarity(), 31);
}

#[test]
fn input_part_2() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let mut list = input.parse::<List>().unwrap();
    list.sort();
    assert_eq!(list.similarity(), 25574739);
}
