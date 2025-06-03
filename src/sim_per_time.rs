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
    let mut t = 0.0;
    let mut tempos = vec![t];
    let mut angulos = vec![delta_n_ini];
    let mut velocidades = vec![delta_w_ini];
    let mut delta_n = delta_n_ini;
    let mut delta_w = delta_w_ini;
    let mut cra = 0.0;
    let mut crr = 0.0;

    let num_steps = (t_max / delta_t) as usize;
    let mut current_pe = pe1;

    // println!("=== Execução da simulação com Runge-Kutta e transições por tempo ===");

    for _i in 0..num_steps {
        if t == 0.0 {
            // println!("CURTO-CIRCUITO em t={:.3}s: Transição Pe1 → Pe2", t);
            current_pe = pe2;
        } else if current_pe == pe2 && t >= tab {
            // println!("Transição Pe2 → Pe3 em t={:.3}s", t);
            cra = delta_n;
            current_pe = pe3;
        } else if current_pe == pe3 && t >= tr {
            // println!("Transição Pe3 → Pe1 em t={:.3}s", t);
            crr = delta_n;
            current_pe = pe1;
        }

        // Atualizando com os últimos valores de delta_w e delta_n
        let (new_delta_w, new_delta_n) =
            runge_kutta_with_d(delta_w, delta_n, current_pe, delta_t, D);
        delta_n = new_delta_n;
        delta_w = new_delta_w;

        // println!(
        //     "t={:.3}s, δn={:.4}° {:.4} rad | Δw={:.4} rad/s, Pe={:.4} sen(δ)",
        //     tempos[i],
        //     angulos[i].to_degrees(),
        //     angulos[i],
        //     velocidades[i],
        //     current_pe
        // );
        t += delta_t;
        tempos.push(t);
        angulos.push(delta_n);
        velocidades.push(delta_w);

        if t >= t_max {
            // println!("Fim da simulação: t_max atingido ({:.3}s)", t_max);
            break;
        }
    }

    (tempos, angulos, velocidades, cra, crr)
}
