mod graph;
mod models;
mod recommender;
mod seed;

use graph::Graph;
use recommender::recommend;
use seed::demo_graph;

fn main() {
    let graph = demo_graph();
    println!("=== ConectaStore | MegaStore ===");
    println!("Demonstração de recomendação baseada em grafos\n");

    println!("Produtos cadastrados:");
    for product in graph.products.values() {
        println!("- [{}] {} ({})", product.id, product.name, product.category);
    }

    let customer_id = "C1";
    println!("\nRecomendações para {customer_id}:");
    let results = recommend(&graph, customer_id, 5, 3);
    if results.is_empty() {
        println!("Nenhuma recomendação encontrada.");
    } else {
        for (i, item) in results.iter().enumerate() {
            println!("{}. {} — pontuação {:.1}", i + 1, item.name, item.score);
        }
    }

    println!("\nDigite um ID de produto para consultar recomendações (ex.: P1),");
    println!("ou pressione Enter para sair.");
    let mut input = String::new();
    if std::io::stdin().read_line(&mut input).is_ok() {
        let id = input.trim();
        if !id.is_empty() {
            println!("\nRecomendações a partir de {id}:");
            let results = recommend(&graph, id, 5, 3);
            if results.is_empty() {
                println!("Nenhuma recomendação encontrada para esse ID.");
            } else {
                for (i, item) in results.iter().enumerate() {
                    println!("{}. {} — pontuação {:.1}", i + 1, item.name, item.score);
                }
            }
        }
    }
}
