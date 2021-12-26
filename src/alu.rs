#[macro_use]
extern crate scan_rules;

use std::collections::HashMap;
use std::iter::Iterator;

#[derive(Debug,Default, Eq, PartialEq, Hash, Clone)]
struct State {
    pub w: i32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl State {
    fn read(&self, registry: char) -> i32 {
        match registry {
            'w' => self.w,
            'x' => self.x,
            'y' => self.y,
            'z' => self.z,
            r => panic!("Invalid registry given: {}", r),
        }
    }

    fn read_and_modify<F>(&mut self, registry: char, mapper: F)
    where
        F: Fn(i32) -> i32,
    {
        match registry {
            'w' => self.w = mapper(self.w),
            'x' => self.x = mapper(self.x),
            'y' => self.y = mapper(self.y),
            'z' => self.z = mapper(self.z),
            r => panic!("Invalid registry given: {}", r),
        }
    }
}

#[derive(Debug)]
enum Operand {
    Register(char),
    Value(i32),
}

#[derive(Debug)]
enum Instruction {
    Inp(char),
    Add(char, Operand),
    Mul(char, Operand),
    Div(char, Operand),
    Mod(char, Operand),
    Eql(char, Operand),
}

fn resolve_operand(state: &State, o: &Operand) -> i32 {
    match *o {
        Operand::Register(r) => state.read(r),
        Operand::Value(x) => x,
    }
}

fn eval(state: &State, it: &mut dyn Iterator<Item = i32>, inst: &Instruction) -> Option<State> {
    let mut new_state: State = state.clone();
    match inst {
        Instruction::Inp(r) => {
            let n = it.next().unwrap();
            new_state.read_and_modify(*r, |_| n)
        }
        Instruction::Add(a, b) => new_state.read_and_modify(*a, |r| r + resolve_operand(state, b)),
        Instruction::Mul(a, b) => new_state.read_and_modify(*a, |r| r * resolve_operand(state, b)),
        Instruction::Div(a, b) => {
            let b_resolved = resolve_operand(state, b);
            if b_resolved == 0 {
                return None;
            }
            new_state.read_and_modify(*a, &|r| r / b_resolved);
        }
        Instruction::Mod(a, b) => {
            let b_resolved = resolve_operand(state, b);
            if state.read(*a) < 0 || b_resolved <= 0 {
                return None;
            }
            new_state.read_and_modify(*a, &|r| r % b_resolved);
        }
        Instruction::Eql(a, b) => {
            new_state.read_and_modify(*a, &|r| (r == resolve_operand(state, b)) as i32)
        }
    }
    Some(new_state)
}

fn is_valid_version(s: &State) -> bool {
    s.z == 0
}

fn dfs(
    state: State,
    inputs: &[i32],
    programs: &[Vec<Instruction>],
    // State, depth
    memo: &mut HashMap<(State, usize), Option<Vec<i32>>>,
) -> Option<Vec<i32>> {
    let depth = inputs.len();
    if let Some(r) = memo.get(&(state.clone(), depth)) {
        return r.clone();
    }
    if depth < programs.len() {
        for val in (1..10).rev() {
            let mut new_inputs: Vec<i32> = inputs.iter().cloned().collect();
            new_inputs.push(val);
            if let Some(final_state) = programs[depth].iter().fold(Some(state.clone()), |s, inst| {
                s.map(|ss| eval(&ss, &mut vec![val].into_iter(), inst))
                    .flatten()
            }) {
                if let Some(mut res) = dfs(final_state, &new_inputs, programs, memo) {
                    res.push(val);
                    memo.insert((state.clone(), depth), Some(res.clone()));
                    return Some(res);
                }
            }
            memo.entry((state.clone(), depth)).or_insert(None);
        }
        return None;
    } else if is_valid_version(&state) {
        return Some(vec![]);
    } else {
        return None;
    }
}

fn main() {
    let mut instructions: Vec<Vec<Instruction>> = vec![];
    while let Ok(i) = try_readln! {
        ("inp ", let r: char) => Instruction::Inp(r),
        ("add ", let a: char, " ", let b: i32) => Instruction::Add(a, Operand::Value(b)),
        ("add ", let a: char, " ", let b: char) => Instruction::Add(a, Operand::Register(b)),
        ("mul ", let a: char, " ", let b: i32) => Instruction::Mul(a, Operand::Value(b)),
        ("mul ", let a: char, " ", let b: char) => Instruction::Mul(a, Operand::Register(b)),
        ("div ", let a: char, " ", let b: i32) => Instruction::Div(a, Operand::Value(b)),
        ("div ", let a: char, " ", let b: char) => Instruction::Div(a, Operand::Register(b)),
        ("mod ", let a: char, " ", let b: i32) => Instruction::Mod(a, Operand::Value(b)),
        ("mod ", let a: char, " ", let b: char) => Instruction::Mod(a, Operand::Register(b)),
        ("eql ", let a: char, " ", let b: i32) => Instruction::Eql(a, Operand::Value(b)),
        ("eql ", let a: char, " ", let b: char) => Instruction::Eql(a, Operand::Register(b)),
    } {
        match i {
            Instruction::Inp(r) => instructions.push(vec![Instruction::Inp(r)]),
            x => instructions.last_mut().unwrap().push(x),
        }
    }

    let mut memo: HashMap<(State, usize), Option<Vec<i32>>> = HashMap::new();

    let r = dfs(State::default(), &vec![], &instructions, &mut memo);
    println!("{:?}", r);
}
