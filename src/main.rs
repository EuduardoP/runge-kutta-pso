mod area;
mod objective_function;
mod plot;
mod runge_kutta;
mod sim_per_time;
mod values;

use area::calcular_areas_melhorado;
use objective_function::objective_function;
use plot::{abrir_imagem, plotar_angulos_velocidades, plotar_curva_potencia};
use pso_rs::{pso::PSO, *};
use sim_per_time::sim_pet_time;
use std::time::Instant;
use values::*;

fn main() {
    let inicio = Instant::now();

    let config = Config {
        dimensions: vec![2],
        bounds: vec![(0.149999, 0.1500001), (0.15, 5.0)],
        c1: 2.05,
        c2: 2.05,
        population_size: 10000,
        t_max: 100000,
        ..Config::default()
    };

    fn terminate(f_best: f64) -> bool {
        f_best < 1e-6
    }

    println!("=== INICIANDO PSO ===");
    let pso: PSO = match pso_rs::run(config, objective_function, Some(terminate)) {
        Ok(pso_result) => pso_result,
        Err(e) => {
            println!("Erro ao executar PSO: {}", e);
            return;
        }
    };

    let model = pso.model;
    println!("Resultado do PSO [tab,tr]:{:?}", model.get_x_best());

    // *** SIMULAÇÃO FINAL ***
    println!("\n=== SIMULAÇÃO FINAL ===");
    let tab = 0.15;
    let tr = 0.3;
    let delta_n_ini = (PM / PE1).asin();
    println!(
        "Parâmetros finais: tab: {:.4}s, tr: {:.4}s, delta_n_ini: {:.6} rad",
        tab, tr, delta_n_ini
    );
    let (tempos_finais, angulos_finais, velocidades_finais, cra_final, crr_final) = sim_pet_time(
        PE1,
        PE2,
        PE3,
        tab,
        tr,
        DELTA_W_INI,
        delta_n_ini,
        T_MAX,
        DELTA_T,
    );

    println!(
        "Resultados da simulação final:\nCRA: {:.4}° | {:.6} rad -> tab: {:.4}s\nCRR: {:.4}° | {:.6} rad -> tr: {:.4}s",
        cra_final.to_degrees(),
        cra_final,
        tab,
        crr_final.to_degrees(),
        crr_final,
        tr
    );

    println!("\n=== MÉTODO DAS ÁREAS IGUAIS ===");
    let (area1, area2, area3) = calcular_areas_melhorado(PE1, PE2, PE3, PM, cra_final, crr_final);
    println!(
        "Áreas calculadas (método melhorado):\nÁrea 1: {:.4}, Área 2: {:.4}, Área 3: {:.4}\n",
        area1, area2, area3
    );
    println!(
        "Diferença das áreas: {:.4}",
        ((area1 + area2) - area3).abs()
    );

    // *** GERAR OS GRÁFICOS ***
    println!("\n=== GERANDO GRÁFICOS ===");

    if let Err(e) = plotar_angulos_velocidades(
        &tempos_finais,
        &angulos_finais,
        &velocidades_finais,
        "out/simulacao_no_tempo.png",
    ) {
        println!("Erro ao plotar ângulos e velocidades: {}", e);
    } else {
        println!("Gráfico de ângulos e velocidades gerado com sucesso!");
        abrir_imagem("out/simulacao_no_tempo.png");
    }

    if let Err(e) =
        plotar_curva_potencia(PE1, PE2, PE3, PM, cra_final, crr_final, "out/potencia.png")
    {
        println!("Erro ao plotar CRA/CRR vs ângulo: {}", e);
    } else {
        println!("Gráfico de CRA/CRR vs ângulo gerado com sucesso!");
        abrir_imagem("out/potencia.png");
    }

    let fim = inicio.elapsed();
    println!("\n=== FIM DA EXECUÇÃO ===");
    println!("Tempo total de execução: {:.2?}", fim);
}
