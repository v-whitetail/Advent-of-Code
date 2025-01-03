extern crate tinyvec;

fn main() {

    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut manual_revision = input.parse::<ManualRevision>().unwrap();

    println!("Part 1: {:#?}", manual_revision.check_updates());
    println!("Part 2: {:#?}", manual_revision.fix_updates());
}

const MAX_PAGES_IN_UPDATE: usize = 24;

#[derive(Default, Debug)]
struct ManualRevision {
    rules: Vec<Rule>,
    updates: Vec<Update>
}

impl ManualRevision {

    fn check_updates(&self) -> usize {

        let mut update_total: usize = 0;

        'updates: for update in &self.updates {

            '_rules: for rule in &self.rules {

                if let UpdateCheck::Fail = update.check_rule(&rule) {

                    continue 'updates;
                }

            }

            update_total += update.pages[update.pages.len() / 2];
        }

        return update_total

    }

    fn fix_updates(&mut self) -> usize {

        let mut update_total: usize = 0;

        '_updates: for update in self.updates.iter_mut() {

            let mut is_fixed = false;
            let mut pass_count: usize = 0;

            'rules: for rule in self.rules.iter().cycle() {

                if let UpdateCheck::Fixed = update.fix_rule(&rule) {

                    is_fixed = true;
                    pass_count = 0;

                } else {

                    pass_count += 1;
                }

                if self.rules.len() < pass_count {

                    break 'rules
                }
            }

            if is_fixed {

                update_total += update.pages[update.pages.len() / 2];
            }
        }

        return update_total
    }
}

#[derive(Debug)]
enum ManualRevisionParseErr {
    NoSplit,
    _RuleParseErr,
    _UpdateParseErr,
}

impl std::str::FromStr for ManualRevision {

    type Err = ManualRevisionParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let (rules_str, updates_str) = s.split_once("\n\n").ok_or(ManualRevisionParseErr::NoSplit)?;

        let rules = rules_str.lines()
            .filter_map(|line| line.parse::<Rule>().ok())
            .collect::<Vec<_>>();

        let updates = updates_str.lines()
            .filter_map(|line| line.parse::<Update>().ok())
            .collect::<Vec<_>>();

        Ok(Self{rules, updates})
    }
}

#[derive(Default, Debug)]
struct Rule {
    lower: usize,
    upper: usize,
}

#[derive(Debug)]
enum RuleParseErr {
    NoSplit,
    ParseInt,
}

impl From<std::num::ParseIntError> for RuleParseErr {
    fn from(_value: std::num::ParseIntError) -> Self {
        Self::ParseInt
    }
}

impl std::str::FromStr for Rule {

    type Err = RuleParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let (lower_str, upper_str) = s.split_once('|').ok_or(RuleParseErr::NoSplit)?;
        let lower = lower_str.parse::<usize>()?;
        let upper = upper_str.parse::<usize>()?;

        Ok(Self{lower, upper})
        
    }
}

#[derive(Default, Debug)]
struct Update {
    pages: tinyvec::ArrayVec<[usize; MAX_PAGES_IN_UPDATE]>
}

#[derive(Debug)]
enum UpdateCheck {
    Ignore,
    Pass,
    Fail,
    Fixed,
}

impl Update {
    
    fn check_rule(&self, rule: &Rule) -> UpdateCheck {

        let mut lower: Option<usize> = None;
        let mut upper: Option<usize> = None;

        for (i, page) in self.pages.iter().enumerate() {

            if rule.lower == *page {
                lower = Some(i);
            }
            if rule.upper == *page {
                upper = Some(i);
            }
        }

        match (lower, upper) {
            (None, _) => UpdateCheck::Ignore,
            (_, None) => UpdateCheck::Ignore,
            (Some(lhs), Some(rhs)) if lhs < rhs => UpdateCheck::Pass,
            _ => UpdateCheck::Fail
        }
    }

    fn fix_rule(&mut self, rule: &Rule) -> UpdateCheck {

        let mut lower: Option<usize> = None;
        let mut upper: Option<usize> = None;

        for (i, page) in self.pages.iter().enumerate() {

            if rule.lower == *page {
                lower = Some(i);
            }
            if rule.upper == *page {
                upper = Some(i);
            }
        }

        match (lower, upper) {
            (None, _) => UpdateCheck::Ignore,
            (_, None) => UpdateCheck::Ignore,
            (Some(lhs), Some(rhs)) if lhs < rhs => UpdateCheck::Pass,
            (Some(lhs), Some(rhs)) if lhs > rhs => {
                self.pages.swap(lhs, rhs);
                UpdateCheck::Fixed
            },
            _ => UpdateCheck::Ignore
        }

    }
}

#[derive(Debug)]
enum UpdateParseErr {
    NoComma,
    ParseInt,
}

impl From<std::num::ParseIntError> for UpdateParseErr {
    fn from(_value: std::num::ParseIntError) -> Self {
        Self::ParseInt
    }
}

impl std::str::FromStr for Update {

    type Err = UpdateParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut pages = tinyvec::array_vec!([usize; MAX_PAGES_IN_UPDATE]);

        if !s.contains(',') {
            return Err(UpdateParseErr::NoComma)
        };

        for page in s.split(',') {
            pages.push(page.parse::<usize>()?)
        }

        Ok(Self{pages})
        
    }
}

#[test]
fn test_part_1() {
    let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    let manual_revision = input.parse::<ManualRevision>().unwrap();
    assert_eq!(manual_revision.check_updates(), 143);
}

#[test]
fn input_part_1() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let manual_revision = input.parse::<ManualRevision>().unwrap();
    assert_eq!(manual_revision.check_updates(), 5509);
}

#[test]
fn test_part_2() {
    let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    let mut manual_revision = input.parse::<ManualRevision>().unwrap();
    let score = manual_revision.fix_updates();
    assert_eq!(score, 123);
}

#[test]
fn input_part_2() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut manual_revision = input.parse::<ManualRevision>().unwrap();
    let score = manual_revision.fix_updates();
    assert_ne!(score, 3607);
    assert_eq!(score, 4407);
}
