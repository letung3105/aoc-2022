use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl BinaryOperation {
    fn apply(&self, x: u64, y: u64) -> u64 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
            Self::Multiply => x * y,
            Self::Divide => x / y,
        }
    }
}

#[derive(Debug)]
enum Operand {
    Old,
    Num(u64),
}

#[derive(Debug)]
struct Operation {
    binary_operation: BinaryOperation,
    first_operand: Operand,
    second_operand: Operand,
}

impl Operation {
    fn apply(&self, old: u64) -> u64 {
        let (x, y) = match (&self.first_operand, &self.second_operand) {
            (&Operand::Old, &Operand::Old) => (old, old),
            (&Operand::Old, &Operand::Num(n)) => (old, n),
            (&Operand::Num(n), &Operand::Old) => (n, old),
            (&Operand::Num(n), &Operand::Num(m)) => (n, m),
        };
        self.binary_operation.apply(x, y)
    }
}

#[derive(Debug)]
struct TestCase {
    modulus: u64,
    outcome: (usize, usize),
}

impl TestCase {
    fn find_outcome(&self, value: u64) -> usize {
        if self.sastify(value) {
            self.outcome.0
        } else {
            self.outcome.1
        }
    }

    fn sastify(&self, value: u64) -> bool {
        value % self.modulus == 0
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test_case: TestCase,
}

impl Monkey {
    fn parse<I>(mut lines: I) -> std::io::Result<Self>
    where
        I: Iterator<Item = String>,
    {
        Self::parse_header(&mut lines)?;
        let items = Self::parse_starting_items(&mut lines)?;
        let operation = Self::parse_operation(&mut lines)?;
        let test_case = Self::parse_test_case(&mut lines)?;
        Ok(Self {
            items,
            operation,
            test_case,
        })
    }

    fn parse_header<I>(mut lines: I) -> std::io::Result<()>
    where
        I: Iterator<Item = String>,
    {
        let line = Self::try_get_line(&mut lines)?;
        Self::parse_prefixed(line.trim(), "Monkey")?;
        Ok(())
    }

    fn parse_starting_items<I>(mut lines: I) -> std::io::Result<Vec<u64>>
    where
        I: Iterator<Item = String>,
    {
        let line = Self::try_get_line(&mut lines)?;
        let starting_items_encoded = Self::parse_prefixed(line.trim(), "Starting items:")?;
        let mut starting_items = Vec::default();
        for tok in starting_items_encoded.split(",") {
            let starting_item = tok
                .trim()
                .parse()
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;
            starting_items.push(starting_item);
        }
        Ok(starting_items)
    }

    fn parse_operation<I>(mut lines: I) -> std::io::Result<Operation>
    where
        I: Iterator<Item = String>,
    {
        let line = Self::try_get_line(&mut lines)?;
        let operation_expression = Self::parse_prefixed(line.trim(), "Operation: new =")?;
        let mut operation_expression_tokens = operation_expression.split_whitespace();

        let first_operand_encoded =
            operation_expression_tokens
                .next()
                .ok_or(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "expecting an operand",
                ))?;
        let first_operand = Self::parse_operand(first_operand_encoded)?;

        let binary_operation_encoded =
            operation_expression_tokens
                .next()
                .ok_or(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "expecting an arithmetic operation",
                ))?;
        let binary_operation = Self::parse_binary_operation(binary_operation_encoded)?;

        let second_operand_encoded =
            operation_expression_tokens
                .next()
                .ok_or(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "expecting an operand",
                ))?;
        let second_operand = Self::parse_operand(second_operand_encoded)?;

        Ok(Operation {
            binary_operation,
            first_operand,
            second_operand,
        })
    }

    fn parse_test_case<I>(mut lines: I) -> std::io::Result<TestCase>
    where
        I: Iterator<Item = String>,
    {
        let line = Self::try_get_line(&mut lines)?;
        let divisible_by_encoded = Self::parse_prefixed(line.trim(), "Test: divisible by")?;
        let divisible_by = divisible_by_encoded
            .parse()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;

        let line = Self::try_get_line(&mut lines)?;
        let happy_case_encoded = Self::parse_prefixed(line.trim(), "If true: throw to monkey")?;
        let happy_case = happy_case_encoded
            .parse()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;

        let line = Self::try_get_line(&mut lines)?;
        let unhappy_case_encoded = Self::parse_prefixed(line.trim(), "If false: throw to monkey")?;
        let unhappy_case = unhappy_case_encoded
            .parse()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;

        Ok(TestCase {
            modulus: divisible_by,
            outcome: (happy_case, unhappy_case),
        })
    }

    fn parse_prefixed<'a>(s: &'a str, p: &str) -> std::io::Result<&'a str> {
        s.trim()
            .strip_prefix(p)
            .map(str::trim)
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("expecting '{}' prefix", p),
            ))
    }

    fn parse_binary_operation(s: &str) -> std::io::Result<BinaryOperation> {
        match s {
            "+" => Ok(BinaryOperation::Add),
            "-" => Ok(BinaryOperation::Subtract),
            "*" => Ok(BinaryOperation::Multiply),
            "/" => Ok(BinaryOperation::Divide),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "unknown operation",
            )),
        }
    }

    fn parse_operand(s: &str) -> std::io::Result<Operand> {
        match s {
            "old" => Ok(Operand::Old),
            s => {
                let n = s
                    .parse()
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
                Ok(Operand::Num(n))
            }
        }
    }

    fn try_get_line<I>(mut lines: I) -> std::io::Result<String>
    where
        I: Iterator<Item = String>,
    {
        lines.next().ok_or(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "expecting a line",
        ))
    }
}

fn run(file: File, iterations: usize, very_worry: bool) {
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(Result::unwrap).peekable();

    let mut monkeys = Vec::default();
    while let Some(line) = lines.peek() {
        if line.trim().is_empty() {
            lines.next();
        } else {
            monkeys.push(Monkey::parse(&mut lines).unwrap());
        }
    }

    let mut inspections_counts = vec![0usize; monkeys.len()];
    let modulus: u64 = monkeys.iter().map(|m| m.test_case.modulus).product();

    for _ in 0..iterations {
        for i in 0..monkeys.len() {
            let throws: Vec<(usize, u64)> = {
                let monkey = &mut monkeys[i];
                monkey
                    .items
                    .drain(..)
                    .map(|mut worry| {
                        worry = monkey.operation.apply(worry);
                        if !very_worry {
                            worry /= 3;
                        }
                        worry %= modulus;
                        (monkey.test_case.find_outcome(worry), worry)
                    })
                    .collect()
            };
            inspections_counts[i] += throws.len();
            for throw in throws {
                monkeys[throw.0].items.push(throw.1);
            }
        }
    }

    inspections_counts.sort();
    println!(
        "{}",
        inspections_counts.iter().rev().take(2).product::<usize>()
    );
}

fn main() {
    let fpath = env::args()
        .nth(1)
        .expect("Path to input file is not given!");

    run(File::open(&fpath).unwrap(), 20, false);
    run(File::open(&fpath).unwrap(), 10000, true);
}
