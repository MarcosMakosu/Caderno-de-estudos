# Lingugens (anotações)

## Rust
 - Shadowing

## Python
 ### - Append:

É um método do python que adiciona um único "item" no final de uma lista.
para usar o método, deve-se usar a seguindo a estrutura **`variavel.append(item)`**

Exemplo: 
```python
# python
V = [0, 1, 2]

V.append(4)

print(V)

# Resultado do codigo: [0, 1, 2, 3, 4]
```

> [!WARNING] 
> **Modificação direta (In-Place):** O Método altera a própria memoria e retorna `none`. **Nunca** faça `V = V.append(valor)` pois isso apaguará sua lista retornando `none`.

Podes também inserir um lista como elemento final.
Exemplo:
```python
# python
V = [0, 1, 2, 3, 4]

V.append([0, 1])

print(V)

# Resultado do codigo: [0, 1, 2, 3, 4, [0, 1]]
```
