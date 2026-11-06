const TIPO_DE_DADO:i8 = 2;
static mut ESTATICA:i8 = 3;

fn main() {
    let mut x: i32 = 5;
    x += 6; 
    println!(" mut x, mais 6 = {}", x);
    println!("print do main da global {}", TIPO_DE_DADO);

    unsafe{ 
        ESTATICA = 4; 
        println!("print dentro do unsafe {}", ESTATICA);
    }
    /* ----------------------------- */
    variavel_global();
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