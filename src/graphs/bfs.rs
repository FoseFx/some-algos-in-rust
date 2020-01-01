// breadth-first search
use std::iter::FromIterator;
use std::collections::HashSet;

type Vertex = usize;
type Graph = Vec<HashSet<Vertex>>;

pub fn some_graph() -> Graph {
    let mut graph = vec![];
    graph.push(std::collections::HashSet::from_iter(vec![1, 2, 5])); // 0
    graph.push(std::collections::HashSet::from_iter(vec![0])); // 1
    graph.push(std::collections::HashSet::from_iter(vec![0, 1, 3, 4])); // 2
    graph.push(std::collections::HashSet::from_iter(vec![2, 5, 4])); // 3
    graph.push(std::collections::HashSet::from_iter(vec![2, 3])); // 4
    graph.push(std::collections::HashSet::from_iter(vec![0, 3])); // 5
    graph.push(std::collections::HashSet::from_iter(vec![7])); // 6
    graph.push(std::collections::HashSet::from_iter(vec![6])); // 7
    return graph;
}

pub fn are_connected(graph: &Graph, a: &Vertex, b: &Vertex) -> bool {
    let visited = breadth_first_search(graph, a, Some(b));
    return visited.contains(b);
}

/// Returns visited vertecies
fn breadth_first_search<'a>(graph: &'a Graph, starting_point: &'a Vertex, stop_at: Option<&'a Vertex>) -> HashSet<&'a Vertex> {
    let mut q: Vec<&Vertex> = vec![starting_point];
    let mut visited_vertecies: HashSet<&Vertex> = HashSet::new();
    visited_vertecies.insert(starting_point);

    while !q.is_empty() {
        let this_vertex = q.remove(0);

        let neighbours = graph.get(*this_vertex);
        if neighbours.is_some() {
            for neighbour in neighbours.unwrap().iter() {
                if stop_at.is_some() {
                    let stop_at = stop_at.unwrap();
                    if neighbour == stop_at {
                        visited_vertecies.insert(neighbour);
                        break;
                    }
                }
                if !visited_vertecies.contains(neighbour) {
                    q.push(neighbour);
                    visited_vertecies.insert(neighbour);
                }
            }
        }
    }
    return visited_vertecies;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breadth_first_search() {
        let graph = some_graph();
        let visited = breadth_first_search(&graph, &0, None);
        assert_eq!(visited.contains(&0), true);
        assert_eq!(visited.contains(&1), true);
        assert_eq!(visited.contains(&2), true);
        assert_eq!(visited.contains(&3), true);
        assert_eq!(visited.contains(&4), true);
        assert_eq!(visited.contains(&5), true);
        assert_eq!(visited.contains(&6), false);
        assert_eq!(visited.contains(&7), false);
        let visited = breadth_first_search(&graph, &0, Some(&5));
        assert_eq!(visited.contains(&0), true);
        assert_eq!(visited.contains(&1), false);
        assert_eq!(visited.contains(&2), false);
        assert_eq!(visited.contains(&3), false);
        assert_eq!(visited.contains(&4), false);
        assert_eq!(visited.contains(&5), true);
        assert_eq!(visited.contains(&6), false);
        assert_eq!(visited.contains(&7), false);
    }

    #[test]
    fn test_are_connected() {
        assert_eq!(are_connected(&some_graph(), &0, &5), true);
        assert_eq!(are_connected(&some_graph(), &0, &4), true);
        assert_eq!(are_connected(&some_graph(), &0, &6), false);
        assert_eq!(are_connected(&some_graph(), &7, &6), true);
    }
}
