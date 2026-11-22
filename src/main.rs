fn main() {
    // Dia 1: implementa aqui o exercício descrito no README.md.
    let temperaturas = [27.0, 31.0, 150.0, 29.0, -50.0, 27.5, 32.0]; // Exemplo de temperatura em Celsius
    let mut contador_ciclos = 0;
    let mut ventoinha_ligada = false;
    let mut estado_anterior = ventoinha_ligada;
    let mut contador_valido = 0;
    let mut contador_invalido = 0;

    for temperatura in temperaturas {
        contador_ciclos += 1;

        if !temperatura_valida(temperatura) {
            println!(
                "Ciclo {} - Temperatura inválida: {}°C. Ignorando leitura.",
                contador_ciclos, temperatura
            );
            contador_invalido += 1;
            continue; // Ignora a leitura inválida e passa para a próxima iteração
            // podia meter um else e colocar o resto do código dentro do else, mas assim é mais limpo
        }

        contador_valido += 1;

        println!("Ciclo {} - Temperatura: {}°C", contador_ciclos, temperatura);

        ventoinha_ligada = atualizar_estado_ventoinha(temperatura, ventoinha_ligada);

        if ventoinha_ligada != estado_anterior {
            println!(
                "Evento: ventoinha mudou de estado para {}",
                if ventoinha_ligada {
                    "ligada"
                } else {
                    "desligada"
                }
            );
        }
        estado_anterior = ventoinha_ligada;
    }

    println!("Ciclos válidos: {}", contador_valido);
    println!("Ciclos inválidos: {}", contador_invalido);
    println!("Ciclos totais: {}", contador_ciclos);
}

fn temperatura_valida(temperatura: f64) -> bool {
    let limite_inferior = -40.0; // Limite inferior de temperatura
    let limite_superior = 125.0; // Limite superior de temperatura

    temperatura >= limite_inferior && temperatura <= limite_superior
}

fn atualizar_estado_ventoinha(temperatura: f64, ventoinha_ligada: bool) -> bool {
    let limite_ligar = 30.0; // Limite superior para ligar ventoinha
    let limite_desligar = 28.0; // Limite inferior para desligar ventoinha

    if temperatura > limite_ligar {
        true
    } else if temperatura < limite_desligar {
        false
    } else {
        ventoinha_ligada
    }
}
