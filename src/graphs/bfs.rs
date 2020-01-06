// breadth-first search
use std::iter::FromIterator;
use std::collections::{HashSet, HashMap};

type Vertex = usize;
type Graph = Vec<HashSet<Vertex>>;

///
/// 
/// 
/// 0 <---------------------> 1
/// ^^        +---------------^
/// ||        |
/// |+-----> 2++
/// |         ||
/// |         ||
/// |         ||
/// |       3<------->4
/// v       ^
///  5 <----+
/// 
/// 
///  6 <-------> 7
/// 
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
    let (visited, _) = breadth_first_search(graph, a, Some(b));
    return visited.contains(b);
}

pub fn shortest_path<'a>(graph: &'a Graph, a: &'a Vertex, b: &'a Vertex) -> Option<Vec<&'a Vertex>> {
    let (visited, visited_by) = breadth_first_search(graph, a, Some(b));
    if !visited.contains(b) {
        return None;
    }
    let mut history = vec![b];
    let mut current_vertex = *visited_by.get(b).unwrap();
    while current_vertex != a {
        history.push(current_vertex);
        current_vertex = *visited_by.get(current_vertex).unwrap();
    }
    history.push(a);
    history.reverse();
    return Some(history);
}

/// Returns (visited vertecies, vertex first visited by)
fn breadth_first_search<'a>(graph: &'a Graph, starting_point: &'a Vertex, stop_at: Option<&'a Vertex>) -> (HashSet<&'a Vertex>, HashMap<&'a Vertex, &'a Vertex>) {
    let mut q: Vec<&Vertex> = vec![starting_point];
    let mut visited_vertecies: HashSet<&Vertex> = HashSet::new();
    let mut visited_by: HashMap<&Vertex, &Vertex> = HashMap::new();
    visited_vertecies.insert(starting_point);
    visited_by.insert(starting_point, starting_point);

    while !q.is_empty() {
        let this_vertex = q.remove(0);

        let neighbours = graph.get(*this_vertex);
        if neighbours.is_some() {
            for neighbour in neighbours.unwrap().iter() {
                if stop_at.is_some() {
                    let stop_at = stop_at.unwrap();
                    if neighbour == stop_at {
                        visited_vertecies.insert(neighbour);
                        if !visited_by.contains_key(neighbour) {
                            visited_by.insert(neighbour, this_vertex);
                        }
                        break;
                    }
                }
                if !visited_vertecies.contains(neighbour) {
                    q.push(neighbour);
                    visited_vertecies.insert(neighbour);
                    if !visited_by.contains_key(neighbour) {
                        visited_by.insert(neighbour, this_vertex);
                    }
                }
            }
        }
    }
    return (visited_vertecies, visited_by);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breadth_first_search() {
        let graph = some_graph();
        let (visited, v_by) = breadth_first_search(&graph, &0, None);
        assert_eq!(visited.contains(&0), true);
        assert_eq!(visited.contains(&1), true);
        assert_eq!(visited.contains(&2), true);
        assert_eq!(visited.contains(&3), true);
        assert_eq!(visited.contains(&4), true);
        assert_eq!(visited.contains(&5), true);
        assert_eq!(visited.contains(&6), false);
        assert_eq!(visited.contains(&7), false);
        
        assert_eq!(v_by[&0], &0);
        assert_eq!(v_by[&1], &0);
        assert_eq!(v_by[&2], &0);
        assert_eq!(v_by[&5], &0);
        assert!(v_by[&3] == &2 || v_by[&3] == &5);
        assert!(v_by[&4] == &2 || v_by[&4] == &3);
        assert_eq!(v_by.get(&6), None);
        assert_eq!(v_by.get(&7), None);

        let (visited, v_by) = breadth_first_search(&graph, &0, Some(&5));
        assert_eq!(visited.contains(&0), true);
        assert_eq!(visited.contains(&5), true);
        assert_eq!(visited.contains(&6), false);
        assert_eq!(visited.contains(&7), false);

        assert_eq!(v_by[&5], &0);
    }

    #[test]
    fn test_are_connected() {
        assert_eq!(are_connected(&some_graph(), &0, &5), true);
        assert_eq!(are_connected(&some_graph(), &0, &4), true);
        assert_eq!(are_connected(&some_graph(), &0, &6), false);
        assert_eq!(are_connected(&some_graph(), &7, &6), true);
    }
    fn test_shortest_path() {
        let graph = &some_graph();
        let res = shortest_path(graph, &0, &6);
        assert_eq!(res, None);

        let res = shortest_path(graph, &1, &5);
        assert_eq!(res.is_some(), true);
        let res = res.unwrap();
        assert_eq!(res, vec![&1, &0, &5])
    }
}
