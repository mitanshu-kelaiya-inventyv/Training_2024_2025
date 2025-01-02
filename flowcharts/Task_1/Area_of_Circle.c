#include <stdio.h>

int main() {
    float radius, area;

    area = 0;
    printf("Enter the radius of the circle: ");
    scanf("%f", &radius);

    area = 3.14 * radius * radius;
    printf("The area of the circle is: %.2f\n", area);

    return 0;
}


// JavaScript
// let radius = parseFloat(prompt("Enter the radius of the circle:"));
// let area = 3.14 * radius * radius;
// console.log(`The area of the circle is: ${area.toFixed(2)}`);
