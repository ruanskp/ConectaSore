use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub category: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Customer {
    pub id: String,
    pub name: String,
    pub purchased: HashSet<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Edge {
    pub to: String,
    pub relation: Relation,
    pub weight: f64,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Relation {
    Purchased,
    Interested,
    Similar,
    SameCategory,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Recommendation {
    pub id: String,
    pub name: String,
    pub score: f64,
}
