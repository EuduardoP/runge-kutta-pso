use std::f64::consts::PI;

use crate::values::*;
// Implementação do Runge Kutta com termo de amortecimento D
pub fn runge_kutta_with_d(
    delta_w_ini: f64,
    delta_n_ini: f64,
    pe: f64,
    delta_t: f64,
    d: f64,
) -> (f64, f64) {
    let m = H / (PI * F); // Mantendo a inércia como H

    let k1 = delta_w_ini * delta_t;
    let l1 =
        (1.0 / m) * (PM - pe * delta_n_ini.sin()) * delta_t - (d / m) * delta_w_ini * delta_t;

    let k2 = (delta_w_ini + 0.5 * l1) * delta_t;
    let l2 = (1.0 / m) * (PM - pe * (delta_n_ini + 0.5 * k1).sin()) * delta_t
        - (d / m) * (delta_w_ini + 0.5 * l1) * delta_t;

    let k3 = (delta_w_ini + 0.5 * l2) * delta_t;
    let l3 = (1.0 / m) * (PM - pe * (delta_n_ini + 0.5 * k2).sin()) * delta_t
        - (d / m) * (delta_w_ini + 0.5 * l2) * delta_t;

    let k4 = (delta_w_ini + l3) * delta_t;
    let l4 = (1.0 / m) * (PM - pe * (delta_n_ini + k3).sin()) * delta_t
        - (d / m) * (delta_w_ini + l3) * delta_t;

    let delta_w = delta_w_ini + (l1 + 2.0 * l2 + 2.0 * l3 + l4) / 6.0;
    let delta_n = delta_n_ini + (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0;

    (delta_w, delta_n)
}
