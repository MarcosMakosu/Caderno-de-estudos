const TIPO_DE_DADO:i8 = 2;
static mut ESTATICA:i8 = 3;
const PONTO_VIRGULA:i32 = 2; // vezes que eu esqueci ponto é virgula 

    /*  
        0 - erro de escrita
        1 - digitou i31 em vez de i32
        2 - digitou () em vez de {} no println!
        3 - esqueceu do ! no println!, escrevendo apenas println = 4
        4 - esqueceu de por a função na main 
        5 - escreveu com i maiusculo em do menusculo no i32 
        6 - fez a vavivel mas não usou, tipo não escreveu = 3
        7 - digitou printFn em vez de printLn 
        8 - digitou uma variavel com pontos para diefrenciar as casas dos pontos
     */

fn main() {
    println!("---Bloco de Erros------------------------");
    println!("Vezes que esqueceu o ponto e virgula = {}", PONTO_VIRGULA);

    println!("---Começo da main de fato----------------");
    let mut x: i32 = 5;
    x += 6; 
    println!(" mut x, mais 6 = {}", x);
    println!("print do main da global {}", TIPO_DE_DADO);

    unsafe{ 
        ESTATICA = 4; 
        println!("print dentro do unsafe {}", ESTATICA);
        // aqui é unsafe pq to mexendo numa variavel global
    }
    /* ----------------------------- */
    variavel_global();
    shadowing();
    tipos_de_dados();
    metodo();
    exemplo_size();
}

fn variavel_global() {
    println!("Usando a função fora do main");
    println!("Variavel global {}", TIPO_DE_DADO);
    /*Paquita del pollo, usando funções  */
    unsafe{
        println!("Variavel estatica {}", ESTATICA);
        /*toda vez que refwe */
    }
    /*caso tente alterar uma varivel estatica, pode causar erros de da Data Racing ao mexer com multiplas threads, acontencendo de uma thread correndo em cima da outra */
}

fn shadowing() {
    println!("---Shadowing-------------------------------------");

    let sw: i32 = 5;
    println!("Valor de sw é {}, {:p}", sw, &sw);
    let sw: i32 = sw + 1;
    println!("Valor de sw é {}, {:p}", sw, &sw); // binding: usa a propria varival anteriomente para calcular 
    let sw: i32 = sw * 2;
    println!("Valor de sw é {}, {:p}", sw, &sw);

    println!("valor do shadowing é {}", sw);

    /*  
        O Shadowing é o ato de declarar uma nova variável em cima de uma já existente, 
        no entanto a variável não sobrescrita e continua a existir até o fim da aplicação,
        diferentemente do mut, do qual só é possivel alterar o valor de uma variável,
        o shadowing de fato cria uma nova variável podendo alterar o formato/tipo.
            utilidade disso está em Type Casting, Preservação de Imutabilidade, Escopos Temporaios é Coleta de Lixo 
    */
    // exemplo
    let swtm = "a";
    println!("printando swtm {}", swtm);
    let swtm = 32;
    println!("printando o shadowing 2 swtm {}", swtm);

    // teve uso de ponteiro mas vou ver depois, so foi para explicar que mesmo redefinindo o valor a cade momento e botou em um celula de memoria diferente
    // com isso esconde a varivel anterios sem problema de memoria

}

fn tipos_de_dados() {
    println!("--- Tipos de dados -----------------------------");

    // tipos de dado i, numeros negativos

    // i8 vai de -128 até 127
    let oito_bits: i8 = 127;
    
    // i16 vai de -32.768 até 32.767, provavelmente o que você vai usar 
    let dezesseis_bits: i16 = 32767;

    // i32 vai de -2.147.483.648 até 2.147.483.647, pouco provavel
    let trinta_e_dois_bits: i32 = 2147483647;

    // i64 vai de -9.223.372.036.854.775.808 até 9.223.372.036.854.775.807, 9 quintilhões
    let meia_quatro_bits: i64 = 9223372036854775807; 

    // i128 vai de -170.141.183.460.469.231.731.687.303.715.884.105.728 até 170.141.183.460.469.231.731.687.303.715.884.105.727, 170 umdecilhão 
    let umdoisoito_bits: i128 = 170141183460469231731687303715884105727;

    println!("Tipo de dado de numero inteiro i8 = {}", oito_bits);
    println!("Tipo de dado de numero inteiro i16 = {}", dezesseis_bits);
    println!("Tipo de dado de numero inteiro i32 = {}", trinta_e_dois_bits);
    println!("Tipo de dado de numero inteiro i64 = {}", meia_quatro_bits);
    println!("Tipo de dado de numero inteiro i128 = {}", umdoisoito_bits);
    // variaveis não podem começar com numero, problemas na copilação
    
    println!("----------- dado u -------");
    // tipos de dado u, apenas números positivos (sem sinal)

    // u8 vai de 0 até 255
    let oito_bits_u: u8 = 255;

    // u16 vai de 0 até 65.535
    let dezesseis_bits_u: u16 = 65_535;

    // u32 vai de 0 até 4.294.967.295, 4 bilhões
    let trinta_e_dois_bits_u: u32 = 4_294_967_295;

    // u64 vai de 0 até 18.446.744.073.709.551.615, 18 quintilhões
    let meia_quatro_bits_u: u64 = 18_446_744_073_709_551_615;

    // u128 vai de 0 até 340.282.366.920.938.463.463.374.607.431.768.211.455, 340 undecilhões
    let umdoisoito_bits_u: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;

    println!("Tipo de dado de numero inteiro sem sinal u8 = {}", oito_bits_u);
    println!("Tipo de dado de numero inteiro sem sinal u16 = {}", dezesseis_bits_u);
    println!("Tipo de dado de numero inteiro sem sinal u32 = {}", trinta_e_dois_bits_u);
    println!("Tipo de dado de numero inteiro sem sinal u64 = {}", meia_quatro_bits_u);
    println!("Tipo de dado de numero inteiro sem sinal u128 = {}", umdoisoito_bits_u);

    // ao declarar variaveis numericas pode ser usar _ para numeros grandes, o copilador ignora.

    println!("--- Isize & Usize ------------------------------");

    
}



fn metodo() {
    println!("--- Por enquanto nada --------------------------");
}