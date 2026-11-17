# Conceitos de programação
## Binding:
Associação de um **nome** a um **valor**. sería mais corretor afirma que é a associação de um nome a um **lugar**,a declaração de uma **ligação** (que pode ser mutável ou imutável).

Exemplo: Em rust através do Shadowing esse conceito fica presente, pois o **valor** ligado ao **nome** muda, não sendo uma variavel, por meio de uma nova ligação, sendo a primeira ligação inda presente.
```rust 
//rust

let NOME = "a" //aqui foi feito um Binding

let NOME = "b" //aqui foi feito um **novo** Binding
```
O Primeiro Binding entre **NOME** e o valor **a**, Continua existindo e não sendo de fato sobrescrito, por esse motivo o conceito de binding é mais aplicado.

## Ponteiro

Ponteiros são variaveis que guardam o endereço de memoria de outra variavel, elas funcionam apontando **exatamente** o endereço de local exato na memoria ram onde o **dado** está guardado.

Imagine a memória de um computador como uma rua cheia de casas, sendo cada casa um "espaço" da memória e seu ou seus inquilino/s sendo o valor armazenado nessa memória.
O Ponteiro tem como papel informa o número residencial dessa casa, para então fazer proveito de seu valor.

Em linguagens de alto nível tende a não lidar com isso, como o Python e o Lua, em geral fazendo o gerenciamento de memória por conta propria no Interpretador ou copilador, ao inferir uma váriavel com um valor e depois declarar uma váriavel que busque se referir a váriavel anterior, a váriavel posterior acaba por criar uma copia da váriavel anterior e não se referir a ela de verdade, ocorrendo de dois valores identicas(ou não) mas de mesmo intuito coexistirem no processamento ocupando a memória duas vezes.

Exemplo: 
```python
# python

	x = 5  # Linha 1: X começa a armazenar o valor 5
	y = x  # Linha 2: y = 5, pois x é igual a 5
	y = 10 # Linha 3: aqui tentamos alterar o valor 5 para 10 

	print(x, y)
	# Resultado do Print: 5, 10
```
Na linha 1 o X é vinculado ao valor 5, então em algum lugar da memoria ram foi declarado que ali reside o valor 5, seguindo para a linha 2 o y foi vinculado a x, nesse momento y recebe uma copia do valor de x, mas não apontando para x de fato, então quando chegamos na linha 3 aonde y tem seu valor alterado para 10, x se diferencia de y pois y não está vinculado com x, possuindo apenas uma copia que teve seu valor alterado para 10. 


> [!NOTE]
> Dentro do python (e Linguagens modernas de alto nível) por motivos de aperfeiçoadmento da performe do projeto, caso nesse mesmo codigo **não** houvesse a linha 3, ou seja alguma tentativa de alterar o valor de x por meio de outra váriavel, ao ser interpretada, e estaria apontando para o endereço de memoria de x, é não recebendo uma copia do valor, funcionando como um ponteiro.

Enquanto linguagens de alto nível buscam fazer esse gerenciamento pelo programador, linguagens de baixo nível buscam lhe dá controle **absoluto**, não só isso mas como a total **responsabilidade** de administrar a memória, a linguagem C de porgramção é o exemplo mais conhecido disso.

exemplo:
```c
// C
#include <stdio.h>

int main() {
	int x = 5;    // Linha 1: Armazenado o valor 5 em x.
	int *y = &x;  // Linha 2: o "*" tem a função de avisar o copilador que aquela váriavel será um pnteiro, é o & busca saber o endereço de memória de x.
	*y = 10;      // Linha 3: Aqui o "*" ja serve para indicar qual é o ponteiro.

	printf("X = %d e y = %d", x, *y);
	// Resultado do Print: X = 10 e Y = 10

	return 0;
}
```

Como o observado o resultado de x e y é identico, pois de fato houve uma organização a fim de peformar a memória sem consumos desnecessarios, após a linha 1, quando x foi vinculado a 5, a linha 2 faz a vinculação de y deixando **explicito** como um ponteiro de x, fazendo com que y na linha 3 consiga alterar o valor de x para 10 como o demostrado. 

Interessante observa outras linguagens de baixo de nível como Rust, que fazem o uso de ponteiro atraves de uma dinamica intitulada **Referências**, conceito em conformidade com a filosofia do rust de priorizar a segurança de memória, a **Referência** representando pelo **"&"** pega um valor emprestado sem assumir sua posse.

```rust
	// Rust
	fn main() {
    	let mut x = 10; // Linha 1: Cria um vinculo explicitamente mutável de "x" com 10
    	let y = &mut x; // Linha 2: Cria um outro vinculo mutável apontado para "x"
    	*y = 25;        // Linha 3: Desvincula para alterar o valor original
    
    	println!("Novo x: {}", x); 
		// Resultado: execute através do RustPlaygraunds (botão de seta)
	}

```

### O motivo do uso de ponteiros

O uso de ponteiros está relacionado a busca de performace contra gastos desnecessarios, permitindo passar grandes dados como matrizes e estruturas complexas, não criando copias que ocupariam mais memória, e em estruturas de dados avançados, são essecias para a construção de listas encadeadas, árvores e grafos, pois cada elemento necessita saber onde está o proximo da memória. 