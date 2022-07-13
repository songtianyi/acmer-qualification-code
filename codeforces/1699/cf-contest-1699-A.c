#include<stdio.h>
#include <stdlib.h>
int main() {
    int t = 0;
    scanf("%d", &t);
    for(int i = 0; i < t;i++) {
        int n;
        scanf("%d", &n);
        int a,b,c;
        int max = n/2,min = n/6;
        int s = rand() % (max-min) + min;
            for(b = min;b <= max;b++) {
                for(c = b;c <= max;c++) {
                    if ((s^b) + (s^c) + (s^c) == n) {
                        printf("%d %d %d\n", a,b,c);
                        goto end;
                    }
                }
            }
        //for(a = max;a >= min;a--) {
        //    for(b = a;b >= min;b--) {
        //        for(c = b;c >= min;c--) {
        //            if ((a^b) + (b^c) + (a^c) == n) {
        //                printf("%d %d %d\n", a,b,c);
        //                goto end;
        //            }
        //        }
        //    }
        //}
        printf("-1\n");
        end:;
    }
}