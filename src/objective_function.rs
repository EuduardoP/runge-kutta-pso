use std::f64::consts::PI;

use pso_rs::Particle;

use crate::{area::calcular_areas_melhorado, sim_per_time::sim_pet_time, values::*};

pub fn objective_function(p: &Particle, _flat_dim: usize, _dimensions: &Vec<usize>) -> f64 {
    let tab = p[0];
    let tr = p[1];
    let delta_n_ini = (PM / PE1).asin();

    let (_tempos, _angulos, _velocidades, cra, mut crr) = sim_pet_time(
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

    crr = crr.min(PI);
    let (area1, area2, area3) = calcular_areas_melhorado(PE1, PE2, PE3, PM, cra, crr);

    let erro = ((area1 + area2) - area3).powi(4); // Erro quadrático para otimização fina
    let penalidade = if (area1 + area2) > area3 { 1e10 } else { 1.0 };

    erro * penalidade
}
