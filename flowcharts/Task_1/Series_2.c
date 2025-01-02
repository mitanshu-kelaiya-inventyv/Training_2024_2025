// Series: 1!, 3!, 5!, ......

#include<stdio.h>

int main() {
    int n, i = 1;
    long fact = 1;
    
    printf("Enter a number: ");
    scanf("%d", &n);
    
    while(i <= n) {
        fact = fact * i * (i+1);
        i += 2;
    }
    
    printf("Factorial is: %ld", fact);
    
    return 0;
}
// ```

// ```javascript
// let readlineSync = require('readline-sync');

// let n = parseInt(readlineSync.question("Enter a number: "));
// let i = 1;
// let fact = 1;

// while (i <= n) {
//     fact = fact * i * (i+1);
//     i += 2;
// }

// console.log("Factorial is: " + fact);