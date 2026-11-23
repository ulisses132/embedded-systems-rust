struct ControladorTemperatura {
    limite_ligar: f64,
    limite_desligar: f64,
    ventoinha_ligada: bool,
}

fn main() {
    let temperaturas = [27.0, 31.0, 35.0, 31.0, 27.0]; // Exemplo de temperatura em Celsius

    let mut contador_ciclos = 0;

    let mut controlador_a = ControladorTemperatura::new(30.0, 28.0);
    let mut controlador_b = ControladorTemperatura::new(34.0, 32.0);

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

        controlador_a.atualizar_estado_ventoinha(temperatura);
        controlador_b.atualizar_estado_ventoinha(temperatura);

        println!("Ventoinha A ligada: {}", controlador_a.ventoinha_ligada);
        println!("Ventoinha B ligada: {}", controlador_b.ventoinha_ligada);
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

impl ControladorTemperatura {
    fn atualizar_estado_ventoinha(&mut self, temperatura: f64) {
        if temperatura > self.limite_ligar {
            self.ventoinha_ligada = true;
        } else if temperatura < self.limite_desligar {
            self.ventoinha_ligada = false;
        }
    }

    fn new(limite_ligar: f64, limite_desligar: f64) -> Self {
        ControladorTemperatura {
            limite_ligar,
            limite_desligar,
            ventoinha_ligada: false,
        }
    }
}
