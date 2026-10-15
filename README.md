# ConectaStore — MegaStore

Sistema acadêmico de recomendação de produtos em Rust usando grafo, lista de adjacência, `HashMap`, `VecDeque` (BFS) e `HashSet`.

## Requisitos
- Rust e Cargo instalados: https://www.rust-lang.org/tools/install

## Executar
```bash
cargo run
```

O programa apresenta produtos de demonstração e recomendações para o cliente `C1`. Ao final, digite um ID como `P1` para consultar recomendações a partir de um produto, ou pressione Enter para sair.

## Testes
```bash
cargo test
```

## Compilar executável
```bash
cargo build --release
```
No Windows, o executável será `target\\release\\megastore.exe`.

## Arquitetura
- `src/models.rs`: modelos de produto, cliente, aresta e recomendação.
- `src/graph.rs`: grafo e cadastro de entidades/conexões.
- `src/recommender.rs`: BFS limitada e pontuação de candidatos.
- `src/seed.rs`: dados fictícios de demonstração.
- `src/main.rs`: interface de terminal.
- `tests/integration_test.rs`: testes de integração.

## Estruturas e complexidade
- Lista de adjacência: memória `O(V + E)`.
- `HashMap`: busca média `O(1)`.
- BFS: `O(Vr + Er)` para a região visitada.
- Ordenação de R candidatos: `O(R log R)`.

## Desempenho
Ainda não há resultados de benchmark registrados. Execute os testes e acrescente medições reais antes da entrega. Não apresente estimativas como se fossem resultados medidos.

## Observações
Projeto acadêmico com dados fictícios. A pontuação e os pesos são demonstrativos e devem ser validados antes de uso comercial.
