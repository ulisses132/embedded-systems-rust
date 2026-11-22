fn main() {
    // Dia 1: implementa aqui o exercício descrito no README.md.
    let temperaturas = [27.0, 31.0, 29.0, 30.0, 28.0, 27.5, 29.0, 32.0]; // Exemplo de temperatura em Celsius
    let limite_ligar = 30.0; // Limite superior de temperatura
    let limite_desligar = 28.0; // Limite inferior de temperatura
    let mut contador = 0;
    let mut ventoinha_ligada = false;
    let mut estado_anterior = ventoinha_ligada;

    for temperatura in temperaturas {
        contador += 1;

        println!("Ciclo {} - Temperatura: {}°C", contador, temperatura);

        if temperatura > limite_ligar {
            ventoinha_ligada = true;
        } else if temperatura < limite_desligar {
            ventoinha_ligada = false;
        }

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
}
