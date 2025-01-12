use std::collections::{BTreeSet, HashMap, HashSet};

pub const INPUT1: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
pub const OUTPUT1: usize = 7;

pub const INPUT2: &str = INPUT1;
pub const OUTPUT2: &str = "co,de,ka,ta";

pub type Computer<'a> = &'a str;
pub type IncidentMap<'a> = HashMap<Computer<'a>, BTreeSet<Computer<'a>>>;
pub type Connection<'a> = (Computer<'a>, Computer<'a>);
pub struct Graph<'a> {
    pub incident_map: IncidentMap<'a>,
    pub connections: HashSet<Connection<'a>>,
    pub computers: BTreeSet<Computer<'a>>,
}
pub type Triple<'a> = (Computer<'a>, Computer<'a>, Computer<'a>);

pub fn parse_input(input: &str) -> Graph {
    let mut incident_map = IncidentMap::new();
    let mut connections: HashSet<Connection> = HashSet::new();
    input
        .lines()
        .map(|line| {
            let mut computers = line.split('-');
            (computers.next().unwrap(), computers.next().unwrap())
        })
        .for_each(|connection| {
            if connection.0 < connection.1 {
                connections.insert((connection.0, connection.1));
            } else {
                connections.insert((connection.1, connection.0));
            }
            incident_map
                .entry(connection.0)
                .and_modify(|end| {
                    end.insert(connection.1);
                })
                .or_insert(BTreeSet::from([connection.1]));
            incident_map
                .entry(connection.1)
                .and_modify(|end| {
                    end.insert(connection.0);
                })
                .or_insert(BTreeSet::from([connection.0]));
        });
    let computers: BTreeSet<Computer> = incident_map.keys().copied().collect();

    Graph {
        incident_map,
        connections,
        computers,
    }
}
