use plotters::prelude::*;
use std::f64::consts::PI;
use std::process::Command;

// Função para plotar ângulos e velocidades ao longo do tempo
pub fn plotar_angulos_velocidades(
    tempos: &[f64],
    angulos: &[f64],
    velocidades: &[f64],
    nome_arquivo: &str,
    t_max_plot: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(nome_arquivo, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // Dividir em dois subplots
    let areas = root.split_evenly((2, 1));
    let upper = &areas[0];
    let lower = &areas[1];

    // Plot 1: Ângulos vs Tempo
    {
        let mut chart = ChartBuilder::on(upper)
            .caption("Ângulos vs Tempo", ("sans-serif", 40))
            .margin(10)
            .x_label_area_size(50)
            .y_label_area_size(60)
            .build_cartesian_2d(
                tempos[0]..t_max_plot,
                angulos.iter().fold(f64::INFINITY, |a, &b| a.min(b))
                    ..angulos.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)),
            )?;

        chart
            .configure_mesh()
            .x_desc("Tempo (s)")
            .y_desc("Ângulo (rad)")
            .draw()?;

        chart
            .draw_series(LineSeries::new(
                tempos.iter().zip(angulos.iter()).map(|(t, a)| (*t, *a)),
                &BLUE,
            ))?
            .label("Ângulo")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &BLUE));

        chart.configure_series_labels().draw()?;
    }

    // Plot 2: Velocidades vs Tempo
    {
        let mut chart = ChartBuilder::on(lower)
            .caption("Velocidades vs Tempo", ("sans-serif", 40))
            .margin(10)
            .x_label_area_size(50)
            .y_label_area_size(60)
            .build_cartesian_2d(
                tempos[0]..t_max_plot,
                velocidades.iter().fold(f64::INFINITY, |a, &b| a.min(b))
                    ..velocidades.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)),
            )?;

        chart
            .configure_mesh()
            .x_desc("Tempo (s)")
            .y_desc("Velocidade (rad/s)")
            .draw()?;

        chart
            .draw_series(LineSeries::new(
                tempos.iter().zip(velocidades.iter()).map(|(t, v)| (*t, *v)),
                &RED,
            ))?
            .label("Velocidade")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &RED));

        chart.configure_series_labels().draw()?;
    }

    root.present()?;
    // println!("Gráfico salvo como: {}", nome_arquivo);
    Ok(())
}

// Função para calcular potência elétrica
fn calcular_potencia_eletrica(angulo_rad: f64, pe_max: f64) -> f64 {
    pe_max * angulo_rad.sin()
}

// Função para encontrar interseções entre curva de potência e PM
fn encontrar_intersecoes(pe_max: f64, pm: f64) -> (f64, f64) {
    if pe_max > pm {
        // Primeira interseção (delta_n_ini)
        let delta_n_ini = (pm / pe_max).asin();

        // Segunda interseção (delta_m)
        let delta_m = PI - delta_n_ini;
        (delta_n_ini, delta_m)
    } else {
        (0.0, 0.0)
    }
}

// Função para plotar curva de potência elétrica vs ângulo de potência
pub fn plotar_curva_potencia(
    pe1: f64, // Pe máximo pré-falta
    pe2: f64, // Pe máximo durante falta
    pe3: f64, // Pe máximo pós-falta
    pm: f64,  // Potência mecânica
    cra: f64, // Limite inferior de critério de estabilidade
    crr: f64, // Limite superior de critério de estabilidade
    nome_arquivo: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(nome_arquivo, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // Dados de ângulo
    let num_pontos = 10_000;
    let angulos_rad: Vec<f64> = (0..=num_pontos)
        .map(|i| (i as f64 / num_pontos as f64) * PI)
        .collect();
    let angulos_graus: Vec<f64> = angulos_rad.iter().map(|&a| a.to_degrees()).collect();

    // Cálculo das potências
    let pe1_valores: Vec<f64> = angulos_rad
        .iter()
        .map(|&a| calcular_potencia_eletrica(a, pe1))
        .collect();
    let pe2_valores: Vec<f64> = angulos_rad
        .iter()
        .map(|&a| calcular_potencia_eletrica(a, pe2))
        .collect();
    let pe3_valores: Vec<f64> = angulos_rad
        .iter()
        .map(|&a| calcular_potencia_eletrica(a, pe3))
        .collect();

    // Interseções
    let intersecoes_pe1 = encontrar_intersecoes(pe1, pm);
    let intersecoes_pe3 = encontrar_intersecoes(pe3, pm);

    let delta_n_ini_graus = intersecoes_pe1.0.to_degrees();
    let delta_m_graus = intersecoes_pe1.1.to_degrees();
    let delta_cr_graus = intersecoes_pe3.0.to_degrees();
    let delta_cl_graus = intersecoes_pe3.1.to_degrees();

    // Limites do gráfico
    let max_pe = pe1.max(pe2).max(pe3).max(pm).max(cra).max(crr);
    let y_max = max_pe * 1.2;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Curva de Potência Elétrica x Ângulo de Potência",
            ("sans-serif", 40),
        )
        .margin(10)
        .x_label_area_size(60)
        .y_label_area_size(80)
        .build_cartesian_2d(0.0..180.0, 0.0..y_max)?;

    chart
        .configure_mesh()
        .x_desc("Ângulo de potência (°)")
        .y_desc("Potência elétrica (P)")
        .draw()?;

    // NOVA ÁREA 1: Entre delta_n_ini e CRA, limitada por Pm (cima) e Pe2 (baixo)
    let cra_graus = cra.to_degrees();

    // Criando polígono para área 1 (entre Pm e Pe2)
    let mut area1_poligono: Vec<(f64, f64)> = Vec::new();
    // Parte superior (Pm)
    for (angulo, _pe2_val) in angulos_graus
        .iter()
        .zip(pe2_valores.iter())
        .filter(|(a, _)| **a >= delta_n_ini_graus && **a <= cra_graus)
    {
        area1_poligono.push((*angulo, pm));
    }
    // Parte inferior (Pe2) - em ordem reversa
    for (angulo, pe2_val) in angulos_graus
        .iter()
        .zip(pe2_valores.iter())
        .filter(|(a, _)| **a >= delta_n_ini_graus && **a <= cra_graus)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
    {
        area1_poligono.push((*angulo, *pe2_val));
    }

    if !area1_poligono.is_empty() {
        chart.draw_series(std::iter::once(Polygon::new(area1_poligono, BLUE.mix(0.3))))?;
    }

    // NOVA ÁREA 2: Entre CRA e CRR, limitada por Pm (baixo) e Pe3 (cima)
    let crr_graus = crr.to_degrees();
    let mut area2_poligono: Vec<(f64, f64)> = Vec::new();
    // Parte superior (Pe3)
    for ((angulo, pe3_val), _) in angulos_graus
        .iter()
        .zip(pe3_valores.iter())
        .zip(pe1_valores.iter())
        .filter(|((a, _), _)| **a >= cra_graus && **a <= crr_graus)
    {
        area2_poligono.push((*angulo, *pe3_val));
    }
    // Parte inferior (Pm) - em ordem reversa
    for ((angulo, _pe3_val), _) in angulos_graus
        .iter()
        .zip(pe3_valores.iter())
        .zip(pe1_valores.iter())
        .filter(|((a, _), _)| **a >= cra_graus && **a <= crr_graus)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
    {
        area2_poligono.push((*angulo, pm));
    }

    if !area2_poligono.is_empty() {
        chart.draw_series(std::iter::once(Polygon::new(area2_poligono, BLUE.mix(0.3))))?;
    }

    // NOVA ÁREA 3: Entre CRR e delta_m, limitada por Pe1 (cima) e Pm (baixo) - COR DIFERENTE
    let mut area3_poligono: Vec<(f64, f64)> = Vec::new();
    // Parte superior (Pe1)
    for (angulo, pe1_val) in angulos_graus
        .iter()
        .zip(pe1_valores.iter())
        .filter(|(a, _)| **a >= crr_graus && **a <= delta_m_graus)
    {
        area3_poligono.push((*angulo, *pe1_val));
    }
    // Parte inferior (Pm) - em ordem reversa
    for (angulo, _pe1_val) in angulos_graus
        .iter()
        .zip(pe1_valores.iter())
        .filter(|(a, _)| **a >= crr_graus && **a <= delta_m_graus)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
    {
        area3_poligono.push((*angulo, pm));
    }

    if !area3_poligono.is_empty() {
        chart.draw_series(std::iter::once(Polygon::new(
            area3_poligono,
            MAGENTA.mix(0.3),
        )))?;
    }

    // ÁREA DE ACELERAÇÃO ORIGINAL: verde entre delta_n_ini e delta_cr (mantida)
    if pe1 > pm && pe3 > pm && delta_n_ini_graus < delta_cr_graus {
        let area_acel: Vec<(f64, f64)> = angulos_graus
            .iter()
            .zip(pe3_valores.iter())
            .filter(|(a, _)| **a >= delta_n_ini_graus && **a <= delta_cr_graus)
            .map(|(a, p)| (*a, (*p).max(pm)))
            .chain(std::iter::once((delta_cr_graus, pm)))
            .chain(std::iter::once((delta_n_ini_graus, pm)))
            .collect();

        chart.draw_series(std::iter::once(Polygon::new(area_acel, GREEN.mix(0.3))))?;
    }

    // ÁREA DE DESACELERAÇÃO ORIGINAL: vermelha entre delta_cl e delta_m (mantida)
    if pe1 > pm && pe3 > pm && delta_cl_graus < delta_m_graus {
        let area_desacel: Vec<(f64, f64)> = angulos_graus
            .iter()
            .zip(pe2_valores.iter())
            .filter(|(a, _)| **a >= delta_cl_graus && **a <= delta_m_graus)
            .map(|(a, p)| (*a, pm.min(*p)))
            .chain(std::iter::once((delta_m_graus, pm)))
            .chain(std::iter::once((delta_cl_graus, pm)))
            .collect();

        chart.draw_series(std::iter::once(Polygon::new(area_desacel, RED.mix(0.3))))?;
    }

    // Curvas Pe1, Pe2, Pe3
    chart
        .draw_series(LineSeries::new(
            angulos_graus
                .iter()
                .zip(pe1_valores.iter())
                .map(|(&a, &p)| (a, p)),
            &BLUE,
        ))?
        .label("Pré-falta")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &BLUE));

    chart
        .draw_series(LineSeries::new(
            angulos_graus
                .iter()
                .zip(pe2_valores.iter())
                .map(|(&a, &p)| (a, p)),
            &RED,
        ))?
        .label("Durante falta")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &RED));

    chart
        .draw_series(LineSeries::new(
            angulos_graus
                .iter()
                .zip(pe3_valores.iter())
                .map(|(&a, &p)| (a, p)),
            &GREEN,
        ))?
        .label("Pós-falta")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &GREEN));

    // Linha Pm
    chart
        .draw_series(LineSeries::new(
            vec![(0.0, pm), (180.0, pm)],
            CYAN.stroke_width(2),
        ))?
        .label(format!("Pm = {:.2}", pm))
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &CYAN));

    // Linhas verticais CRA e CRR
    chart.draw_series(std::iter::once(PathElement::new(
        vec![(cra.to_degrees(), 0.0), (cra.to_degrees(), y_max)],
        &BLACK,
    )))?;
    chart.draw_series(std::iter::once(PathElement::new(
        vec![(crr.to_degrees(), 0.0), (crr.to_degrees(), y_max)],
        &BLACK,
    )))?;

    // Legenda
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .draw()?;

    root.present()?;
    // println!("Gráfico salvo como: {}", nome_arquivo);
    Ok(())
}

pub fn abrir_imagem(caminho: &str) {
    #[cfg(target_os = "windows")]
    {
        let _ = Command::new("cmd")
            .args(&["/C", "start", "", caminho])
            .spawn();
    }

    #[cfg(target_os = "linux")]
    {
        let _ = Command::new("xdg-open").arg(caminho).spawn();
    }

    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("open").arg(caminho).spawn();
    }
}
