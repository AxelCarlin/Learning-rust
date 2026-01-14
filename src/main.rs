use std::io::stdin;

fn main() {
    
}

// Funcion que separa una frase y la inserta en un arreglo.
fn split_input(){
    let mut myword = String::new();
    println!("Ingresa una frase: ");
    stdin().read_line(&mut myword).expect("Hay un problema");
    let myword: Vec<&str> = myword.trim().split(' ').collect();
    println!("{:?}", myword);
}