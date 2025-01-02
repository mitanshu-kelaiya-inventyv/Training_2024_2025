#include <stdio.h>

int main() {
    float radius, perimeter;

    perimeter = 0;
    printf("Enter the radius of the circle: ");
    scanf("%f", &radius);

    perimeter = 2 * 3.14 * radius;
    printf("The perimeter of the circle is: %.2f\n", perimeter);

    return 0;
}

/*
JavaScript Code:
let radius = parseFloat(prompt("Enter the radius of the circle:"));
let perimeter = 2 * 3.14 * radius;
console.log(`The perimeter of the circle is: ${perimeter.toFixed(2)}`);
*/
