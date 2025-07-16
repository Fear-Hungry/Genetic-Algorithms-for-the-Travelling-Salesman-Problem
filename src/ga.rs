use rand::prelude::*;
use rand::distributions::{Distribution, WeightedIndex};
use crate::tsp::{Instance, Tour};
use crate::operators::{edge_recombination_crossover, insertion_mutation};

// Estrutura de configuração final, fiel ao paper
pub struct GAConfig {
    //pub instance: Instance,
    pub population_size: usize,    // μ = 200
    pub mutation_rate: f64,        // pm = 0.01
    pub selective_pressure: f64,   // b = 1.90
    pub max_evaluations: usize,    // 50_000 avaliações
    pub stagnation_limit: usize,   // 1_000 iterações sem melhora
}

pub struct Population{
    pub individuals: Vec<Tour>,
}

impl Population{
    // População Randômica
    pub fn new_random(instance: &Instance, config: &GAConfig) -> Self {
        let mut rng = thread_rng();
        let mut inds = Vec::with_capacity(config.population_size);
        for _ in 0..config.population_size {
            let mut tour = Tour {
                cities: (0..instance.num_cities).collect(),
                cost: 0.0,
            };
            tour.cities.shuffle(&mut rng);
            tour.calculate_cost(instance);
            inds.push(tour);
        }
        Population { individuals: inds }
    }

    pub fn sort_by_cost(&mut self){
        self.individuals.sort_unstable_by(|a, b| a.cost.partial_cmp(&b.cost).unwrap_or(std::cmp::Ordering::Equal));
    }

    pub fn ranked_select(&self, b: f64) -> &Tour {
        let n = self.individuals.len() as f64;
        let mut rng = thread_rng();
        let probs: Vec<f64> = (0..self.individuals.len()).map(|i| {
            (1.0/n) * (b - (2.0 * (b - 1.0) * (i as f64)) / (n - 1.0))
        }).collect();
        let dist = WeightedIndex::new(&probs).expect("Falha ao criar distribuição ponderada.");
        &self.individuals[dist.sample(&mut rng)]
    }
}

/// Loop principal final com ambos os critérios de parada
pub fn run_ga(instance: &Instance, config: &GAConfig) -> Tour {
    let mut population = Population::new_random(instance, config);
    population.sort_by_cost();

    let mut best_tour = population.individuals[0].clone();
    let mut evaluations = config.population_size;

    // Variáveis para o critério de estagnação
    let mut last_avg_cost = population.individuals.iter().map(|t| t.cost).sum::<f64>() / population.individuals.len() as f64;
    let mut stagnation_counter = 0;

    while evaluations < config.max_evaluations {
        // 1. Selecionar pais
        let p1 = population.ranked_select(config.selective_pressure);
        let p2 = population.ranked_select(config.selective_pressure);

        // 2. Crossover (sempre ocorre) e Mutação
        let mut child = edge_recombination_crossover(p1, p2);
        insertion_mutation(&mut child, config.mutation_rate);
        
        // 3. Avaliar o filho
        child.calculate_cost(instance);
        evaluations += 1;

        // 4. Esquema de substituição GENITOR
        let worst_tour = population.individuals.last().unwrap();
        if child.cost < worst_tour.cost {
            population.individuals.pop();
            let pos = population.individuals.binary_search_by(|t| t.cost.partial_cmp(&child.cost).unwrap()).unwrap_or_else(|e| e);
            population.individuals.insert(pos, child);
        }
        
        // 5. Atualizar o melhor tour global
        if population.individuals[0].cost < best_tour.cost {
            best_tour = population.individuals[0].clone();
        }

        // 6. Lógica de verificação de estagnação
        let current_avg_cost = population.individuals.iter().map(|t| t.cost).sum::<f64>() / population.individuals.len() as f64;
        if current_avg_cost >= last_avg_cost {
            stagnation_counter += 1;
        } else {
            stagnation_counter = 0; // Houve melhora, reseta o contador
        }
        last_avg_cost = current_avg_cost;

        if stagnation_counter >= config.stagnation_limit {
            println!("\nCritério de estagnação atingido ({} iterações sem melhora no custo médio).", config.stagnation_limit);
            break;
        }
    }

    if evaluations >= config.max_evaluations {
        println!("\nLimite de avaliações ({}) atingido.", config.max_evaluations);
    }

    best_tour
}