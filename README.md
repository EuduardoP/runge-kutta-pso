# Análise de Estabilidade Transitória usando PSO e Método das Áreas Iguais

Este projeto implementa uma análise de estabilidade transitória de sistemas de potência utilizando **Particle Swarm Optimization (PSO)** e o **Método das Áreas Iguais**. O sistema simula o comportamento dinâmico de um gerador síncrono durante distúrbios (faltas) e determina os tempos críticos de abertura e religamento para manter a estabilidade.

## Características Principais

- **Simulação Runge-Kutta**: Integração numérica das equações diferenciais do sistema
- **Otimização PSO**: Determinação automática dos tempos críticos de abertura (`tab`) e religamento (`tr`)
- **Método das Áreas Iguais**: Verificação da estabilidade através do equilíbrio de áreas de aceleração e desaceleração
- **Visualização**: Geração automática de gráficos das simulações e curvas de potência
- **Análise Temporal**: Simulação detalhada do comportamento do sistema ao longo do tempo

## Estrutura do Projeto

```
src/
├── main.rs              # Arquivo principal de execução
├── values.rs            # Constantes e parâmetros do sistema
├── pso_config.rs        # Configurações do algoritmo PSO
├── sim_per_time.rs      # Simulação temporal do sistema
├── runge_kutta.rs       # Implementação do método Runge-Kutta
├── area.rs              # Cálculo do método das áreas iguais
├── objective_function.rs # Função objetivo para o PSO
└── plot.rs              # Funções de plotagem e visualização
```

## Configuração

### 1. Parâmetros do Sistema (`values.rs`)

Modifique as constantes no arquivo `src/values.rs` conforme seu sistema:

```rust
pub const PE1: f64 = 1.83333;  // Potência elétrica máxima pré-falta
pub const PE2: f64 = 1.13491;  // Potência elétrica máxima durante falta
pub const PE3: f64 = 1.22222;  // Potência elétrica máxima abertura mono/bifásica

pub const PM: f64 = 1.44;      // Potência mecânica constante
pub const F: f64 = 60.0;       // Frequência do sistema (Hz)
pub const H: f64 = 9.0;        // Constante de inércia do gerador
pub const D: f64 = 0.0;        // Coeficiente de amortecimento

pub const DELTA_W_INI: f64 = 0.0;  // Velocidade angular inicial
pub const T_MAX: f64 = 5.0;        // Tempo máximo de simulação (s)
pub const DELTA_T: f64 = 0.05;     // Passo de integração (s)
```

### 2. Configurações do PSO (`pso_config.rs`)

Ajuste os parâmetros do algoritmo PSO conforme necessário:

```rust
let config = Config {
    dimensions: vec![2],                           // [tab, tr]
    bounds: vec![(0.149999, 0.1500001), (0.15, 1.0)], // Limites [tab_min, tab_max], [tr_min, tr_max]
    c1: 2.05,                                     // Coeficiente cognitivo
    c2: 2.05,                                     // Coeficiente social
    population_size: 1000,                        // Tamanho da população
    t_max: 5000,                                  // Número máximo de iterações
    ..Config::default()
};
```

**Parâmetros importantes:**
- `bounds`: Define os limites de busca para `tab` (tempo de abertura) e `tr` (tempo de religamento)
- `population_size`: Número de partículas no enxame
- `t_max`: Máximo de iterações do algoritmo
- `c1` e `c2`: Controlam o comportamento exploratório vs. exploitativo

## Instalação e Execução

### Pré-requisitos

- Rust (versão 1.70 ou superior)
- Cargo (gerenciador de pacotes do Rust)

### Dependências

O projeto utiliza as seguintes bibliotecas:
- `pso_rs`: Implementação do algoritmo PSO
- `plotters`: Geração de gráficos

### Execução

1. **Clone o repositório** (se aplicável) ou certifique-se de ter todos os arquivos

2. **Execute o programa** passando o nome da pasta de saída:

```bash
cargo run -- <nome_da_pasta>
```

**Exemplo:**
```bash
cargo run -- teste1
```

Isso criará uma pasta `out/teste1/` com todos os resultados.

3. **Execução sem abertura automática de imagens**:

Para gerar os gráficos sem abri-los automaticamente, use a flag `--no-print`:

```bash
cargo run -- <nome_da_pasta> --no-print
```

**Exemplo:**
```bash
cargo run -- teste1 --no-print
```

Esta opção é útil para:
- Execuções em batch/automáticas
- Servidores sem interface gráfica
- Processamento de múltiplos casos
- Integração com scripts

### Estrutura de Saída

O programa gera os seguintes arquivos na pasta `out/<nome_da_pasta>/`:

- `resultados.txt`: Relatório completo da execução com:
  - Parâmetros otimizados (`tab` e `tr`)
  - Valores de CRA e CRR encontrados
  - Cálculo das áreas pelo método das áreas iguais
  - Tempo total de execução

- `simulacao_no_tempo.png`: Gráfico mostrando:
  - Evolução do ângulo do rotor ao longo do tempo
  - Evolução da velocidade angular ao longo do tempo

- `potencia.png`: Gráfico das curvas de potência mostrando:
  - Curvas Pe1, Pe2 e Pe3 vs. ângulo de potência
  - Linha de potência mecânica constante (Pm)
  - Áreas de aceleração e desaceleração
  - Pontos críticos CRA e CRR

## Funcionamento do Algoritmo

1. **Inicialização**: O PSO inicializa uma população de partículas com valores aleatórios de `tab` e `tr`

2. **Simulação**: Para cada partícula, o sistema simula o comportamento temporal usando Runge-Kutta

3. **Avaliação**: A função objetivo calcula a diferença entre as áreas de aceleração e desaceleração

4. **Otimização**: O PSO minimiza essa diferença até encontrar os tempos ótimos

5. **Resultado**: Os melhores valores de `tab` e `tr` são utilizados para a simulação final

## Interpretação dos Resultados

### Áreas Iguais
- **Área 1 + Área 2**: Energia de aceleração disponível
- **Área 3**: Energia de desaceleração necessária
- **Estabilidade**: Sistema estável quando Área1 + Área2 = Área3

### Pontos Críticos
- **CRA (Critical Relay Angle)**: Ângulo no momento da abertura do religador
- **CRR (Critical Reclosing angle)**: Ângulo no momento do religamento

### Tempos Críticos
- **tab**: Tempo de abertura do disjuntor após início da falta
- **tr**: Tempo total até o religamento (tab + tempo morto)

## Troubleshooting

### Problemas Comuns

1. **PSO não converge**: 
   - Aumente `t_max` ou `population_size`
   - Ajuste os limites (`bounds`) dos parâmetros
   - Verifique se os parâmetros do sistema são fisicamente válidos

2. **Sistema instável**:
   - Verifique se PM < PE_max para todas as condições
   - Ajuste os valores de potência no `values.rs`

3. **Erro na geração de gráficos**:
   - Certifique-se de que a pasta de saída tem permissões de escrita
   - Verifique se as dependências do `plotters` estão instaladas

## Exemplo de Uso

```bash
# Configurar parâmetros em values.rs e pso_config.rs

# Executar simulação (com abertura automática de imagens)
cargo run -- caso_teste_1

# Executar simulação sem abrir imagens
cargo run -- caso_teste_1 --no-print

# Os resultados estarão em:
# - out/caso_teste_1/resultados.txt
# - out/caso_teste_1/simulacao_no_tempo.png  
# - out/caso_teste_1/potencia.png
```

## Contribuição

Para contribuir com o projeto:
1. Faça um fork do repositório
2. Crie uma branch para sua feature
3. Implemente suas modificações
4. Teste thoroughly
5. Submeta um pull request