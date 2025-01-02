#include <stdio.h>

int main() {
    int n, i;

    printf("Enter a number: ");
    scanf("%d", &n);

    i = 1;

    while (i <= n) {
        printf("%d ", i);
        i = i + 1;
    }

    printf("\n");

    return 0;
}

/*
JavaScript Code:

function printNumbers(n) {
    for (let i = 1; i <= n; i++) {
        console.log(i);
    }
}

const n = parseInt(prompt("Enter a number:"));
printNumbers(n);
*/