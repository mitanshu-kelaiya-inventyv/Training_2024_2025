#include <stdio.h>

int main() {
    int x, y;

    printf("Enter two numbers: ");
    scanf("%d %d", &x, &y);

    if (x > y) {
        printf("The maximum number is: %d\n", x);
    } else {
        printf("The maximum number is: %d\n", y);
    }

    return 0;
}

/*
JavaScript Code:
let x = parseInt(prompt("Enter the first number:"));
let y = parseInt(prompt("Enter the second number:"));

if (x > y) {
    console.log(`The maximum number is: ${x}`);
} else {
    console.log(`The maximum number is: ${y}`);
}
*/
