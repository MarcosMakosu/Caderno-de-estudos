#include <stdio.h>

int main() {
	int x = 5;    // Linha 1: Armazenado o valor 5 em x
	int *y = &x;
	*y = 10;

	printf("Valores x = %d e y = %d", x, *y);

	return 0;
}