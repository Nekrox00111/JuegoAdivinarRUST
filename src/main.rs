use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("¡¡¡Adivine el numero!!!");

    let numero_secreto = rand::thread_rng().gen_range(1..101);

    loop{
        println!("Ingrese su numero: ");
        let mut adivina = String::new();

        io::stdin()
        .read_line(&mut adivina)
        .expect("Fallo al leer linea");
        
        let adivina: u32 = match adivina.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Adivinas: {}",adivina);

        match adivina.cmp(&numero_secreto){
            Ordering::Less => println!("Muy pequeño!"),
            Ordering::Greater => println!("Muy grande!"),
            Ordering::Equal => {
                println!("Felicidades \nGanaste!!!");
                break;
            }
        }
    }
}
