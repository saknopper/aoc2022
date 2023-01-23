use std::collections::HashMap;

use sscanf::sscanf;

#[derive(Debug)]
struct Valve {
    flow_rate: isize,
    connected_to: Vec<String>,
}

fn main() {
    println!("Day 16");

    let input = include_str!("../example/part1.txt");

    let valves = parse_input(input);

    valves.iter().for_each(|(name, valve)| println!("{:?}: {:?}", name, valve));
}

fn parse_input(input: &str) -> HashMap<String, Valve> {
    let mut valves: HashMap<String, Valve> = HashMap::new();
    input.lines().for_each(|l| {
        let normalized = l.replace("tunnels", "tunnel").replace("leads", "lead").replace("valves", "valve");
        let (name, flow_rate, connected_to) =
            sscanf!(normalized, "Valve {String} has flow rate={isize}; tunnel lead to valve {str}").unwrap();
        let peers: Vec<String> = connected_to.split(", ").map(str::to_string).collect();
        valves.insert(name, Valve { flow_rate: flow_rate, connected_to: peers });
    });

    valves
}
