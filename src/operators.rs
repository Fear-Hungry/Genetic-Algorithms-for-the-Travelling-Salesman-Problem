use crate::tsp::Tour;
use rand::prelude::*;
use std::collections::HashSet;

pub fn edge_recombination_crossover(p1: &Tour, p2: &Tour) -> Tour {
    let n = p1.cities.len();
    if n == 0 {
        return Tour { cities: vec![], cost: 0.0 };
    }

    // 1. Construir o mapa de arestas
    let mut edge_map: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    for parent in [&p1.cities, &p2.cities] {
        for i in 0..n {
            let city = parent[i];
            let right_neighbor = parent[(i + 1) % n];
            let left_neighbor = parent[(i + n - 1) % n];
            edge_map[city].insert(right_neighbor);
            edge_map[city].insert(left_neighbor);
        }
    }

    let mut rng = thread_rng();
    let mut current = if rng.gen_bool(0.5) { p1.cities[0] } else { p2.cities[0] };
    let mut child_cities = Vec::with_capacity(n);
    let mut visited = vec![false; n];

    while child_cities.len() < n {
        child_cities.push(current);
        visited[current] = true;

        // Remover a cidade atual das listas de adjacência
        for neighbors in edge_map.iter_mut() {
            neighbors.remove(&current);
        }

        // 2. Encontrar o próximo candidato
        let next_opt = if !edge_map[current].is_empty() {
            // Heurística: escolher vizinho com a menor lista de adjacências
            edge_map[current]
                .iter()
                .min_by_key(|&&neighbor| edge_map[neighbor].len())
                .cloned()
        } else {
            // Se não houver vizinhos, escolher uma cidade aleatória não visitada
            (0..n).filter(|&i| !visited[i]).choose(&mut rng)
        };

        if let Some(next) = next_opt {
            current = next;
        } else {
            break; // Fim, todas as cidades foram visitadas
        }
    }

    Tour {
        cities: child_cities,
        cost: 0.0, // O custo será calculado depois
    }
}

pub fn insertion_mutation(tour: &mut Tour, mutation_rate: f64) {
    let n = tour.cities.len();
    let mut rng = thread_rng();

    // A mutação só ocorre se o número aleatório for menor que a taxa
    if n > 1 && rng.gen_bool(mutation_rate) {
        // Escolhe uma cidade para mover
        let from_idx = rng.gen_range(0..n);
        let city_to_move = tour.cities.remove(from_idx);

        // Escolhe uma nova posição para inserir (garante que não seja a mesma)
        let mut to_idx = rng.gen_range(0..n);
        if to_idx == from_idx {
            to_idx = (to_idx + 1) % n;
        }
        
        tour.cities.insert(to_idx, city_to_move);
    }
}