#include <stdio.h>
#include <stdlib.h>

int main() {
    printf("1\n100000\n");
    int n = 100000;
    while(n--) {
        int len = 8;
        while(len--) {
            printf("%c", rand() % 26 + 'a');
        }
        printf("\n");
    }
}