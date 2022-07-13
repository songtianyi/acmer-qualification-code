#include <stdio.h>
#include <stdlib.h>
#include <string.h>
int main()
{
    int t = 0;
    scanf("%d", &t);
    for (int i = 0; i < t; i++)
    {
        int n = 0;
        scanf("%d", &n);
        printf("2\n");
        int a[n + 1];
        memset(a, 0, sizeof(a) + 1);
        int res[n + 1];
        int index = 0;
        int next = 1;
        while (1)
        {
            int j = next, isfirst = 1;
            int ok = 1;
            while (j <= n)
            {
                if (a[j] == 0)
                {
                    if (isfirst)
                    {
                        isfirst = 0;
                        next = j;
                    }
                    a[j] = 1;
                    res[index++] = j;
                    j = j * 2;
                    ok = 0;
                }
                else
                {
                    j++;
                }
            }
            if (ok)
            {
                break;
            }
        }
        for (int j = 0; j < n; j++)
        {
            printf("%d ", res[j]);
        }
        printf("\n");
    }
}