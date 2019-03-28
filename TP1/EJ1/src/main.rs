extern crate rand;

use rand::Rng;

fn main() {
    let mut exitos : i32 = 0;
    let mut tiradas : i32 = 0;

    let mut prob_act : f64 = 0f64;
    let mut prob_anterior : f64 = -1f64;

    /*
        Para inciso a) epsilon: 0.001f64; tiradas_min: 1000000
        Para inciso b) epsilon: 0.0000001f64; tiradas_min: 10000
        Para inciso c) epsilon: 0.0000001f64; tiradas_min 1000:
    */

    while tiradas < 1000 || !converge(prob_anterior, prob_act) {
        let extraccion1 = extraer_sobre();
        let extraccion2 = extraer_sobre();

        // Para probar sin reposicion
        //let extraccion2 = extraer_sobre_sin_reposicion(extraccion1);

        // Inciso a)
        /*if extraccion1 || extraccion2 {
            exitos += 1;
        }*/

        // Inciso b)
        /*if (extraccion1 && !extraccion2) || (!extraccion1 && extraccion2) {
            exitos += 1;
        }*/

        // Inciso c)
        if extraccion1 && extraccion2 {
            exitos += 1;
        }

        tiradas += 1;

        prob_anterior = prob_act;
        prob_act = exitos as f64 / tiradas as f64;
    }

    println!("Probabilidad obtenida: {}", prob_act);
    println!("Probabilidad anterior: {}", prob_anterior);
    println!("Tiradas: {}", tiradas);
}

fn converge(prob_ant : f64, prob_act : f64) -> bool {
    (prob_act - prob_ant).abs() < 0.0000001f64
}

fn extraer_sobre() -> bool {
    // Devuelve verdadero si es facil

    let mut rng = rand::thread_rng();

    let num : f64 = rng.gen();

    num < 3f64 / 5f64
}

fn extraer_sobre_sin_reposicion(extracc_ant : bool) -> bool {
    let mut rng = rand::thread_rng();

    let num : f64 = rng.gen();

    if extracc_ant {
        num < 1f64 / 2f64
    } else {
        num < 3f64 / 4f64
    }
}
