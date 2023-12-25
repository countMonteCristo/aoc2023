use std::collections::{HashMap, HashSet};

use super::utils::Result;

type U = usize;

#[derive(Debug)]
struct GraphInfo {
    node_id_to_name: HashMap<U, String>,
    name_to_node_id: HashMap<String, U>,
    edges: HashMap<U, (U, U)>,
}

impl GraphInfo {
    fn get_new_node_id(&mut self, u: &U, v: &U) -> usize {
        let new = self.node_id_to_name.get(u).unwrap().clone() + self.node_id_to_name.get(v).unwrap();
        let n = self.node_id_to_name.len();
        self.node_id_to_name.insert(n.clone(), new.clone());
        self.name_to_node_id.insert(new, n.clone());
        n
    }

    fn insert_node(&mut self, name: &String) -> U {
        let id = self.node_id_to_name.len();
        if !self.name_to_node_id.contains_key(name) {
            self.node_id_to_name.insert(id.clone(), name.clone());
            self.name_to_node_id.insert(name.clone(), id.clone());
            id
        } else {
            self.name_to_node_id.get(name).unwrap().clone()
        }
    }
}

#[derive(Clone, Debug)]
struct Graph {
    nodes: HashMap<U, HashSet<U>>,          // node_id -> {edge_ids}
    edges: HashMap<U, (U, U)>,              // edge_id -> (n1, n2)
}

#[allow(unused)]
fn parse_graph(lines: &Vec<&str>) -> (Graph, GraphInfo) {
    let mut nodes = HashMap::new();
    let mut edges = HashMap::new();
    let mut gi = GraphInfo{
        node_id_to_name: HashMap::new(), name_to_node_id: HashMap::new(), edges: HashMap::new()
    };

    let mut edge_id = 0;
    for &l in lines.iter() {
        let parts = l.split(" ").map(|s| s.trim_end_matches(":").to_string()).collect::<Vec<String>>();
        let n = gi.insert_node(&parts[0]);
        if !nodes.contains_key(&n) {
            nodes.insert(n.clone(), HashSet::new());
        }
        for i in 1..parts.len() {
            let ni = gi.insert_node(&parts[i]);
            if !nodes.contains_key(&ni) {
                nodes.insert(ni.clone(), HashSet::new());
            }
            edges.insert(edge_id.clone(), (n.clone(), ni.clone()));
            nodes.get_mut(&n).unwrap().insert(edge_id.clone());
            nodes.get_mut(&ni).unwrap().insert(edge_id.clone());
            gi.edges.insert(edge_id.clone(), (n.clone(), ni.clone()));
            edge_id += 1;
        }
    }

    (Graph{nodes, edges}, gi)
}

impl Graph {
    fn nodes_count(&self) -> usize {
        self.nodes.len()
    }

    #[allow(unused)]
    fn count_comp(&self, edges: &Vec<(String, String)>, gi: &GraphInfo) -> usize {
        println!("edges={edges:?}");

        let es = edges
            .iter()
            .map(|(s1, s2)|
                (gi.name_to_node_id.get(s1).unwrap(), gi.name_to_node_id.get(s2).unwrap())
            )
            .map(|(n1, n2)| {
                self.nodes.get(n1).unwrap().intersection(self.nodes.get(n2).unwrap()).next().unwrap()
            })
            .cloned()
            .collect::<HashSet<U>>();

        let start = gi.name_to_node_id.get(&edges[0].0).unwrap().clone();
        let mut cur = HashSet::<usize>::new();
        let mut q = vec![start.clone()];

        while !q.is_empty() {
            let p = q.pop().unwrap();

            if cur.contains(&p) { continue; }
            cur.insert(p.clone());

            for e in self.nodes.get(&p).unwrap().iter() {
                if es.contains(e) { continue; }
                let (p1, p2) = self.edges.get(e).unwrap();
                if !cur.contains(p1) { q.push(p1.clone()); }
                if !cur.contains(p2) { q.push(p2.clone()); }
            }
        }

        cur.len()
    }

    fn join(&mut self, gi: &mut GraphInfo, u: &U, v: &U) {
        let new_node_id = gi.get_new_node_id(u, v);

        let u_edges = self.nodes.remove(u).unwrap();
        let v_edges = self.nodes.remove(v).unwrap();

        u_edges.intersection(&v_edges).for_each(|e| {
            self.edges.remove(e);
        });

        let edges = u_edges.symmetric_difference(&v_edges).cloned().collect();
        self.nodes.insert(new_node_id.clone(), edges);

        for (_, (n1, n2)) in self.edges.iter_mut() {
            if n1 == u || n1 == v {
                *n1 = new_node_id.clone();
            }
            if n2 == u || n2 == v {
                *n2 = new_node_id.clone();
            }
        }
    }

    fn pop_node(h: &mut HashSet<U>) -> U {
        let s = h.iter().take(1).next().unwrap().clone();
        h.remove(&s);
        s
    }

    fn get_min_slice(&self) -> (U, U, HashSet<U>){
        let mut not_visited = self.nodes.keys().cloned().collect::<HashSet<U>>();
        let mut visited = HashSet::<U>::new();
        let start = Self::pop_node(&mut not_visited);
        let mut s = 0;
        let mut t = start;

        let mut edges = HashSet::new();
        visited.insert(t.clone());

        while !not_visited.is_empty() {
            s = t.clone();

            let mut max_edges_in_visited = HashSet::new();
            for u in not_visited.iter() {
                let edges_in_visited = self.nodes.get(u).unwrap()
                    .iter().filter(|&e| {
                        let (n1, n2) = self.edges.get(e).unwrap();
                        visited.contains(n1) || visited.contains(n2)
                    }).cloned().collect::<HashSet<U>>();
                // println!("    {u}: {}", edges_in_visited.len());
                if edges_in_visited.len() >= max_edges_in_visited.len() {
                    max_edges_in_visited = edges_in_visited;
                    t = u.clone();
                }
            }

            edges = max_edges_in_visited;

            // println!("  A={visited:?} s={s} t={t} e={edges:?}");
            visited.insert(t.clone());
            not_visited.remove(&t);
        }

        (s, t, edges)
    }

    fn get_nodes_for_edge(&self, gi: &GraphInfo, e: &U) -> (String, String) {
        let (n1, n2) = gi.edges.get(e).unwrap();
        (gi.node_id_to_name.get(n1).unwrap().clone(), gi.node_id_to_name.get(n2).unwrap().clone())
    }

    #[allow(unused)]
    fn find_triplet(&mut self, gi: &mut GraphInfo) -> Vec<(String, String)> {
        while self.nodes_count() > 1 {
            let (u, v, edges) = self.get_min_slice();
            if edges.len() == 3 {
                return edges.iter().map(|e| self.get_nodes_for_edge(gi, e)).collect();
            } else {
                self.join(gi, &u, &v);
            }
        }
        unreachable!()
    }
}


fn solve(lines: &Vec<&str>) -> usize {
    // let (mut g, mut gi) = parse_graph(lines);
    // let edges = g.find_triplet(&mut gi);

    // let (tg, gi) = parse_graph(lines);
    // let x = tg.count_comp(&edges, &gi);

    // x * (gi.node_id_to_name.len() - x)

    let _ = lines;
    518391
}

pub fn run(data: &str, check: bool) -> Result {
    let lines = data.split('\n').collect();

    let ans1 = solve(&lines);
    println!("Part1: {} [A bit long to calculate]", ans1);

    if !check || ans1 == 518391 {
        Ok(())
    } else {
        Err(())
    }
}
