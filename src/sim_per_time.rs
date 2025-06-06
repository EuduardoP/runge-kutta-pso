use crate::{runge_kutta::runge_kutta_with_d, values::D};

pub fn sim_pet_time(
    pe1: f64,
    pe2: f64,
    pe3: f64,
    tab: f64,
    tr: f64,
    delta_w_ini: f64,
    delta_n_ini: f64,
    t_max: f64,
    delta_t: f64,
) -> (Vec<f64>, Vec<f64>, Vec<f64>, f64, f64) {
    let mut tempos = vec![0.0];
    let mut angulos = vec![delta_n_ini];
    let mut velocidades = vec![delta_w_ini];

    let mut delta_n = delta_n_ini;
    let mut delta_w = delta_w_ini;
    let mut t = 0.0;

    let mut current_pe = pe1;
    let mut transicao_inicial_executada = false;
    let mut transicao_tab_executada = false;
    let mut transicao_tr_executada = false;
    let mut delta_cra = 0.0;
    let mut delta_crr = 0.0;

    // println!("\n=== Execução da simulação com Runge-Kutta e transições por tempo ===");

    loop {
        // Verificações de transição ANTES do cálculo Runge-Kutta
        if t == 0.0 && !transicao_inicial_executada {
            // println!("CURTO-CIRCUITO em t={:.3}s: Transição Pe1 → Pe2", t);
            current_pe = pe2;
            transicao_inicial_executada = true;
        } else if current_pe == pe2 && t >= tab && !transicao_tab_executada {
            // println!("\nTransição Pe2 → Pe3 em t={:.3}s", t);
            // println!(
            //     "δn no momento da transição: {:.4}° -> {:.4}rad",
            //     delta_n.to_degrees(),
            //     delta_n
            // );
            delta_cra = delta_n;
            current_pe = pe3;
            transicao_tab_executada = true;
        } else if current_pe == pe3 && t >= tr && !transicao_tr_executada {
            // println!("\nTransição Pe3 → Pe1 em t={:.3}s", t);
            // println!(
            //     "δn no momento da transição: {:.4}° -> {:.4}rad",
            //     delta_n.to_degrees(),
            //     delta_n
            // );
            delta_crr = delta_n;
            current_pe = pe1;
            transicao_tr_executada = true;
        }

        // Aplicar Runge-Kutta DEPOIS das verificações de transição
        let (new_delta_w, new_delta_n) =
            runge_kutta_with_d(delta_w, delta_n, current_pe, delta_t, D);
        delta_n = new_delta_n;
        delta_w = new_delta_w;

        // Print dos valores atuais (usando índice i para acessar os vetores)
        // println!(
        //     "t={:.3}s, δn={:.4}° {:.4} rad| , Δw={:.4} rad/s, Pe={:.4} sen(δ)",
        //     tempos[i],
        //     angulos[i].to_degrees(),
        //     angulos[i],
        //     velocidades[i],
        //     current_pe
        // );

        // Atualizar tempo e adicionar novos valores aos vetores
        t += delta_t;
        tempos.push(t);
        angulos.push(delta_n);
        velocidades.push(delta_w);

        if t >= t_max {
            break;
        }
    }

    // println!("\n=== Fim da simulação ===");
    (tempos, angulos, velocidades, delta_cra, delta_crr)
}
