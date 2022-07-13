#include<stdio.h>
int main() {
    int t;
    scanf("%d", &t);
    for(int i = 0;i < t;i++) {
        int a[2][2];
        int count = 0;
        for(int row = 0;row<2;row++) {
            for(int col = 0;col < 2;col++) {
                scanf("%d", &a[row][col]);
                if (a[row][col] == 1) {
                    count++;
                }
            }
        }
        if(count == 1) {
            printf("1\n");
        }else{
        printf("%d\n", count/2);   
        }
    }
}