use std::collections::HashMap;

fn process_input<'a>(
    airports: &'a Vec<&str>,
    routes: &Vec<(&str, &str)>,
) -> (HashMap<&'a str, usize>, Vec<Vec<usize>>) {
    let mut map = HashMap::new();
    let mut adj = Vec::new();
    // map each airport to a number
    for (id, airport) in airports.iter().enumerate() {
        map.insert(*airport, id);
        adj.push(Vec::new());
    }
    // build the adjacency list from the routes
    for route in routes {
        if let Some(&n) = map.get(&route.0) {
            if let Some(&m) = map.get(&route.1) {
                adj[n].push(m);
            }
        }
    }

    (map, adj)
}

fn kosaraju(adj: &Vec<Vec<usize>>) -> Vec<usize> {
    let n = adj.len(); // number of nodes in the graph

    // Depth-first traverse
    fn visit<F>(start: usize, adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>, f: &mut F)
    where
        F: FnMut(usize),
    {
        visited[start] = true;
        for out_neighbour in &adj[start] {
            if !visited[*out_neighbour] {
                visit(*out_neighbour, adj, visited, f);
            }
        }
        f(start);
    }

    // Traverse the graph
    let mut visited = vec![false; n];
    let mut stack = Vec::new();
    for u in 0..n {
        if !visited[u] {
            visit(u, adj, &mut visited, &mut |v| stack.push(v));
        }
    }

    // Build the inversed adjacency list
    let mut radj: Vec<Vec<usize>> = vec![vec![]; n];
    for i in 0..n {
        for j in &adj[i] {
            radj[*j].push(i);
        }
    }

    // Traverse in reverse order
    let mut visited = vec![false; n];
    let mut rep = vec![9; n];
    while let Some(u) = stack.pop() {
        if !visited[u] {
            visit(u, &radj, &mut visited, &mut |v| (rep[v] = u));
        }
    }

    // Representative node of the strongly connected component
    rep
}

fn solve(adj: &Vec<Vec<usize>>, start: usize) -> Vec<usize> {
    let n = adj.len();
    let rep = kosaraju(adj);

    // In degree of all strongly connected components
    let mut in_deg = vec![0; adj.len()];
    for (i, u) in adj.iter().enumerate() {
        for j in u {
            if rep[i] != rep[*j] {
                in_deg[rep[*j]] += 1;
            }
        }
    }

    // Number of nodes not reachable from start that aren't start
    let mut ans = Vec::new();
    for i in 0..n {
        if rep[i] == i && in_deg[i] == 0 && i != start {
            ans.push(i);
        }
    }

    ans
}

fn main() {
    let airports = vec![
        "BGI", "CDG", "DEL", "DOH", "DSM", "EWR", "EYW", "HND", "ICN",
        "JFK", "LGA", "LHR", "ORD", "SAN", "SFO", "SIN", "TLV", "BUD",
    ];
    let routes = vec![
        ("DSM", "ORD"),
        ("ORD", "BGI"),
        ("BGI", "LGA"),
        ("SIN", "CDG"),
        ("CDG", "SIN"),
        ("CDG", "BUD"),
        ("DEL", "DOH"),
        ("DEL", "CDG"),
        ("TLV", "DEL"),
        ("EWR", "HND"),
        ("HND", "ICN"),
        ("HND", "JFK"),
        ("JFK", "LGA"),
        ("EYW", "LHR"),
        ("LHR", "SFO"),
        ("SFO", "SAN"),
        ("SFO", "DSM"),
        ("SAN", "EYW"),
    ];

    let (map, adj) = process_input(&airports, &routes);

    println!("solve(&adj, map.get(LGA)) = {:?}", solve(&adj, *map.get("LGA").unwrap()));
}
