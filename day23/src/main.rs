use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut graph = HashMap::<_, HashSet<_>>::new();

    input.lines().for_each(|line| {
        let (a, b) = line.split_once("-").unwrap();

        graph.entry(a.to_owned()).or_default().insert(b.to_owned());
        graph.entry(b.to_owned()).or_default().insert(a.to_owned());
    });

    let mut seen = HashSet::new();

    graph
        .keys()
        .filter(|name| name.starts_with("t"))
        .for_each(|name| {
            fn dfs(
                graph: &HashMap<String, HashSet<String>>,
                node: &String,
                depth: u32,
                mut path: Vec<String>,
                seen: &mut HashSet<Vec<String>>,
            ) {
                if depth == 0 {
                    path.sort();
                    seen.insert(path);
                    return;
                }
                path.push(node.clone());
                graph.get(node).unwrap().iter().for_each(|edge| {
                    if if depth != 1 {
                        !path.contains(edge)
                    } else {
                        edge == path.first().unwrap()
                    } {
                        dfs(graph, edge, depth - 1, path.clone(), seen);
                    }
                });
            }

            let path = Vec::new();
            dfs(&graph, name, 3, path, &mut seen);
        });

    println!("Part 1: {}", seen.len());

    let mut max_clique = graph
        .keys()
        .map(|key| {
            let mut clique = Vec::new();
            clique.push(key.clone());

            for conn in graph.get(key).unwrap() {
                if clique
                    .iter()
                    .all(|other_conn| graph.get(conn).unwrap().contains(other_conn))
                {
                    clique.push(conn.clone());
                }
            }
            clique
        })
        .max_by_key(|clique| clique.len())
        .unwrap();

    max_clique.sort();

    println!("Part 2: {}", max_clique.join(","));
}
