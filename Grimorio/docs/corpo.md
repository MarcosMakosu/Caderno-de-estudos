# Conceitos de programação
## Binding:
Associação de um **nome** a um **valor**. sería mais corretor afirma que é a associação de um nome a um **lugar**,a declaração de uma **ligação** (que pode ser mutável ou imutável).

Exemplo: Em rust através do Shadowing esse conceito fica presente, pois o **valor** ligado ao **nome** muda, não sendo uma variavel, por meio de uma nova ligação, sendo a primeira ligação inda presente.
```rust 

let NOME = "a" //aqui foi feito um Binding

let NOME = "b" //aqui foi feito um **novo** Binding
```
O Primeiro Binding entre NOME e o valor "a", Continua existindo e não sendo de fato sobrescrito, por esse motivo o conceito de binding é mais aplicado.

## Ponteiro

Ponteiros são variaveis que guardam o endereço de memoria de outra variavel, elas funcionam apontando **exatamente** o endereço de local exato na memoria ram onde o **dado** está guardado.

Exemplo: 
```python
	x = 5 
	y = x # y = 10
	x = 10

	print(x, y)
```
A resposta desse codigo é "15, 10" na primeira linha o X é vinculado ao valor 10, então em algum lugar da memoria ram foi declarado que ali reside o valor 10, seguindo para a segunda linha o y foi vinculado a x, nesse momento y recebe uma copia do valor de x, mas não apontando para x de fato, então quando chegamos na terceira linha a onde x tem seu valor alterado para 10, y não acompanha por ele está com uma "copia" anterior de x.

> [!NOTE]
> Dentro do python por motivos de aperfeiçoadmento da performe do projeto, caso nesse mesmo codigo não houvesse a terceira linha, ao ser interpretada, y estaria apontando para o endereço de memoria de x, é não recebendo uma copia do valor, funcionando como um ponteiro.


# Markdown 

## Citações:
- Uso de avisos:
dá para usar avisos dentro das citações com "[]" e a declaração 
```
> [!NOTE]
> Informação útil que o leitor deve saber, mesmo lendo por cima.

> [!TIP]
> Dica opcional ou atalho para fazer algo de forma mais fácil e rápida.

> [!IMPORTANT]
> Informação crucial que o usuário precisa saber para ter sucesso.

> [!WARNING]
> Conteúdo urgente que exige atenção imediata para evitar problemas.

> [!CAUTION]
> Aviso sobre riscos, perda de dados ou consequências negativas de uma ação.
```
O que gera os avisos:
> [!NOTE]
> Informação útil que o leitor deve saber, mesmo lendo por cima.

> [!TIP]
> Dica opcional ou atalho para fazer algo de forma mais fácil e rápida.

> [!IMPORTANT]
> Informação crucial que o usuário precisa saber para ter sucesso.

> [!WARNING]
> Conteúdo urgente que exige atenção imediata para evitar problemas.

> [!CAUTION]
> Aviso sobre riscos, perda de dados ou consequências negativas de uma ação.

# rust

  

### Shadowing

  

# GIT

  

### Convetional gits