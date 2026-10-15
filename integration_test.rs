use megastore::{graph::Graph, models::{Customer, Product}, recommender::recommend, seed::demo_graph};
use std::collections::HashSet;

#[test]
fn demo_recommends_related_products() {
    let graph = demo_graph();
    let results = recommend(&graph, "C1", 5, 3);
    assert!(!results.is_empty());
    assert!(results.iter().any(|r| r.id == "P2" || r.id == "P3" || r.id == "P4"));
}

#[test]
fn does_not_recommend_already_purchased_product() {
    let graph = demo_graph();
    let results = recommend(&graph, "C1", 10, 3);
    assert!(!results.iter().any(|r| r.id == "P1"));
}

#[test]
fn no_duplicate_recommendations() {
    let graph = demo_graph();
    let results = recommend(&graph, "C1", 20, 4);
    let mut ids = HashSet::new();
    for item in results { assert!(ids.insert(item.id)); }
}

#[test]
fn unknown_id_returns_empty() {
    let graph = demo_graph();
    assert!(recommend(&graph, "UNKNOWN", 5, 3).is_empty());
}

#[test]
fn product_lookup_works() {
    let mut graph = Graph::new();
    graph.add_product(Product { id: "P99".into(), name: "Produto teste".into(), category: "Teste".into() });
    assert_eq!(graph.product("P99").unwrap().name, "Produto teste");
}

#[test]
fn empty_customer_can_be_added() {
    let mut graph = Graph::new();
    graph.add_customer(Customer { id: "C99".into(), name: "Teste".into(), purchased: HashSet::new() });
    assert!(graph.customers.contains_key("C99"));
}
