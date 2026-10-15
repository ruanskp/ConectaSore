use std::collections::HashSet;
use crate::{graph::Graph, models::{Customer, Product, Relation}};

pub fn demo_graph() -> Graph {
    let mut graph = Graph::new();

    let products = vec![
        ("P1", "Notebook", "Eletrônicos"),
        ("P2", "Mouse sem fio", "Eletrônicos"),
        ("P3", "Teclado mecânico", "Eletrônicos"),
        ("P4", "Suporte para notebook", "Acessórios"),
        ("P5", "Garrafa térmica", "Casa"),
        ("P6", "Monitor", "Eletrônicos"),
        ("P7", "Mochila para notebook", "Acessórios"),
    ];
    for (id, name, category) in products {
        graph.add_product(Product { id: id.into(), name: name.into(), category: category.into() });
    }

    for (id, name, purchased) in [
        ("C1", "Cliente demonstração", vec!["P1"]),
        ("C2", "Cliente A", vec!["P1", "P2", "P3"]),
        ("C3", "Cliente B", vec!["P1", "P4", "P6"]),
    ] {
        let customer = Customer { id: id.into(), name: name.into(), purchased: HashSet::new() };
        graph.add_customer(customer);
        for product_id in purchased { graph.record_purchase(id, product_id); }
    }

    graph.connect_similar_products("P1", "P2", 3.0);
    graph.connect_similar_products("P1", "P3", 2.8);
    graph.connect_similar_products("P1", "P4", 2.2);
    graph.connect_similar_products("P1", "P6", 2.5);
    graph.connect_similar_products("P1", "P7", 1.8);
    graph.connect_similar_products("P2", "P3", 2.2);
    graph.connect_similar_products("P4", "P7", 2.0);
    graph.add_edge("C1", "P5", Relation::Interested, 0.5);
    graph
}
