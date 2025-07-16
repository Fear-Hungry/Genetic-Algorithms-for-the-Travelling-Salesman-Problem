use std::fs;
use std::io;
use std::path::Path;

#[derive(Clone)]
pub struct Instance {
    pub distances: Vec<Vec<f64>>,
    pub num_cities: usize,
}

impl Instance {
    /// Lê uma instância do problema do caixeiro viajante de um arquivo .tsp (formato TSPLIB).
    /// Suporta apenas o tipo de peso de aresta EUC_2D.
    pub fn from_tsp_file(path: &Path) -> io::Result<Self> {
        let content = fs::read_to_string(path)?;
        let mut lines = content.lines();

        let mut dimension = 0;
        let mut coords: Vec<(f64, f64)> = Vec::new();
        let mut reading_coords = false;

        for line in &mut lines {
            let trimmed_line = line.trim();
            if trimmed_line.starts_with("NODE_COORD_SECTION") {
                reading_coords = true;
                continue;
            } else if trimmed_line.starts_with("EOF") {
                break;
            }

            if reading_coords {
                let parts: Vec<&str> = trimmed_line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let x: f64 = parts[1].parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Coordenada X inválida: {}", e)))?;
                    let y: f64 = parts[2].parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Coordenada Y inválida: {}", e)))?;
                    coords.push((x, y));
                }
            } else if let Some((key, value)) = trimmed_line.split_once(':') {
                match key.trim() {
                    "DIMENSION" => {
                        dimension = value.trim().parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Dimensão inválida: {}", e)))?;
                    }
                    "EDGE_WEIGHT_TYPE" => {
                        if value.trim() != "EUC_2D" {
                            return Err(io::Error::new(io::ErrorKind::Unsupported, "Tipo de aresta não suportado. Apenas EUC_2D é implementado."));
                        }
                    }
                    _ => {} // Ignora outros campos do cabeçalho
                }
            }
        }

        if dimension == 0 || coords.len() != dimension {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Disparidade na dimensão ou dados de coordenadas ausentes."));
        }

        // Calcula a matriz de distâncias euclidianas
        let mut distances = vec![vec![0.0; dimension]; dimension];
        for i in 0..dimension {
            for j in i..dimension {
                let dx = coords[i].0 - coords[j].0;
                let dy = coords[i].1 - coords[j].1;
                let dist = ((dx * dx + dy * dy).sqrt() + 0.5).floor(); // ARREDONDAR!
                distances[i][j] = dist;
                distances[j][i] = dist;
            }
        }

        Ok(Instance {
            distances,
            num_cities: dimension,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Tour {
    pub cities: Vec<usize>,
    pub cost: f64,
}

impl Tour {
    // Calcula o custo total do tour
    pub fn calculate_cost(&mut self, inst: &Instance) {
        if self.cities.len() < 2 {
            self.cost = 0.0;
            return;
        }
        let mut cost = 0.0;
        for window in self.cities.windows(2) {
            let a = window[0];
            let b = window[1];
            cost += inst.distances[a][b];
        }
        if let (Some(&first), Some(&last)) = (self.cities.first(), self.cities.last()) {
            cost += inst.distances[last][first];
        }
        self.cost = cost;
    }
}