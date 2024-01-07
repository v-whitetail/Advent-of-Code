pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;

pub enum Input {
    Real(u8),
    Test(u8),
}
impl Input {
    pub fn new(file_expansion: &str) -> Self {
        let day = file_expansion
            .split('_')
            .last()
            .unwrap()
            .split('.')
            .nth(0)
            .unwrap()
            .parse::<u8>()
            .unwrap();
        Self::Real(day)
    }
    pub fn test(self) -> Self {
        match self {
            Self::Real(day) => Self::Test(day),
            Self::Test(day) => Self::Test(day),
        }
    }
    pub fn dir(&self) -> &str {
        match self {
            Self::Real(day) => "inputs",
            Self::Test(day) => "tests",
        }
    }
    pub fn handle(&self) -> String {
        match self {
            Self::Real(day) => format!("day_{:?}.nu", day),
            Self::Test(day) => format!("day_{:?}.nu", day),
        }
    }
    pub fn fetch(&self, path: &std::path::PathBuf) {
        let key = crate::personal_info::SESSION_KEY;
        match self {
            Self::Real(day) => {
                println!("Fetching input for Day {day}");
                let url = format!("https://adventofcode.com/2023/day/{:#?}/input",day);
                let mut curl = std::process::Command::new("curl");
                curl.arg("-s");
                curl.arg("-H");
                curl.arg(key);
                curl.arg(url);
                curl.arg("-o");
                curl.arg(path);
                curl.spawn();
                println!("Saving to: {path:?}");
                std::thread::sleep(std::time::Duration::from_secs(1));
            },
            Self::Test(day) => {
                println!("Fetching test input for Day {day}");
                let url = format!("https://adventofcode.com/2023/day/{:?}",day);
                let mut curl = std::process::Command::new("curl");
                curl.arg("-s");
                curl.arg("-H");
                curl.arg(key);
                curl.arg(url);
                curl.arg("-o");
                curl.arg(path);
                curl.spawn();
                println!("Saving to: {path:?}");
                std::thread::sleep(std::time::Duration::from_secs(1));
                let test = std::fs::read_to_string(path).unwrap();
                let (_, test) = test.split_once("<p>For example:</p>").unwrap();
                let (_, test) = test.trim().split_once("<pre><code>").unwrap();
                let (test, _) = test.trim().split_once("</code></pre>").unwrap();
                std::fs::write(path, test);
            },
        };
    }
    pub fn read(&self) -> String {
        let module = module_path!().split("::").collect::<Vec<_>>();
        let mut path = std::path::PathBuf::new();
        path.extend(["src", "aoc2023", self.dir(), &self.handle()]);
        if !path.exists(){
            self.fetch(&path);
        }
        std::fs::read_to_string(&path).unwrap()
    }
}
