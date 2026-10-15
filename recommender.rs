use std::collections::{HashMap, HashSet, VecDeque};
use crate::{graph::Graph, models::{Recommendation, Relation}};

/// Recomenda produtos por BFS limitada e pontuação ponderada.
/// `max_depth` limita a exploração; `limit` define quantos resultados retornar.
pub fn recommend(graph: &Graph, start_id: &str, limit: usize, max_depth: usize) -> Vec<Recommendation> {
    if limit == 0 || !graph.adjacency.contains_key(start_id) {
        return Vec::new();
    }

    let purchased: HashSet<String> = graph.customers.get(start_id)
        .map(|c| c.purchased.clone())
        .unwrap_or_default();

    let mut queue: VecDeque<(String, usize, f64)> = VecDeque::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut scores: HashMap<String, f64> = HashMap::new();
    queue.push_back((start_id.to_string(), 0, 1.0));
    visited.insert(start_id.to_string());

    while let Some((node, depth, path_score)) = queue.pop_front() {
        if depth >= max_depth { continue; }
        if let Some(edges) = graph.adjacency.get(&node) {
            for edge in edges {
                let next_depth = depth + 1;
                let weight = edge.weight.max(0.0);
                let next_score = path_score * weight / (next_depth as f64);

                if graph.products.contains_key(&edge.to) && edge.to != start_id && !purchased.contains(&edge.to) {
                    *scores.entry(edge.to.clone()).or_insert(0.0) += next_score;
                }

                if visited.insert(edge.to.clone()) {
                    queue.push_back((edge.to.clone(), next_depth, next_score));
                }
            }
        }
    }

    let mut results: Vec<Recommendation> = scores.into_iter()
        .filter_map(|(id, score)| graph.products.get(&id).map(|p| Recommendation {
            id: p.id.clone(), name: p.name.clone(), score
        }))
        .collect();
    results.sort_by(|a, b| b.score.total_cmp(&a.score).then_with(|| a.id.cmp(&b.id)));
    results.truncate(limit);
    results
}

pub fn relation_weight(relation: &Relation) -> f64 {
    match relation {
        Relation::Purchased => 3.0,
        Relation::Interested => 2.0,
        Relation::Similar => 2.5,
        Relation::SameCategory => 1.0,
    }
}
