// Series: 1,1,2,1,1,2,3,2,1 ....

#include <stdio.h>

void generate_series(int rows) {
    for (int i = 1; i <= rows; i++) {
        for (int j = 1; j <= i * 2 - 1; j++) {
            if (j <= i) {
                printf("%d ", j);
            } else {
                printf("%d ", i * 2 - j);
            }
        }
        //printf("\n");
    }
}

int main() {
    int rows;
    printf("Enter the number of rows: ");
    scanf("%d", &rows);
    generate_series(rows);
    return 0;
}


// function generateSeries(rows) {
//   for (let i = 1; i <= rows; i++) {
//     let row = "";
//     for (let j = 1; j <= i * 2 - 1; j++) {
//       if (j <= i) {
//         row += j + " ";
//       } else {
//         row += (i * 2 - j) + " ";
//       }
//     }
//     console.log(row);
//   }
// }

// const rows = 5; // Change this to the desired number of rows
// generateSeries(rows);