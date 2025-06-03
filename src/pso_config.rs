use pso_rs::Config;

pub fn pso_config() -> (Config, Option<fn(f64) -> bool>) {
    let config = Config {
        dimensions: vec![2],
        bounds: vec![(0.149999, 0.1500001), (0.15, 1.0)],
        c1: 2.05,
        c2: 2.05,
        population_size: 100,
        t_max: 1000,
        ..Config::default()
    };

    fn terminate(f_best: f64) -> bool {
        f_best < 1e-6
    }

    (config, Some(terminate))
}
