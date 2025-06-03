use std::f64::consts::PI;

pub fn calcular_areas_melhorado(
    pe1: f64, // Pe máximo pré-falta
    pe2: f64, // Pe máximo durante falta
    pe3: f64, // Pe máximo pós-falta
    pm: f64,  // Potência mecânica
    cra: f64, // Limite inferior de critério de estabilidade
    crr: f64, // Limite superior de critério de estabilidade
) -> (f64, f64, f64) {
    // Interseções (em radianos)
    let intersecoes_pe1 = encontrar_intersecoes(pe1, pm);
    let delta_n_ini_rad = intersecoes_pe1.0;
    let delta_m_rad = intersecoes_pe1.1;

    // Usando integração por Simpson ou Trapézio com mais pontos
    let num_pontos = 1_000_000; // Aumentar precisão

    // ÁREA 1: Entre delta_n_ini e CRA, Integrate[pm-pe2,{x,delta_n_ini,cra}]
    let area1 = integrar_simpson(delta_n_ini_rad, cra, num_pontos, |x| {
        pm - calcular_potencia_eletrica(x, pe2)
    });

    // ÁREA 2: Entre CRA e CRR, Integrate[pm-pe3,{x,cra,crr}]
    let area2 = integrar_simpson(cra, crr, num_pontos, |x| {
        pm - calcular_potencia_eletrica(x, pe3)
    });

    // ÁREA 3: Entre CRR e delta_m, Integrate[pe1-pm,{x,crr,delta_m}]
    let area3 = integrar_simpson(crr, delta_m_rad, num_pontos, |x| {
        calcular_potencia_eletrica(x, pe1) - pm
    });

    (area1, area2, area3)
}

// Implementação da integração por Simpson
fn integrar_simpson<F>(a: f64, b: f64, n: usize, f: F) -> f64
where
    F: Fn(f64) -> f64,
{
    if (b - a).abs() < 1e-10 {
        return 0.0;
    }

    let n = if n % 2 == 0 { n } else { n + 1 }; // Garantir que n seja par
    let h = (b - a) / n as f64;

    let mut soma = f(a) + f(b);

    for i in 1..n {
        let x = a + i as f64 * h;
        if i % 2 == 0 {
            soma += 2.0 * f(x);
        } else {
            soma += 4.0 * f(x);
        }
    }

    soma * h / 3.0
}

// Para debug - verificar os valores nos pontos críticos
#[allow(dead_code)]
pub fn debug_valores(pe1: f64, pe2: f64, pe3: f64, pm: f64, cra: f64, crr: f64) {
    let intersecoes = encontrar_intersecoes(pe1, pm);
    let delta_n_ini = intersecoes.0;
    let delta_m = intersecoes.1;

    println!("=== DEBUG VALORES ===");
    println!(
        "delta_n_ini: {:.4} rad ({:.2}°)",
        delta_n_ini,
        delta_n_ini * 180.0 / PI
    );
    println!("CRA: {:.4} rad ({:.2}°)", cra, cra * 180.0 / PI);
    println!("CRR: {:.4} rad ({:.2}°)", crr, crr * 180.0 / PI);
    println!("delta_m: {:.4} rad ({:.2}°)", delta_m, delta_m * 180.0 / PI);

    println!("\n=== VALORES NOS PONTOS CRÍTICOS ===");
    println!("Em delta_n_ini:");
    println!(
        "  Pe2: {:.4}, Pm: {:.4}, Diferença: {:.4}",
        calcular_potencia_eletrica(delta_n_ini, pe2),
        pm,
        pm - calcular_potencia_eletrica(delta_n_ini, pe2)
    );

    println!("Em CRA:");
    println!(
        "  Pe2: {:.4}, Pe3: {:.4}, Pm: {:.4}",
        calcular_potencia_eletrica(cra, pe2),
        calcular_potencia_eletrica(cra, pe3),
        pm
    );

    println!("Em CRR:");
    println!(
        "  Pe1: {:.4}, Pe3: {:.4}, Pm: {:.4}",
        calcular_potencia_eletrica(crr, pe1),
        calcular_potencia_eletrica(crr, pe3),
        pm
    );
}

// Função auxiliar - assumindo que você já tem esta implementada
fn calcular_potencia_eletrica(angulo: f64, pe_max: f64) -> f64 {
    pe_max * angulo.sin()
}

// Função auxiliar - assumindo que você já tem esta implementada
fn encontrar_intersecoes(pe_max: f64, pm: f64) -> (f64, f64) {
    let sin_inv = (pm / pe_max).asin();
    (sin_inv, PI - sin_inv)
}
