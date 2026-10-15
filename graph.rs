use std::collections::HashMap;
use crate::models::{Customer, Edge, Product, Relation};

#[derive(Default, Debug)]
pub struct Graph {
    pub products: HashMap<String, Product>,
    pub customers: HashMap<String, Customer>,
    pub adjacency: HashMap<String, Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Self { Self::default() }

    pub fn add_product(&mut self, product: Product) {
        self.adjacency.entry(product.id.clone()).or_default();
        self.products.insert(product.id.clone(), product);
    }

    pub fn add_customer(&mut self, customer: Customer) {
        self.adjacency.entry(customer.id.clone()).or_default();
        self.customers.insert(customer.id.clone(), customer);
    }

    pub fn add_edge(&mut self, from: &str, to: &str, relation: Relation, weight: f64) {
        if !self.adjacency.contains_key(from) || !self.adjacency.contains_key(to) {
            return;
        }
        let edges = self.adjacency.entry(from.to_string()).or_default();
        if let Some(existing) = edges.iter_mut().find(|e| e.to == to && e.relation == relation) {
            existing.weight = weight;
        } else {
            edges.push(Edge { to: to.to_string(), relation, weight });
        }
    }

    pub fn connect_similar_products(&mut self, a: &str, b: &str, weight: f64) {
        self.add_edge(a, b, Relation::Similar, weight);
        self.add_edge(b, a, Relation::Similar, weight);
    }

    pub fn record_purchase(&mut self, customer_id: &str, product_id: &str) {
        if !self.customers.contains_key(customer_id) || !self.products.contains_key(product_id) {
            return;
        }
        if let Some(customer) = self.customers.get_mut(customer_id) {
            customer.purchased.insert(product_id.to_string());
        }
        self.add_edge(customer_id, product_id, Relation::Purchased, 3.0);
        self.add_edge(product_id, customer_id, Relation::Purchased, 3.0);
    }

    pub fn product(&self, id: &str) -> Option<&Product> {
        self.products.get(id)
    }
}
