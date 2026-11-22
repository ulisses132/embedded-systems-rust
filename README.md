# Embedded Systems Rust

Projeto de aprendizagem prática: simular um microcontrolador que lê um sensor de temperatura, controla uma ventoinha e regista o seu comportamento.

A simulação corre no computador, sem hardware ou dependências externas. Não emula um processador real nem constitui firmware para uma placa. Por agora, o programa apenas apresenta uma mensagem: o exercício está por implementar.

## Começar

Abre esta pasta no VS Code. A extensão `rust-analyzer` está recomendada no projeto e pode ser instalada pelo painel de extensões.

No terminal integrado, dentro desta pasta:

```sh
cargo run
```

Comandos úteis:

```sh
cargo check       # Verificar se o código compila
cargo fmt         # Formatar o código
cargo clippy      # Pedir sugestões ao analisador Rust
```

## Estrutura

- `Cargo.toml`: configuração do projeto Rust.
- `Cargo.lock`: versões fixadas pelo Cargo; deve acompanhar o Git neste projeto.
- `src/main.rs`: ponto de entrada e local de trabalho do Dia 1.
- `.gitignore`: exclui os ficheiros gerados da compilação.
- `.vscode/extensions.json`: recomendação de apoio a Rust no editor.

## Dia 1 — Sensor e controlador de temperatura

**Objetivo:** compreender o ciclo entrada → decisão → saída, usando variáveis, condições e repetição em Rust.

Neste exercício, o sensor fornece temperaturas em graus Celsius, o controlador decide o que fazer e a ventoinha é representada por um estado ligado/desligado. A saída no terminal serve de registo de cada ciclo.

### Desafio (40–60 minutos)

1. Representa uma leitura de temperatura em Rust e apresenta-a com a unidade °C.
2. Define um limite de 30 °C e um estado para a ventoinha.
3. Faz a ventoinha ligar quando a temperatura for superior a 30 °C. Até esse limite, inclusive, deve ficar desligada.
4. Simula várias leituras, numa sequência fixa: 22, 28, 31, 35, 30 e 26 °C. Não precisas de números aleatórios.
5. Para cada leitura, apresenta o número do ciclo, a temperatura e o estado da ventoinha.
6. Termina o programa depois de processar todas as leituras.

### Como verificar o teu trabalho

- [ ] O programa compila e executa com `cargo run`.
- [ ] Cada leitura gera um registo compreensível.
- [ ] A ventoinha está desligada a 22, 28, 30 e 26 °C.
- [ ] A ventoinha está ligada a 31 e 35 °C.
- [ ] O estado é atualizado quando a temperatura volta a descer.
- [ ] Consegues explicar a escolha dos tipos e a condição usada.

### Perguntas para a revisão

- O que representa a entrada, o controlador e a saída nesta simulação?
- Que diferença existe entre uma leitura simulada e uma leitura de um sensor físico?
- O que poderá acontecer se a temperatura oscilar constantemente junto do limite?

Não é necessário resolver já oscilações, erros de sensor, comunicação ou concorrência. Esses temas ficam para os próximos dias.

## Evolução prevista

- Dias 1–3: sensores simulados e conceitos de entradas e saídas.
- Dias 4–6: estados, tratamento de erros e organização do controlador.
- Dias 7–9: comunicação simulada, logs e testes.
- Dias 10–12: análise de logs em Python e experiências com IA.
- Dias 13–14: validação e apresentação do protótipo.

O ritmo pode ser ajustado ao tempo disponível. Implementa primeiro a tua tentativa do Dia 1 e revê as decisões antes de avançar.
