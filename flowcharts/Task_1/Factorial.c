#include<stdio.h>

int main() {
    int n, i = 1, fact = 1;
    
    printf("Enter a number: ");
    scanf("%d", &n);
    
    while(i <= n) {
        fact = fact * i;
        i++;
    }
    
    printf("Factorial of %d is %d", n, fact);

    return 0;
}

/*
JavaScript Code:

function factorial(n) {
    let fact = 1;
    for (let i = 1; i <= n; i++) {
        fact *= i;
    }
    return fact;
}

const n = parseInt(prompt("Enter a number:"));
const result = factorial(n);
console.log("Factorial of", n, "is:", result);
*/