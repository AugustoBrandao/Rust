use std::io; //Standard input/output

//Transformar string em int
//Retorno esperado de uma string de 32 bits
fn convert_to_int(data_input: & String) -> i32{
    let x = data_input.trim().parse::<i32>().unwrap(); //Tirar os espaços da string //transformar em int 32 bits  //desenvelopar - se não for número, seremos avisados
    x
}


fn main(){
    let mut number1 = String::new();
    io::stdin().read_line(&mut number1).expect("Erro ao ler number1");

    let mut number2 = String::new();
    io::stdin().read_line(&mut number2).expect("Erro ao ler number2");
    
    //(&conversão do tipo)
    if convert_to_int(&number1) > convert_to_int(&number2){
        println!("O número {} é maior que {}", number1, number2);
    }else if convert_to_int(&number1) > convert_to_int(&number2){
        println!("O número {} é menor que {}", number1, number2);
    }
}