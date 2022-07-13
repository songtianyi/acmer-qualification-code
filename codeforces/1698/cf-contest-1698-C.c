#include<stdio.h>
int main() {
    int t = 0;
    scanf("%d", &t);
    for(int i = 0;i < t;i++) {
        int n;
        scanf("%d", &n);
        int a[n], sum = 0;
        for(int j = 0;j < n;j++) {
            scanf("%d", &a[j]);
            sum += a[j];
        }
        if (sum % 2 != 0) {
            printf("NO\n");
            continue;
        }
        if (n == 3) {
            int flag = 0;
            for(int j = 0;j < 3;j++) {
                if (a[j] == sum) {
                    flag = 1;
                    break;
                }
            }
            if (flag) {
                printf("YES\n");
            }else {
                printf("NO\n");
            }
        }else {
            int flag = 1;
            for(int j = 3;j < n;j++) {
                if (a[j] == 0 || (a[j] + a[j-1] + a[j-2]) == 0) {
                    continue;
                }else {
                    flag = 0; 
                    break;
                }
            }
            if (flag) {
                printf("YES\n");
            }else {
                printf("NO\n");
            }
        }
    }
}