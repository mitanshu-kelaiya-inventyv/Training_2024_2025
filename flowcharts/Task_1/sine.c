#include <stdio.h>
#include <math.h>

#define PI 3.14159265

int main() {
    int n, i;
    float x, sum = 0, temp = 0;

    printf("Enter the value of x in degrees: ");
    scanf("%f", &x);
    printf("Enter the number of terms (n): ");
    scanf("%d", &n);

    

    x = (x * PI )/ 180; 

    temp = x;
    sum = x;

    for (i = 1; i < n; i++) {
        temp = temp * (-1) * x * x / ((2 * i) * (2 * i + 1));
        sum += temp;
    }

    printf("Sine of x = %.6f\n", sum);

    return 0;
}


// //JavaScript Code:
// function sinTaylorSeries(x, n) {
//   x = (x * Math.PI) / 180; 
//   let sum = x;
//   let temp = x;

//   for (let i = 1; i < n; i++) {
//     temp = temp * (-1) * x * x / ((2 * i) * (2 * i + 1));
//     sum += temp;
//   }

//   return sum;
// }

// const degrees = 45;
// const terms = 10; 
// const result = sinTaylorSeries(degrees, terms);
// console.log(`Sine of ${degrees} degrees (using ${terms} terms): ${result}`);