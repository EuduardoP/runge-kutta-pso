mod area;
mod objective_function;
mod plot;
mod pso_config;
mod runge_kutta;
mod sim_per_time;
mod values;

use area::calcular_areas_melhorado;
use objective_function::objective_function;
use plot::{abrir_imagem, plotar_angulos_velocidades, plotar_curva_potencia};
use pso_config::pso_config;
use pso_rs::pso::PSO;
use sim_per_time::sim_pet_time;
use std::env;
use std::fs;
use std::io::Write;
use std::time::Instant;
use values::*;

use crate::area::debug_valores;

fn main() {
    let inicio = Instant::now();

    // Capturar argumentos da linha de comando
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Uso: {} <nome_da_pasta>", args[0]);
        eprintln!("Exemplo: cargo run -- a1");
        return;
    }

    let pasta_nome = &args[1];
    let pasta_saida = format!("out/{}", pasta_nome);

    // Criar diretório de saída se não existir
    if let Err(e) = fs::create_dir_all(&pasta_saida) {
        eprintln!("Erro ao criar diretório '{}': {}", pasta_saida, e);
        return;
    }

    println!("Saída será salva em: {}/", pasta_saida);

    // Criar arquivo de resultados
    let caminho_resultado = format!("{}/resultados.txt", pasta_saida);
    let mut arquivo_resultado = match fs::File::create(&caminho_resultado) {
        Ok(file) => file,
        Err(e) => {
            eprintln!(
                "Erro ao criar arquivo de resultados '{}': {}",
                caminho_resultado, e
            );
            return;
        }
    };

    // Função auxiliar para escrever no arquivo e na tela
    let mut escrever = |texto: &str| {
        print!("{}", texto);
        if let Err(e) = write!(arquivo_resultado, "{}", texto) {
            eprintln!("Erro ao escrever no arquivo: {}", e);
        }
    };

    let (config, terminate) = pso_config();

    escrever("=== INICIANDO PSO ===\n");
    let pso: PSO = match pso_rs::run(config, objective_function, terminate) {
        Ok(pso_result) => pso_result,
        Err(e) => {
            let erro_msg = format!("Erro ao executar PSO: {}\n", e);
            escrever(&erro_msg);
            return;
        }
    };

    let model = pso.model;
    let resultado_pso = format!("Resultado do PSO [tab,tr]:{:?}\n", model.get_x_best());
    escrever(&resultado_pso);

    // *** SIMULAÇÃO FINAL ***
    escrever("\n=== SIMULAÇÃO FINAL ===\n");
    let tab = model.get_x_best()[0];
    let tr = model.get_x_best()[1];
    let delta_n_ini = (PM / PE1).asin();
    let parametros_msg = format!(
        "Parâmetros finais: tab: {:.4}s, tr: {:.4}s, delta_n_ini: {:.6} rad -> {:.4}°\n",
        tab,
        tr,
        delta_n_ini,
        delta_n_ini.to_degrees()
    );
    escrever(&parametros_msg);
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

    debug_valores(PE1, PE2, PE3, PM, cra_final, crr_final);
    let resultados_msg = format!(
        "Resultados da simulação final:\nCRA: {:.4}° | {:.6} rad -> tab: {:.4}s\nCRR: {:.4}° | {:.6} rad -> tr: {:.4}s\n",
        cra_final.to_degrees(),
        cra_final,
        tab,
        crr_final.to_degrees(),
        crr_final,
        tr
    );
    escrever(&resultados_msg);

    escrever("\n=== MÉTODO DAS ÁREAS IGUAIS ===\n");
    let (area1, area2, area3) = calcular_areas_melhorado(PE1, PE2, PE3, PM, cra_final, crr_final);
    let areas_msg = format!(
        "Áreas calculadas (método melhorado):\nÁrea 1: {:.4}, Área 2: {:.4}, Área 3: {:.4}\n\n",
        area1, area2, area3
    );
    escrever(&areas_msg);

    let diferenca_msg = format!(
        "Diferença das áreas: {:.4}\n",
        ((area1 + area2) - area3).abs()
    );
    escrever(&diferenca_msg);

    // *** GERAR OS GRÁFICOS ***
    escrever("\n=== GERANDO GRÁFICOS ===\n");

    let caminho_simulacao = format!("{}/simulacao_no_tempo.png", pasta_saida);
    let caminho_potencia = format!("{}/potencia.png", pasta_saida);

    if let Err(e) = plotar_angulos_velocidades(
        &tempos_finais,
        &angulos_finais,
        &velocidades_finais,
        &caminho_simulacao,
    ) {
        let erro_msg = format!("Erro ao plotar ângulos e velocidades: {}\n", e);
        escrever(&erro_msg);
    } else {
        escrever("Gráfico de ângulos e velocidades gerado com sucesso!\n");
        abrir_imagem(&caminho_simulacao);
    }

    if let Err(e) =
        plotar_curva_potencia(PE1, PE2, PE3, PM, cra_final, crr_final, &caminho_potencia)
    {
        let erro_msg = format!("Erro ao plotar CRA/CRR vs ângulo: {}\n", e);
        escrever(&erro_msg);
    } else {
        escrever("Gráfico de CRA/CRR vs ângulo gerado com sucesso!\n");
        abrir_imagem(&caminho_potencia);
    }

    let fim = inicio.elapsed();
    escrever("\n=== FIM DA EXECUÇÃO ===\n");
    let tempo_msg = format!("Tempo total de execução: {:.2?}\n", fim);
    escrever(&tempo_msg);

    // Garantir que tudo seja escrito no arquivo
    if let Err(e) = arquivo_resultado.flush() {
        eprintln!("Erro ao finalizar escrita no arquivo: {}", e);
    } else {
        println!("Resultados salvos em: {}", caminho_resultado);
    }
}
