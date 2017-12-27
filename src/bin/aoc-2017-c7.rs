use std::io;
use std::collections::BTreeMap;
use std::num::ParseIntError;

extern crate petgraph;
use petgraph::Graph;
use petgraph::prelude::*;
use petgraph::visit::Dfs;


// we will model the tree using a directed graph where each node has outgoing edges to each of the
// child programs. The edges will be updated to have to total weight of all children. While this is
// a little backwards its easier to work with given that we need to create all the nodes before the
// edges from input.
//
// resulting implementation is a mess. I clearly need more practice with graph algos.
//
// this worked but resulted in 3 unbalanced nodes being detected. not sure if this correct of it
// there is a bug and the first answer I tried just worked

// Use for graph nodes
#[derive(Debug,Clone)]
struct Entry {
    weight: usize, // this is the node weight
    name: String,
}

// Intermediate data structure used during parsing input
#[derive(Debug)]
struct ParsedEntry {
    entry: Entry,
    children: Vec<String>,
}

//TODO remove unwrap
fn get_node_name(graph: &Graph<Entry, Option<usize>>, node: NodeIndex) -> String {
    graph.node_weight(node).unwrap().name.clone()
}

fn find_root(graph: &Graph<Entry, Option<usize>>) -> Result<NodeIndex, String>
{
    let candidates: Vec<_> = graph.externals(petgraph::Incoming).collect();
    match candidates.len() {
        0 => Err("No nodes in graph are root".to_string()),
        1 => Ok(candidates[0]),
        _ => Err("Multiple roots".to_string()),
    }
}

// consume a list of parsed entries into a graph. Does not calculate edge weights
fn mkgraph(parsed_entries: &[ParsedEntry]) -> Result<Graph<Entry, Option<usize>>, String> {
    let mut graph: Graph<Entry, Option<usize>> = Graph::new();
    let mut nodes: BTreeMap<String, _> = BTreeMap::new();

    // create nodes
    for pentry in parsed_entries {
        let idx = graph.add_node(pentry.entry.clone());
        nodes.insert(graph.node_weight(idx).unwrap().name.clone(), idx);
    }

    // add edges
    for pentry in parsed_entries {
        // we just put all these in the graph so no need to test
        let src_idx = nodes.get(pentry.entry.name.as_str()).unwrap().clone();
        // add an edge for each child
        for child in pentry.children.iter() {
            match nodes.get(child) {
                Some(x) => {
                    graph.add_edge(src_idx,x.clone(),None);
                }
                None => {
                    return Err(format!("Child '{}' node not found in graph", child));
                }
            }
        }
    }

    Ok(graph)
}

// get the weight of a node, if not already calculated will recursively calculate weight
fn get_weight_lazy(graph: &mut Graph<Entry, Option<usize>>,
                   node_idx: NodeIndex) -> Result<usize, String> {
    let mut child_walker = graph.neighbors_directed(node_idx, petgraph::Outgoing).detach();
    let mut sum : usize = 0;
    while let Some((edge_idx, child_idx)) = child_walker.next(graph) {
        let edge = (*(graph.edge_weight(edge_idx).unwrap())).clone();
        let child_weight = match edge {
            Some(w) => w,
            None => {
                let w = get_weight_lazy(graph, child_idx)?;
                *(graph.edge_weight_mut(edge_idx).unwrap()) = Some(w);
                w
            }
        };
        sum += child_weight;
    }
    if let Some(node) = graph.node_weight(node_idx) {
        Ok(sum + (*node).weight)
    } else {
        Err("Not a valid node index".to_string())
    }
}

#[derive(Debug)]
struct Imbal {
    idx: NodeIndex,   // index of imballanced node
    corrected: usize, // new value after correction
}

// use the most common weight as the "correct" weight
fn calc_correction(
    graph: &Graph<Entry, Option<usize>>,
    valmap: &BTreeMap<usize, Vec<NodeIndex>>
) -> Result<Imbal, String> {
    let mut max = 0usize; // max number of agreeing nodes
    let mut cor = 0usize; // target corrected total weight
    let mut bad = 0usize; // the offending node's total weight
    let mut node : Option<NodeIndex> = None; // offending node's index
    // find the value with the most nodes and candidate fixup node
    for (key, val) in valmap {
        let count = (*val).len();
        if count > max {
            max = count;
            cor = *key;
        }
        if count == 1 {
            node = Some((*val)[0].clone());
            bad = *key;
        }
    }
    // output result
    match node {
        Some(x) => {
            let delta = cor as isize - bad as isize;
            match graph.node_weight(x) {
                Some(w) => Ok(Imbal {
                    idx: x,
                    corrected: ((*w).weight as isize + delta) as usize,
                }),
                None => Err("Invalid node index in input map".to_string())
            }
        },
        None => Err("No single offending node".to_string()),
    }
}

// find imbalanced nodes in `graph` starting at node `idx`
// really should borrow graph immutably but I'm kind of done refactoring this code for now
fn find_imbals(graph: &mut Graph<Entry, Option<usize>>,
               idx: NodeIndex) -> Result<Vec<Imbal>, String> {
    let mut result : Vec<Imbal> = Vec::new();
    let mut dfs = Dfs::new(&(*graph), idx);
    while let Some(node_idx) = dfs.next(&(*graph)) {
        let mut child_walker = graph.neighbors_directed(node_idx, petgraph::Outgoing).detach();
        let mut valmap : BTreeMap<usize, Vec<NodeIndex>> = BTreeMap::new();
        // collect the weights for comparison
        while let Some(child_node_idx) = child_walker.next_node(graph) {
            let w = get_weight_lazy(graph, child_node_idx)?;
            let val = valmap.entry(w).or_insert(Vec::new());
            val.push(child_node_idx);
        }
        // check if any mismatch
        match valmap.len() {
            0...1 => (), // not imbalanced
            2 => { // imbalanced and only two weights present, solve for correct weight
                result.push(calc_correction(graph, &valmap)?);
            },
            _ => {return Err("Too many different node weights".to_string());}
        }
    }
    Ok(result)
}

// convert a list of lines into parsed entries
fn parse(input_lines: &[String]) -> Result<Vec<ParsedEntry>, String> {
    let mut entries = Vec::new();
    entries.reserve(input_lines.len());

    // internal helper for making entries from tokens
    fn mkentry(toks: &[&str]) -> Result<Entry, ParseIntError> {
        Ok (
            Entry {
                name: toks[0].to_string(),
                weight: toks[1]
                        .trim()
                        .trim_matches('(')
                        .trim_matches(')')
                        .parse::<usize>()?,
            }
        )
    }

    for line in input_lines {
        let toks: Vec<&str> = line.trim().split_whitespace().collect();
        if toks.len() >= 2 {
            let entry = match mkentry(toks.as_slice()) {
                Ok(ent) => ent,
                Err(_) => { return Err(format!("Invalid input at line {}",line));}
            };
            let children:Vec<String> = if toks.len() >= 4 {
                toks[3..]
                .iter()
                .map(|x| String::from(x
                                      .trim()
                                      .trim_matches(',')))
                .collect()
            } else {
                Vec::new()
            };
            entries.push(
                ParsedEntry {
                    entry: entry,
                    children: children
                }
            );
        }
        else {
            return Err(format!("Invalid input at line {}",line));
        }
    }
    Ok(entries)
}

fn main() {
    println!("Enter puzzle input: ");
    let mut inputs:Vec<String> = Vec::new();
    loop {
        let mut input_str = String::new();
        match io::stdin().read_line(&mut input_str) {
            Ok(num_bytes) if num_bytes > 1 => {
                inputs.push(input_str);
            }
            Ok(_) => {break;}
            Err(_) => {std::process::exit(1);}
        };
    };
    let entries = parse(&inputs).unwrap();
    let mut graph = mkgraph(&entries).unwrap();
    let root = find_root(&graph).unwrap();
    println!("Part 1: {}",get_node_name(&graph, root));
    println!("Part 2: {:?}",find_imbals(&mut graph, root).unwrap());
}

static example_graph : &'static str =
"pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

#[test]
fn test_example_1() {
    let lines:Vec<String> = String::from(example_graph).split_terminator('\n')
       .map(|x| String::from(x))
       .collect();
    let entries = parse(&lines).unwrap();
    let graph = mkgraph(&entries).unwrap();
    assert_eq!(get_node_name(&graph, find_root(&graph).unwrap()), "tknk")
}

#[test]
fn test_example_2() {
    let lines:Vec<String> = String::from(example_graph).split_terminator('\n')
       .map(|x| String::from(x))
       .collect();
    let entries = parse(&lines).unwrap();
    let mut graph = mkgraph(&entries).unwrap();
    let root_idx = find_root(&graph).unwrap();
    assert_eq!(get_weight_lazy(&mut graph, root_idx), Ok(778));
    let imbals = find_imbals(&mut graph, root_idx).unwrap();
    assert_eq!(imbals.len(), 1);
    assert_eq!(imbals[0].corrected, 60);
}
