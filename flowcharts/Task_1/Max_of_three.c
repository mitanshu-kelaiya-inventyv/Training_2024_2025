#include <stdio.h>

int main() {
    int x, y, z;

    printf("Enter three numbers: ");
    scanf("%d %d %d", &x, &y, &z);

    if (x >= y) {
        if (x >= z) {
            printf("Maximum is: %d\n", x);
        } else {
            printf("Maximum is: %d\n", z);
        }
    } else {
        if (z >= y) {
            printf("Maximum is: %d\n", z);
        } else {
            printf("Maximum is: %d\n", y);
        }
    }

    return 0;
}

/*
JS Code:
function findMax(x, y, z) {
    if (x >= y) {
        if (x >= z) {
            return x;
        } else {
            return z;
        }
    } else {
        if (z >= y) {
            return z;
        } else {
            return y;
        }
    }
}

const x = parseInt(prompt("Enter first number:"));
const y = parseInt(prompt("Enter second number:"));
const z = parseInt(prompt("Enter third number:"));

const max = findMax(x, y, z);

console.log("Maximum is:", max);
*/