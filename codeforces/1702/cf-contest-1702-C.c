#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void swapi(int *a, int *b)
{
    int t = *a;
    *a = *b;
    *b = t;
}

void swapl(long *a, long *b)
{
    long t = *a;
    *a = *b;
    *b = t;
}
void sort(long *a, int *b, int n)
{
    for (int i = 0; i < n; i++)
    {
        for (int j = i + 1; j < n; j++)
        {
            if (a[i] > a[j])
            {
                swapl(&a[i], &a[j]);
                swapi(&b[i], &b[j]);
            }
        }
    }
}
void merge_sort(long *a, int *b, int n)
{
    if (n <= 1)
    {
        return;
    }
    int m = n / 2;
    merge_sort(a, b, m);
    merge_sort(a + m, b + m, n - m);
    long *c = (long *)malloc(sizeof(long) * n);
    int *d = (int *)malloc(sizeof(int) * n);
    for (int i = 0; i < n; i++)
    {
        c[i] = a[i];
        d[i] = b[i];
    }
    int i = 0, j = m, k = 0;
    while (i < m && j < n)
    {
        if (c[i] < c[j])
        {
            a[k] = c[i];
            b[k] = d[i];
            i++;
        }
        else
        {
            a[k] = c[j];
            b[k] = d[j];
            j++;
        }
        k++;
    }
    while (i < m)
    {
        a[k] = c[i];
        b[k] = d[i];
        i++;
        k++;
    }
    while (j < n)
    {
        a[k] = c[j];
        b[k] = d[j];
        j++;
        k++;
    }
    free(c);
    free(d);
}
void bubble_sort(long *a, int *b, int n)
{
    for (int i = 0; i < n; i++)
    {
        for (int j = 0; j < n - i - 1; j++)
        {
            if (a[j] > a[j + 1])
            {
                swapl(&a[j], &a[j + 1]);
                swapi(&b[j], &b[j + 1]);
            }
        }
    }
}

long binary_search(long *a, int n, long x)
{
    int l = 0, r = n - 1;
    while (l <= r)
    {
        int mid = (l + r) / 2;
        if (a[mid] == x)
        {
            return mid;
        }
        else if (a[mid] < x)
        {
            l = mid + 1;
        }
        else
        {
            r = mid - 1;
        }
    }
    return -1;
}

int main()
{
    int t = 0;
    scanf("%d", &t);
    while (t--)
    {
        int n,k;
        scanf("%d%d", &n,&k);
        long routes[n];
        int index[n];
        for(int r = 0;r < n;r++) {
            scanf("%ld", &routes[r]);
            index[r] = r;
        }
        merge_sort(routes, index, n);
        //for(int r = 0;r < n;r++) {
          ////// printf("%d %ld\n", index[r], routes[r]);
          //printf("%d ", index[r]);
        //}
        //printf("\n");
        //for(int r = 0;r < n;r++) {
          //printf("%ld ", routes[r]);
        //}
        //printf("\n");
        int jumpl[n];
        int jumpr[n];
        memset(jumpl, 0, sizeof(jumpl));
        memset(jumpr, 0, sizeof(jumpr));
        int last_ans = 0;
        long last_a = -1, last_b = -1;
        for(int q = 0;q < k;q++) {
            long a,b;
            scanf("%ld%ld", &a,&b);
            if(a == last_a && b == last_b) {
                if(last_ans == 0) {
                    printf("NO\n");
                } else {
                    printf("YES\n");
                }
                continue;
            }
            last_a = a;
            last_b = b;
            last_ans = 0;
            int i = binary_search(routes, n, a);
            if (i == -1) {
                printf("NO\n");
                continue;
            }
            int j = binary_search(routes, n, b);
            if (j == -1) {
                printf("NO\n");
                continue;
            }
            if (index[i] <= index[j]) {
                printf("YES\n");
                last_ans = 1;
                continue;
            }else{
                    int l = i;
                    if(jumpl[i]) {
                        l = jumpl[i];
                    }else{
                        while(routes[l] == a) {
                            if(l == n-1 || routes[l+1] != a) {
                                break;
                            }
                            l++;
                        }
                        jumpl[i] = l;
                    }
                    // so l is the new index
                    int r = j;
                    if(jumpr[j]) {
                        r = jumpr[j];
                    }else{
                        while(routes[r] == b) {
                            if(r == 0 || routes[r-1] != b) {
                                break;
                            }
                            r--;
                        }
                        jumpr[j] = r;
                    }
                    if (index[l] <= index[r]) {
                        printf("YES\n");
                        last_ans = 1;
                    }else{
                        printf("NO\n");
                    }
            }   
        }
       
    }
}