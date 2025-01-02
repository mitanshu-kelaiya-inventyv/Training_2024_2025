#include <stdio.h>

int main() {
    int n, i, sum = 0;

    printf("Enter a number: ");
    scanf("%d", &n);

    i = 1;

    while (i <= n) {
        sum = sum + i;
        i = i + 1;
    }

    printf("Sum of numbers from 1 to %d is: %d\n", n, sum);

    return 0;
}

/*
JavaScript Code:

function sumOfNumbers(n) {
    let sum = 0;
    for (let i = 1; i <= n; i++) {
        sum += i;
    }
    return sum;
}

const n = parseInt(prompt("Enter a number:"));
const result = sumOfNumbers(n);
console.log("Sum of numbers from 1 to", n, "is:", result);
*/