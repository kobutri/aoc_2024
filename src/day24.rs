use itertools::Itertools;
use regex::Regex;
use std::collections::BTreeMap;
use petgraph::dot::Dot;
use petgraph::graph::DiGraph;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum WireState {
    True,
    False,
    Null,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum GateType {
    AND,
    OR,
    XOR,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Gate {
    input: (&'static str, &'static str),
    r#type: GateType,
    output: &'static str,
}

const xs: [&'static str; 45] = [
"x00",
"x01",
"x02",
"x03",
"x04",
"x05",
"x06",
"x07",
"x08",
"x09",
"x10",
"x11",
"x12",
"x13",
"x14",
"x15",
"x16",
"x17",
"x18",
"x19",
"x20",
"x21",
"x22",
"x23",
"x24",
"x25",
"x26",
"x27",
"x28",
"x29",
"x30",
"x31",
"x32",
"x33",
"x34",
"x35",
"x36",
"x37",
"x38",
"x39",
"x40",
"x41",
"x42",
"x43",
"x44",
];
const ys: [&'static str; 45] = [
    "y00",
    "y01",
    "y02",
    "y03",
    "y04",
    "y05",
    "y06",
    "y07",
    "y08",
    "y09",
    "y10",
    "y11",
    "y12",
    "y13",
    "y14",
    "y15",
    "y16",
    "y17",
    "y18",
    "y19",
    "y20",
    "y21",
    "y22",
    "y23",
    "y24",
    "y25",
    "y26",
    "y27",
    "y28",
    "y29",
    "y30",
    "y31",
    "y32",
    "y33",
    "y34",
    "y35",
    "y36",
    "y37",
    "y38",
    "y39",
    "y40",
    "y41",
    "y42",
    "y43",
    "y44",
];
fn get_input(fixed: bool) -> (BTreeMap<&'static str, WireState>, Vec<Gate>) {
    let input = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    let input = if fixed {
        include_str!("../input/day24_2.txt")
    } else {
        include_str!("../input/day24_1.txt")
    };
    let (initial, gates) = input.split_once("\n\n").unwrap();
    let initial_regex = Regex::new(r"([\w\d]+): (\d+)").unwrap();
    let mut current = BTreeMap::new();
    initial_regex.captures_iter(initial).for_each(|c| {
        let key = c.get(1).unwrap().as_str();
        let val = match c.get(2).unwrap().as_str().parse::<u8>().unwrap() {
            0 => WireState::False,
            1 => WireState::True,
            _ => unreachable!(),
        };
        current.insert(key, val);
    });
    let gate_regex = Regex::new(r"([\w\d]+) (\w+) ([\w\d]+) -> ([\w\d]+)").unwrap();
    let gates = gate_regex
        .captures_iter(gates)
        .map(|c| {
            let i1 = c.get(1).unwrap().as_str();
            current.entry(i1).or_insert(WireState::Null);
            let r#type = match c.get(2).unwrap().as_str() {
                "AND" => GateType::AND,
                "OR" => GateType::OR,
                "XOR" => GateType::XOR,
                _ => unreachable!(),
            };
            let i2 = c.get(3).unwrap().as_str();
            current.entry(i2).or_insert(WireState::Null);
            let output = c.get(4).unwrap().as_str();
            current.entry(output).or_insert(WireState::Null);

            Gate {
                input: (i1, i2),
                r#type,
                output,
            }
        })
        .collect_vec();

    (current, gates)
}

fn compute(mut current: BTreeMap<&'static str, WireState>, gates: &Vec<Gate>) -> Option<usize> {
    loop {
        let mut has_change = false;
        for gate in gates {
            let input1 = current.get(&gate.input.0).unwrap();
            let input2 = current.get(&gate.input.1).unwrap();
            let output = current.get(&gate.output).unwrap();
            if *input1 != WireState::Null
                && *input2 != WireState::Null
                && *output == WireState::Null
            {
                let new_output = if match gate.r#type {
                    GateType::OR => *input1 == WireState::True || *input2 == WireState::True,
                    GateType::AND => *input1 == WireState::True && *input2 == WireState::True,
                    GateType::XOR => input1 != input2,
                } {
                    WireState::True
                } else {
                    WireState::False
                };
                *current.get_mut(&gate.output).unwrap() = new_output;
                has_change = true;
            }
        }
        if !has_change {
            break;
        }
    }
    // println!("{:#?}", current);
    let total = num_from_letter(&current, 'z');
    total
}

fn num_from_letter(current: &BTreeMap<&'static str, WireState>, letter: char) -> Option<usize> {
    let str1 = format!("{}", letter);
    let str2 = format!("{}99", letter);
    let mut str = String::new();
    for (_z, val) in current.range(str1.as_str()..=str2.as_str()).rev() {
        let v = match val {
            WireState::True => '1',
            WireState::False => '0',
            WireState::Null => return None,
        };
        str.push(v);
    }
    let total = usize::from_str_radix(str.as_str(), 2).ok();
    total
}

pub fn day24_1() {
    let (current, gates) = get_input(false);
    println!("{}", compute(current, &gates).unwrap());
}

pub fn day24_2() {
    let (mut current, mut gates) = get_input(true);
    for i in 0..=44 {
        current.insert(xs[i], WireState::False);
        current.insert(ys[i], WireState::False);
    }
    for i in 0..=44 {
        let mut current = current.clone();
        current.insert(xs[i], WireState::True);
        current.insert(ys[i], WireState::True);
        let sum = compute(current.clone(), &gates).unwrap();
        if sum != 1 << (i + 1) {
            println!("{i}, {sum}");
        }
    }
    let mut graph = DiGraph::new();
    let mut node_map = BTreeMap::new();
    let mut output_map = BTreeMap::new();
    current.range("x"..="x99").for_each(|(n, _)| {
        node_map.insert(n.to_string(), graph.add_node(n.to_string()));
        node_map.insert(n.replace("x", "y"), graph.add_node(n.replace("x", "y")));
    });
    current.range("z"..="z99").for_each(|(n, _)| {
        output_map.insert(n.to_string(), graph.add_node(n.to_string()));
    });
    gates.iter().for_each(|g| {
        let node = graph.add_node(match g.r#type {
            GateType::AND => "AND".to_string(),
            GateType::OR => "OR".to_string(),
            GateType::XOR => "XOR".to_string(),
        });
        node_map.insert(g.output.to_string(), node);
    });
    gates.iter().for_each(|g| {
        if let Some(&node) = node_map.get(g.output) {
            if g.output.starts_with("z") {
                graph.add_edge(node, output_map[g.output], g.output);
            }
            if let Some(n) = node_map.get(g.input.0) {
                graph.add_edge(*n, node, g.input.0);
            }
            if let Some(n) = node_map.get(g.input.1) {
                graph.add_edge(*n, node, g.input.1);
            }
        }
    });
    // println!("{}", Dot::new(&graph));
    // calculated by hand
    println!("bhd,brk,dhg,dpd,nbf,z06,z23,z38");
}