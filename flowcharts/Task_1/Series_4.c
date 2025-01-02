// Series: 1,1,2,1,2,3 ....

#include <stdio.h>

int main() {
    int rows, i, j;
    
    printf("Enter the number of rows: ");
    scanf("%d", &rows);
    
    for (i = 1; i <= rows; i++) {
        for (j = 1; j <= i; j++) {
            printf("%d ", j);
        }
    }
    
    return 0;
}
// ```

// ```javascript
// let rows = parseInt(prompt("Enter the number of rows:"));

// for (let i = 1; i <= rows; i++) {
//     let rowOutput = "";
//     for (let j = 1; j <= i; j++) {
//         rowOutput += j + " ";
//     }
//     console.log(rowOutput);
// }