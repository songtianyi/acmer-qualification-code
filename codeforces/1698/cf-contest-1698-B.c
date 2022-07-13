#include<stdio.h>

int main() {
    int t = 0;
    scanf("%d", &t);
    int n,k;
    for(int i = 0;i < t;i++) {
        scanf("%d%d", &n,&k);
        int a[n];
        for(int j = 0;j < n;j++) {
            scanf("%d", &a[j]);
        }
        if (k == 1) {
            printf("%d\n", (n-1)/2);
        }else{
            int c = 0;
            for(int j = 1;j < n-1;j++) {
                if (a[j] > (a[j-1] + a[j+1])) {
                    c++;
                }
            }
            printf("%d\n", c);
        }
    }
}