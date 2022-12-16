use std::collections::HashMap;
use std::io::{self, Read};

struct Node {
    key: String,
    rate: u32,
    edges: Vec<String>,
}

type Graph = HashMap<String, Node>;

#[derive(Debug)]
struct CompressedNode {
    key: u8,
    name: String,
    rate: u32,
    edges: Vec<u32>,
}

type CompressedGraph = Vec<CompressedNode>;

impl Node {
    fn from_input_line(line: &str) -> Node {
        let parts: Vec<_> = line.split(" ").collect();

        let key = parts[1].to_owned();
        let rate = parts[4]
            .trim_matches(&['r', 'a', 't', 'e', '=', ';'] as &[_])
            .parse::<u32>()
            .unwrap();
        let edges: Vec<_> = parts[9..]
            .iter()
            .map(|p| p.trim_matches(',').to_owned())
            .collect();

        Node { key, rate, edges }
    }
}

fn compute_distances(graph: &Graph, important_nodes: &Vec<&str>) -> (Vec<Vec<u32>>, Vec<u32>) {
    let mut final_distances = vec![vec![u32::MAX; important_nodes.len()]; important_nodes.len()];
    let mut distances = HashMap::<&str, HashMap<&str, u32>>::new();

    for (k1, n1) in graph {
        let mut n1_dists = HashMap::<&str, u32>::new();
        for (k2, n2) in graph {
            n1_dists.insert(k2.as_str(), 1000);
        }
        for e in &n1.edges {
            n1_dists.insert(e.as_str(), 1);
        }
        n1_dists.insert(k1.as_str(), 0);
        distances.insert(k1.as_str(), n1_dists);
    }

    for k in graph.keys() {
        for i in graph.keys() {
            for j in graph.keys() {
                let new_dist =
                    distances[i.as_str()][k.as_str()] + distances[k.as_str()][j.as_str()];
                if distances[i.as_str()][j.as_str()] > new_dist {
                    distances
                        .get_mut(i.as_str())
                        .unwrap()
                        .insert(j.as_str(), new_dist);
                }
            }
        }
    }

    for (i1, k1) in important_nodes.iter().enumerate() {
        for (i2, k2) in important_nodes.iter().enumerate() {
            final_distances[i1][i2] = distances[k1][k2];
        }
    }

    let initial_distances: Vec<_> = important_nodes
        .iter()
        .map(|k| distances[&"AA"][k])
        .collect();

    (final_distances, initial_distances)
}

fn compress(graph: &Graph) -> (CompressedGraph, Vec<u32>) {
    let mut compressed = CompressedGraph::new();

    let important_nodes: Vec<&str> = graph
        .iter()
        .filter(|(_, v)| v.rate != 0)
        .map(|(k, _)| k.as_str())
        .collect();
    let (distances, initial_distances) = compute_distances(&graph, &important_nodes);

    for (id, name) in important_nodes.iter().enumerate() {
        let orig_node = &graph[name.to_owned()];

        compressed.push(CompressedNode {
            name: name.to_string(),
            key: id as u8,
            rate: orig_node.rate,
            edges: distances[id].clone(),
        })
    }

    (compressed, initial_distances)
}

type CacheKey = (
    u8,  // Time spent
    u8,  // Current node
    u64, // Opened valves
);
type ReleasedCache = HashMap<CacheKey, u32>;

fn solve(graph: &CompressedGraph, initial: &Vec<u32>) {
    let mut cache = ReleasedCache::new();

    for (node, dist) in initial.iter().enumerate() {
        cache.insert((*dist as u8 + 1, node as u8, 1 << node), 0);
    }

    for time in 1..=30 {
        for node in 0..(graph.len() as u8) {
            for mask in 0..(1 << graph.len()) {
                let total_rate: u32 = graph
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| (mask & (1 << i)) != 0)
                    .map(|(_, n)| n.rate)
                    .sum();

                // State transition 1: stay in place
                if let Some(pressure_before) = cache.get(&(time - 1, node, mask)) {
                    let new_pressure = pressure_before + total_rate;
                    let key = (time, node, mask);
                    if let Some(max_known) = cache.get(&key) {
                        if *max_known < new_pressure {
                            cache.insert(key, new_pressure);
                        }
                    } else {
                        cache.insert(key, new_pressure);
                    }
                }

                // State transition 2: move to another valve and open it
                if let Some(&pressure_before) = cache.get(&(time, node, mask)) {
                    for dst in 0..(graph.len() as u8) {
                        if dst == node || (mask & (1 << dst)) != 0 {
                            continue;
                        }

                        let dist = graph[node as usize].edges[dst as usize];
                        let new_time = time + (dist as u8) + 1;
                        if new_time > 30 {
                            continue;
                        }

                        let new_pressure = pressure_before + total_rate * (dist + 1);
                        let key = (new_time, dst, mask | (1 << dst));
                        if let Some(max_known) = cache.get(&key) {
                            if *max_known < new_pressure {
                                cache.insert(key, new_pressure);
                            }
                        } else {
                            cache.insert(key, new_pressure);
                        }
                    }
                }
            }
        }

        let local_max = cache
            .iter()
            .filter(|((t, _, _), _)| *t == time)
            .map(|(_, v)| v)
            .max();
        if let Some(local_max) = local_max {
            println!("Max at time {}: {}", time, local_max);
            let valves: Vec<_> = cache
                .iter()
                .filter(|(_, v)| *v == local_max)
                .map(|((_, n, m), _)| (n, m))
                .collect();
            //println!("Valves: {:#?}", valves);
        }
    }

    let result = cache.values().max().unwrap();
    println!("Part 1 result: {}", result);

    let max_mask: u64 = cache.keys().map(|(_, _, m)| *m).max().unwrap();
    let mut max = 0;
    for m_both in 0..=max_mask {
        for m_e in 0..=max_mask {
            if (m_both & m_e) != m_e {
                continue;
            }

            let mut max_e = 0;
            let mut max_h = 0;
            for node in 0..(graph.len() as u8) {
                max_h = max_h.max(*cache.get(&(26, node, m_both & !m_e)).unwrap_or(&0));
                max_e = max_e.max(*cache.get(&(26, node, m_e)).unwrap_or(&0));
            }

            max = max.max(max_h + max_e);
        }
    }
    println!("Part 2 result: {}", max);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let graph: Graph = input
        .trim()
        .split("\n")
        .map(Node::from_input_line)
        .map(|s| (s.key.clone(), s))
        .collect();

    let (compressed_graph, initial_distances) = compress(&graph);

    solve(&compressed_graph, &initial_distances);
}
