# TSP Genetic Algorithm - Larrañaga et al. (1999) Implementation

## 📖 Referência

> **P. Larrañaga, C.M.H. Kuijpers, R.H. Murga, I. Inza, S. Dizdarevic**  
> *Genetic Algorithms for the Travelling Salesman Problem: A Review of Representations and Operators.*  
> **Artificial Intelligence Review 13: 129–170, 1999.**

## 🎯 Configuração Implementada

Esta implementação reproduz **exatamente** a configuração que obteve os melhores resultados no paper:

### Componentes Principais:
- **Representação**: Path representation (permutação de cidades)
- **Seleção**: Linear Ranking (GENITOR) com pressão seletiva b = 1.90
- **Crossover**: Edge Recombination (ER) com heurística "min-list"
- **Mutação**: Insertion Mutation (ISM) com probabilidade pm = 0.01
- **Substituição**: Esquema GENITOR (1 filho por iteração)

### Parâmetros:
- **Tamanho da população**: μ = 200
- **Taxa de mutação**: pm = 0.01 (1%)
- **Pressão seletiva**: b = 1.90
- **Avaliações máximas**: 50.000
- **Critério de estagnação**: 1.000 iterações sem melhora no custo médio

### Cálculo de Distância:
- **Euclidiana 2D** arredondada para inteiro: `((dx² + dy²)^0.5 + 0.5).floor()`

## 🛠 Estrutura do Código

```
src/
├── main.rs           # Configuração e execução do experimento
├── tsp.rs           # Instância TSP e cálculo de distâncias
├── ga.rs            # Algoritmo Genético principal
└── operators.rs     # Operadores de crossover e mutação
data/
└── dj38.tsp         # Instância de teste (38 cidades, Djibouti)
```

## 🚀 Como Executar

```bash
# Compilar e executar
cargo run --release

# Ou apenas compilar
cargo build --release
```

## 📊 Resultados

### Instância dj38.tsp (38 cidades):
- **Ótimo conhecido**: 6.656
- **Resultado obtido**: ~7.181 (diferença de ~7.9%)
- **Critério de parada**: Estagnação (1.000 iterações sem melhora)

### Saída do Programa:
```
Iniciando Algoritmo Genético com os seguintes parâmetros:
 - População: 200
 - Avaliações Máximas: 50000
 - Taxa de Mutação: 0.01
 - Pressão Seletiva: 1.9

Critério de estagnação atingido (1000 iterações sem melhora no custo médio).

--- Resultados Finais ---
Custo da melhor rota encontrada: 7181.062738452499
```

## 🔬 Detalhes da Implementação

### Edge Recombination (ER):
- Constrói mapa de arestas dos dois pais
- Usa heurística "min-list" para escolher próxima cidade
- Mantém conectividade das arestas quando possível

### Insertion Mutation (ISM):
- Remove uma cidade aleatória do tour
- Reinsere em posição aleatória diferente
- Preserva ordem relativa das demais cidades

### Linear Ranking (GENITOR):
- Ordena população por fitness
- Probabilidade de seleção baseada na posição (ranking)
- Fórmula: P(i) = (2-b)/μ + 2*(i-1)*(b-1)/(μ*(μ-1))

## 📈 Características do Algoritmo

- **Elitista**: Sempre mantém o melhor indivíduo
- **Generacional**: Uma geração = uma substituição
- **Determinístico na seleção**: Ranking linear
- **Estocástico**: Crossover e mutação com aleatoriedade controlada

## 🔧 Dependências

```toml
[dependencies]
rand = "0.8"
```

## 📝 Notas de Implementação

1. **Fidelidade ao Paper**: Todos os parâmetros e operadores seguem exatamente a configuração vencedora
2. **Qualidade do Código**: Implementação modular e bem documentada em Rust
3. **Performance**: Uso de `--release` para otimizações do compilador
4. **Reprodutibilidade**: Seed aleatória pode ser fixada para experimentos determinísticos

---

**Este projeto implementa fielmente a melhor configuração encontrada por Larrañaga et al. (1999) para o TSP usando Algoritmos Genéticos.**
