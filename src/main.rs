fn main() {
    // Dia 1: implementa aqui o exercício descrito no README.md.
    let temperaturas = [22.0, 28.0, 31.0, 35.0, 30.0, 26.0]; // Exemplo de temperatura em Celsius
    let limite = 30.0; // Limite superior de temperatura
    let mut contador = 0;

    for temperatura in temperaturas {
        let ventoinha_ligada = temperatura > limite;
        contador += 1;
        println!("Ciclo {} - Temperatura: {}°C", contador, temperatura);
        if ventoinha_ligada {
            println!("Ventoinha ligada ");
        } else {
            println!("Ventoinha desligada");
        }
    }
}
