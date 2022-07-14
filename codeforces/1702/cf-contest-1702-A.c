#include <stdio.h>
#include <stdlib.h>
#include <string.h>
int main()
{
    int t = 0;
    scanf("%d", &t);
    while(t--){
        long m;
        scanf("%ld", &m);
        long a[12] = {1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000, 1000000000, 1000000001};
        int n = sizeof(a) / sizeof(a[0]);
        //if(m == 1000000000) {
        //    printf("0\n");
        //}else{
            for(int i = 0;i < n;i++) {
                if (m < a[i]) {
                    printf("%ld\n", m-a[i-1]);
                    break;
                }
            }
        //}
    }
}