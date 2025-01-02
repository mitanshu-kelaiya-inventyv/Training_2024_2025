// Series: 1, -2, 3, -4, 5, ......

#include <stdio.h>

int main() {
    int n, i = 1, mul = 1;
    scanf("%d", &n);
    while(i <= n) {
        printf("%d\n", i * mul);
        i++;
        mul *= -1;
    }
    return 0;
}


// javascript
// let n = prompt("Enter a number: ");
// let i = 1;
// let mul = 1;

// while(i <= n) {
//     console.log(i * mul);
//     i++;
//     mul *= -1;
// }