# tsp-ga-larranaga1999

Implementação exploratória em Rust de Algoritmos Genéticos para o Problema do Caixeiro Viajante (TSP), centrada na análise e reprodução dos resultados publicados na literatura acadêmica.

---

## 📖 Contexto Literário

Este repositório visa investigar empiricamente os achados do artigo:

> **P. Larrañaga, C.M.H. Kuijpers, R.H. Murga, I. Inza, S. Dizdarevic**
> Genetic Algorithms for the Travelling Salesman Problem: A Review of Representations and Operators.
> *Artificial Intelligence Review 13: 129–170, 1999.*

A proposta é **reproduzir e explorar** as conclusões sobre:

* Representações de solução (path, adjacency, ordinal, matrix)
* Operadores de crossover (Edge Recombination, OX1, POS, OX2, PMX, CX, etc.)
* Operadores de mutação (Insertion, Inversion, Displacement, Swap, Scramble)
* Hibridização com busca local (ex: 2-opt, Lin–Kernighan)

Sem foco em tutoriais, mas sim em **comparações quantitativas** e **insights de pesquisa**.

---

## 🎯 Objetivos de Pesquisa

1. **Reprodução de Experimentos Clássicos**

   * Instâncias: Grötschel24, Grötschel48, capitais espanholas.
   * Métricas: melhor tour, média de tours, velocidade de convergência.

2. **Extensões Exploratórias**

   * Variações de parâmetros (população, prob. de mutação, pressão seletiva).
   * Inclusão/remoção de busca local (2-opt, Or-opt, Lin–Kernighan).
   * Comparação de representações alternativas.

3. **Análises**

   * Gráficos de convergência em diferentes configurações.
   * Estudos de sensibilidade dos operadores dominantes (ER + ISM).
   * Relatórios sobre trade-offs qualidade vs. custo computacional.

---

## 🛠 Implementação

* **Linguagem**: Rust
* **Estrutura sugerida**:

  ```text
  src/
  ├── main.rs       # entrada e configuração de experimentos
  ├── tsp.rs        # instâncias e avaliação de tours
  ├── ga.rs         # ciclo genético: seleção, cruzamento, mutação
  ├── operators.rs  # ER, OX1, POS, OX2, PMX, CX, ISM, IVM, DM...
  └── localsearch.rs# heurísticas 2-opt, LK...
  ```
* **Dependências** (em `Cargo.toml`):

  ```toml
  rand = "0.8"
  itertools = "0.10"
  plotters = "0.3"  # para visualizações exploratórias
  ```

---

## ⚙️ Como Conduzir Experimentos

1. Definir conjunto de instâncias em `data/` (TSPLIB ou CSV).
2. Configurar `ExperimentConfig` em `main.rs`:

   * operadores a testar, parâmetros de GA, número de repetições.
3. Executar:

   ```bash
   cargo run --release -- --config configs/exp1.toml
   ```
4. Resultados em `results/`: tabelas CSV e gráficos PNG.

---

## 📊 Saída Esperada

* Tabelas comparativas de tours (melhor / média).
* Plots de convergência por geração.
* Documentos Markdown com interpretação dos resultados.

---

## 📑 Referências

* Larrañaga et al. (1999). *Genetic Algorithms for the Travelling Salesman Problem...*
* Whitley et al. (1989, 1991). *Edge Recombination Operator.*
* Oliver et al. (1987). *Cycle Crossover.*
* Davis (1985). *Order Crossover.*

---

> **Nota**: Este projeto não é um tutorial, mas sim um **laboratório de pesquisa**; os códigos e resultados devem servir como base para exploração acadêmica.
