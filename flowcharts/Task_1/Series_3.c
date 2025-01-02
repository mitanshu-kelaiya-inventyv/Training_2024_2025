// Series: 1!, -3!, 5!, -7!, ....

#include <stdio.h>

int main() {
    int n, i = 1;
    long long fact = 1;

    scanf("%d", &n);

    while (i <= n) {
        fact *= i * (i - 1) * (-1);
        i += 2;
    }

    printf("Factorial: %lld", fact);

    return 0;
}
// ```  

// ```javascript
// let readline = require('readline');
// let rl = readline.createInterface({
//     input: process.stdin,
//     output: process.stdout
// });

// rl.question('Enter a number: ', (n) => {
//     n = parseInt(n);
//     let i = 1;
//     let fact = 1;

//     while (i <= n) {
//         fact *= i * (i - 1) * (-1);
//         i += 2;
//     }

//     console.log('Factorial: ' + fact);

//     rl.close();
// });