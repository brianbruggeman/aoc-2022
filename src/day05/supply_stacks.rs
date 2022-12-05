pub enum Machine {
    Crane9000(Move),
    Crane9001(Move),
}

pub fn arrange_crates_9001(instructions: &str) -> Vec<String> {
    let mut stacks = parse_stacks(instructions);
    let movements = parse_moves(instructions)
        .iter()
        .map(|movement| Machine::Crane9001(*movement))
        .collect::<Vec<_>>();
    execute_moves(&movements, &mut stacks);
    stacks
        .iter()
        .map(|stack| stack.clone().pop().unwrap())
        .collect::<Vec<_>>()
}

pub fn arrange_crates_9000(instructions: &str) -> Vec<String> {
    let mut stacks = parse_stacks(instructions);
    let movements = parse_moves(instructions)
        .iter()
        .map(|movement| Machine::Crane9000(*movement))
        .collect::<Vec<_>>();
    execute_moves(&movements, &mut stacks);
    stacks
        .iter()
        .map(|stack| stack.clone().pop().unwrap())
        .collect::<Vec<_>>()
}

pub fn parse_stacks(instructions: &str) -> Vec<Vec<String>> {
    let instructions = instructions
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    let end_of_stack_line_no = instructions
        .iter()
        .position(|line| line.trim().starts_with('1'))
        .expect("Could not find end of stack lines");
    let mut stack_lines = instructions[..end_of_stack_line_no].to_vec();
    stack_lines.reverse();
    let mut stacks: Vec<Vec<String>> = vec![];
    stack_lines
        .iter()
        .filter(|line| !line.trim().is_empty())
        .for_each(|line| {
            let row_data = line[1..].chars().step_by(4).collect::<Vec<_>>();
            for (stack_id, stack_value) in row_data.iter().enumerate() {
                if *stack_value == ' ' {
                    continue;
                }
                match stacks.get_mut(stack_id) {
                    Some(v) => v.push(stack_value.to_string()),
                    None => stacks.push(vec![stack_value.to_string()]),
                }
            }
        });
    stacks
}

#[derive(Copy, Clone, Debug)]
pub struct Move {
    count: usize,
    origination: usize,
    destination: usize,
}

impl Move {
    pub fn new(count: usize, origination: usize, destination: usize) -> Self {
        Self { count, origination, destination }
    }
}

pub fn parse_moves(instructions: &str) -> Vec<Move> {
    instructions
        .lines()
        .filter(|line| line.contains("move"))
        .map(|line| {
            let values = line
                .split(' ')
                .flat_map(|c| c.parse::<usize>())
                .collect::<Vec<usize>>();
            Move::new(values[0], values[1], values[2])
        })
        .collect::<Vec<_>>()
}

pub fn execute_moves(movements: &[Machine], stacks: &mut [Vec<String>]) {
    movements
        .iter()
        .for_each(|movement| execute_move(movement, stacks))
}

pub fn execute_move(movement: &Machine, stacks: &mut [Vec<String>]) {
    match movement {
        Machine::Crane9000(movement) => (0..movement.count).for_each(|_move| {
            let item = stacks[movement.origination - 1]
                .pop()
                .expect("Stack was not large enough.");
            if !item.is_empty() {
                stacks[movement.destination - 1].push(item);
            }
        }),
        Machine::Crane9001(movement) => {
            let mut items = (0..movement.count)
                .map(|_i| {
                    stacks[movement.origination - 1]
                        .pop()
                        .expect("No item found")
                })
                .collect::<Vec<_>>();
            items.reverse();
            stacks[movement.destination - 1].extend(items);
        }
    }
}
