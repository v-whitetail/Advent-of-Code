fn main() {

    let input = std::fs::read_to_string("input.txt").unwrap();

    let memory = input.parse::<Memory>().unwrap();

    println!("Part 1: {:#?}", memory.evaluate_all());
    println!("Part 2: {:#?}", memory.evaluate());

}

#[derive(Default, Debug, Clone)]
struct Memory {
    ops: Vec<(bool, Mul)>,
}

impl Memory {

    fn evaluate(&self) -> u32{

        self.ops.iter()
            .filter(|(flag, _)| *flag)
            .map(|(_, op)| op.evaluate())
            .sum::<u32>()

    }

    fn evaluate_all(&self) -> u32{

        self.ops.iter()
            .map(|(_, op)| op.evaluate())
            .sum::<u32>()

    }
}

#[derive(Debug, Copy, Clone)]
struct MemoryParseErr;

impl std::str::FromStr for Memory {

    type Err = MemoryParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut memory = Self::default();

        let mut do_flag = true;
        
        for i in 0..s.len()-8 {

            if s[i..i+7] == *"don't()" { 
                do_flag = false;
            }
            if s[i..i+4] == *"do()" { 
                do_flag = true;
            }

            if let Ok(mul) = s[i..].parse::<Mul>() {

                memory.ops.push((do_flag, mul));

            }
        }

        if memory.ops.is_empty() {

            Err(MemoryParseErr)

        } else {

            Ok(memory)

        }
    }
}


#[derive(Default, Debug, Copy, Clone)]
struct Mul {
    lhs: u32,
    rhs: u32,
}

impl Mul {

    fn evaluate(&self) -> u32 {

        return self.lhs * self.rhs

    }
}

#[derive(Debug, Copy, Clone)]
enum MulParseErr{
    EOL,
    HEAD,
    COMMA,
    CLOSE,
    INT,
}

impl std::str::FromStr for Mul {

    type Err = MulParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        if s.len() < 8 {
            return Err(MulParseErr::EOL)
        };

        if s[0..4] != *"mul(" {
            return Err(MulParseErr::HEAD)
        };

        let close = s.find(')').ok_or(MulParseErr::CLOSE)?;
        let comma = s[4..close].find(',').ok_or(MulParseErr::COMMA)? + 4;

        let lhs = s[4..comma].parse::<u32>().map_err(|_| MulParseErr::INT)?;
        let rhs = s[comma+1..close].parse::<u32>().map_err(|_| MulParseErr::INT)?;

        Ok( Mul{lhs, rhs} )
        
    }
}

#[test]
fn test_part_1() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let memory = input.parse::<Memory>().expect("empty memory region");
    assert_eq!(memory.evaluate_all(), 161);
}

#[test]
fn input_part_1() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let memory = input.parse::<Memory>().expect("empty memory region");
    assert_eq!(memory.evaluate_all(), 184576302);
}

#[test]
fn test_part_2() {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let memory = input.parse::<Memory>().expect("empty memory region");
    assert_eq!(memory.evaluate(), 48);
}

#[test]
fn input_part_2() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let memory = input.parse::<Memory>().expect("empty memory region");
    assert_eq!(memory.evaluate(), 118173507);
}
