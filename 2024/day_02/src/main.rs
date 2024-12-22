fn main() {

    let input = std::fs::read_to_string("input.txt").unwrap();

    let report_rows = input.parse::<ReportRows>().unwrap();

    println!("Part 1: {}", report_rows.count_safe());
}

#[derive(Default, Debug)]
struct ReportRows {
    rows: Vec<ReportRow>
}

impl ReportRows {

    fn count_safe(&self) -> u32{

        let mut count = 0;

        for row in &self.rows {

            if row.is_safe() {

                count += 1;

            }
        }

        return count
    }
}

#[derive(Default, Debug)]
struct ParseReportRowErr;

impl std::str::FromStr for ReportRows {

    type Err = ParseReportRowErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut report_rows = Self::default();

        for line in s.lines() {

            if line.is_empty() { continue }

            report_rows.rows.push(
                line.parse::<ReportRow>().map_err(|_| ParseReportRowErr)?
            );
        }

        Ok(report_rows)
    }
}

#[derive(Default, Debug)]
struct ReportRow{
    levels: Vec<u32>
}

impl ReportRow {

    fn is_safe(&self) -> bool {

        if self.levels.len() < 2 {
            panic!("ReportRow contained less than 2 elements")
        };

        let is_ascending = self.levels[0] < self.levels[1];

        for i in 1..self.levels.len() {

            let is_in_order = (self.levels[i-1] < self.levels[i]) == is_ascending;
            let is_min_rate = self.levels[i-1].abs_diff(self.levels[i]) >= 1;
            let is_max_rate = self.levels[i-1].abs_diff(self.levels[i]) <= 3;

            if !is_in_order || !is_min_rate || !is_max_rate {
                return false;
            }

        }

        return true;
    }
}

#[derive(Default, Debug)]
struct ParseLevelErr;

impl std::str::FromStr for ReportRow {

    type Err = ParseLevelErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut report_line = Self::default();

        for level in s.split(' ').into_iter() {

            report_line.levels.push(
                level.trim().parse::<u32>().map_err(|_| ParseLevelErr)?
            );
        }

        Ok(report_line)
    }
}

#[test]
fn test_part_1() {
    let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    let report_rows = input.parse::<ReportRows>().unwrap();
    assert_eq!(report_rows.count_safe(), 2);
}

#[test]
fn input_part_1() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let report_rows = input.parse::<ReportRows>().unwrap();
    assert_eq!(report_rows.count_safe(), 432);
}

#[test]
fn test_part_2() {
    assert_eq!(1, 1);
}

#[test]
fn input_part_2() {
    assert_eq!(1, 1);
}
