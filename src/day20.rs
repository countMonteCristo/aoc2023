use std::collections::{HashMap, HashSet, VecDeque};

use super::utils::{Result, lcm};

const BROADCASTER: &'static str = "broadcaster";

#[derive(Debug, PartialEq)]
enum ModuleType {
    FLIPFLOP,
    CONJUNCTION,
    BROADCAST,
    NONE,
}

#[derive(Debug)]
enum ModuleState {
    ON,
    OFF,
}

impl ModuleState {
    fn flip(&mut self) {
        *self = match self {
            Self::ON  => Self::OFF,
            Self::OFF => Self::ON,
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Signal {
    LOW,
    HIGH,
    NONE,
}

#[derive(Debug)]
struct Module {
    typ: ModuleType,
    state: ModuleState,
    input: Signal,
    output: Signal,
    mem: HashMap<String, Signal>,
}
type Childs = HashMap::<String, Vec<String>>;

impl Module {
    fn new(s: &str) -> (String, Module, Vec<String>) {
        let mut parts = s.split(" -> ");
        let from = parts.next().unwrap();
        let (typ, name) = if from == BROADCASTER {
            (ModuleType::BROADCAST, from.to_string())
        } else {
            let mt = match from.chars().nth(0).unwrap() {
                '%' => ModuleType::FLIPFLOP,
                '&' => ModuleType::CONJUNCTION,
                _ => unreachable!()
            };
            (mt, from[1..].to_string())
        };
        let to = parts.next().unwrap().split(", ").map(|s| s.to_string()).collect::<Vec<_>>();
        let mem = HashMap::new();
        (name, Module{typ, state: ModuleState::OFF, input: Signal::NONE, output: Signal::NONE, mem}, to)
    }

    fn reset(&mut self) {
        self.input  = Signal::NONE;
        self.output = Signal::NONE;
        self.state  = ModuleState::OFF;
        self.mem.values_mut().for_each(|s| *s = Signal::LOW);
    }

    fn processed(&mut self, input: Signal, from: &String) -> bool {
        match self.typ {
            ModuleType::NONE => {
                self.input = input;
                self.output = Signal::NONE;
                true
            },
            ModuleType::BROADCAST => {
                self.input = input;
                self.output = self.input.clone();
                false
            },
            ModuleType::FLIPFLOP => {
                match input {
                    Signal::HIGH | Signal::NONE => true,
                    Signal::LOW => {
                        self.input = input;
                        self.state.flip();
                        self.output = match self.state {
                            ModuleState::ON  => Signal::HIGH,
                            ModuleState::OFF => Signal::LOW,
                        };
                        false
                    }
                }
            },
            ModuleType::CONJUNCTION => {
                self.input = input;
                let mem_in = self.mem.get_mut(from).unwrap();
                 *mem_in = self.input.clone();
                 self.output = if self.mem.values().all(|s| s == &Signal::HIGH) {
                    Signal::LOW
                } else {
                    Signal::HIGH
                };
                false
            },
        }
    }
}
impl Default for Module {
    fn default() -> Self {
        Self{typ: ModuleType::NONE, state: ModuleState::OFF, input: Signal::NONE, output: Signal::NONE, mem: HashMap::new()}
    }
}

struct Modules {
    m: HashMap<String, Module>,
}
impl Modules {
    fn new(lines: &Vec<&str>) -> (Self, Childs, String) {
        let mut m = HashMap::new();
        let mut childs = HashMap::new();

        lines
            .iter()
            .map(|&s| Module::new(s))
            .for_each(|(name, module, to)| {
                m.insert(name.clone(), module);
                childs.insert(name, to);
            });

        let all = childs
            .values()
            .flatten()
            .cloned()
            .collect::<HashSet<_>>();

        all
            .iter()
            .for_each(|name| {
                if !m.contains_key(name) {
                    m.insert(name.clone(), Module::default());
                    childs.insert(name.clone(), Vec::new());
                }
            });

        let mut modules = Modules{m};
        modules.prepare_conj(&childs);

        (modules, childs, BROADCASTER.to_string())
    }
    fn prepare_conj(&mut self, childs: &Childs) {
        let mut conj = HashMap::<String, Vec<String>>::new();
        for (name, to) in childs.iter() {
            for to_name in to.iter() {
                let to_m = self.m.get(to_name).unwrap();
                if to_m.typ == ModuleType::CONJUNCTION {
                    if !conj.contains_key(to_name) {
                        conj.insert(to_name.clone(), Vec::new());
                    }
                    conj.get_mut(to_name).unwrap().push(name.clone());
                }
            }
        }

        for (conj_name, conj_inputs) in conj.iter() {
            self.m.get_mut(conj_name).unwrap().mem.extend(
                conj_inputs.iter().map(|i| (i.clone(), Signal::LOW))
            )
        }
    }

    fn press_button(&mut self, childs: &Childs, start: &String, n: u64, name: &String, desired: Signal) -> (u64, bool, (u64, u64)) {
        self.m.get_mut(start).unwrap().processed(Signal::LOW, start);

        let mut nlow = 1;
        let mut nhigh = 0;

        let mut q = VecDeque::<&String>::new();
        q.push_back(&start);
        while !q.is_empty() {
            let from = q.pop_front().unwrap();
            let output = self.m.get(from).unwrap().output.clone();
            if output == Signal::NONE {
                continue;
            }
            if from == name && output == desired {
                return (n, true, (nlow, nhigh));
            }
            for to in childs.get(from).unwrap().iter() {
                match output {
                    Signal::LOW  => nlow  += 1,
                    Signal::HIGH => nhigh += 1,
                    _ => unreachable!(),
                }

                if !self.processed(from, to) {
                    q.push_back(to);
                }
            }
            self.m.get_mut(from).unwrap().input = Signal::NONE;
        }
        (n, false, (nlow, nhigh))
    }

    fn processed(&mut self, from: &String, to: &String) -> bool {
        let input = self.m.get_mut(from).unwrap().output.clone();
        self.m.get_mut(to).unwrap().processed(input, from)
    }

    fn reset(&mut self) {
        self.m.values_mut().for_each(|m| m.reset());
    }

    fn count_presses(&mut self, childs: &Childs, start: &String, name: &String, desired: Signal) -> u64 {
        self.reset();
        let mut n = 1;
        while !self.press_button(&childs, start, n, name, desired).1 {
            n += 1;
        }
        n
    }

    fn get_main_conjunctions(&self, childs: &Childs, start: &String) -> HashSet<String> {
        let mut results = HashSet::new();

        let mut stack = vec![start];
        while !stack.is_empty() {
            let cur = stack.pop().unwrap();
            match self.m.get(cur).unwrap().typ {
                ModuleType::CONJUNCTION => {
                    let _ = results.insert(cur);
                },
                _ => {
                    stack.extend(childs.get(cur).unwrap())
                }
            }
        }

        results.iter().map(|&s| s.clone()).collect()
    }
}


fn solve1(modules: &mut Modules, childs: &Childs, start: &String) -> u64 {
    let (low, high) = (0..1000)
        .map(|n| modules.press_button(&childs, start, n, &"".to_string(), Signal::NONE).2)
        .reduce(|(al, ah), (l, h)| (al+l, ah+h))
        .unwrap();
    low * high
}

fn solve2(modules: &mut Modules, childs: &Childs, start: &String) -> u64 {
    modules.get_main_conjunctions(&childs, start)
        .iter()
        .map(|name| modules.count_presses(&childs, start, name, Signal::LOW))
        .reduce(|acc, n| lcm(acc, n))
        .unwrap()
}

pub fn run(data: &str, check: bool) -> Result {
    let lines = data.split('\n').collect();
    let (mut modules, childs, start) = Modules::new(&lines);

    let ans1 = solve1(&mut modules, &childs, &start);
    println!("Part1: {}", ans1);

    let ans2 = solve2(&mut modules, &childs, &start);
    println!("Part2: {}", ans2);

    if !check || (ans1 == 800830848 && ans2 == 244055946148853) {
        Ok(())
    } else {
        Err(())
    }
}

