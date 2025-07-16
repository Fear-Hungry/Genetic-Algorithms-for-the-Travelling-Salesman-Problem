mod tsp;
mod ga;
mod operators;

use std::path::Path;
use crate::tsp::Instance;
use crate::ga::{GAConfig, run_ga};

fn main() -> std::io::Result<()>{
    let instance = Instance::from_tsp_file(Path::new("data/dj38.tsp"))?;
    
    let config = GAConfig {
        //instance: instance.clone(),
        population_size: 200,
        max_evaluations: 50000,
        selective_pressure: 1.90,
        mutation_rate: 0.01,
        stagnation_limit: 1000
    };

    println!("Iniciando Algoritmo Genético com os seguintes parâmetros:");
    println!(" - População: {}", config.population_size);
    println!(" - Avaliações Máximas: {}", config.max_evaluations);
    println!(" - Taxa de Mutação: {}", config.mutation_rate);
    println!(" - Pressão Seletiva: {}", config.selective_pressure);

    let best_tour = run_ga(&instance, &config);

    println!("\n--- Resultados Finais ---");
    println!("Custo da melhor rota encontrada: {}", best_tour.cost);
    
    Ok(())
}