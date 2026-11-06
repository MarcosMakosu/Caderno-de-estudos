const TIPO_DE_DADO:i8 = 2;
static mut ESTATICA:i8 = 3;
const PONTO_VIRGULA:i32 = 1; // vezes que eu esqueci ponto é virgula 

    /* 
        1 - digitou i31 em vez de i32
        2 - digitou () em vez de {} no println!
        3 - esqueceu do ! no println!, escrevendo apenas println
        4 - esqueceu de por a função na main 
        5 - escreveu com i maiusculo em do menusculo no i32 
        6 - fez a vavivel mas não usou, tipo não escreveu
        7 - 
     */

fn main() {
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
}

fn variavel_global() {
    println!("Usando a funçãi31
i31
i31o fora do main");
    println!("Variavel global {}", TIPO_DE_DADO);
    /*Paquita del pollo, usando funções  */
    unsafe{
        println!("Variavel estatica {}", ESTATICA);
        /*toda vez que refwe */
    }
    /*caso tente alterar uma varivel estatica, pode causar erros de da Data Racing ao mexer com multiplas threads, acontencendo de uma thread correndo em cima da outra */
}

fn shadowing() {
    let sw: i32 = 5;
    println!("Valor de sw é {}, {:p}", sw, &sw);
    let sw: i32 = sw + 1;
    println!("Valor de sw é {}, {:p}", sw, &sw); // binding: usa a propria varival anteriomente para calcular 
    let sw: i32 = sw * 2;
    println!("Valor de sw é {}, {:p}", sw, &sw);

    println!("valor do shadowing é {}", sw);

    // interessante notar que o mut no foi usado, ou seja a variavel não é mutavel
    // é uma nova variavel em si, mudando até de tipo, por exemplo de string para usize 

    // exemplo
    let swtm = "a";
    println!("printando swtm {}", swtm);
    let swtm = 32;
    println!("printando o shadowing 2 swtm {}", swtm);

    // teve uso de ponteiro mas vou ver depois, so foi para explicar que mesmo redefinindo o valor a cade momento e botou em um celula de memoria diferente
    // com isso esconde a varivel anterios sem problema de memoria

}