fn main() {
    //STRING
    let mut name: &str = "Augusto"; //let - dentro do bloco - : &str = tipo string
    
    // INTEIRO
    let x = 23; //i32 - inteiro de 32 bits
    let y: i64 = 230; //inteiro de 64 bits 
    let numeropositivo: u32 = 10; //u - somente números positivos e 32 bits
    
    //FLOAT
    let f: f32 = 7.777; //f32 e f64 - sendo o f64 o padrão

    //BOOLEANO
    let b: bool = true;

    name = "Augusto Brandão";   //mut - torna a variável mutável
    println!("Hello {}!", name);
}