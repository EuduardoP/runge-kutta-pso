use pso_rs::Config;

pub fn pso_config() -> (Config, Option<fn(f64) -> bool>) {
    let config = Config {
        dimensions: vec![2],                             // [tab, tr]
        bounds: vec![(0.15, 0.15 + 1e-10), (0.15, 5.0)], // Limites [tab_min, tab_max], [tr_min, tr_max]
        c1: 2.05,                                        // Coeficiente cognitivo
        c2: 2.05,                                        // Coeficiente social
        population_size: 1000,                           // Tamanho da população
        ..Config::default()
    };

    fn terminate(f_best: f64) -> bool {
        f_best < 1e-4 // Critério de parada: o método para se função objetivo é menor que 1e-5
    }

    (config, Some(terminate))
}
