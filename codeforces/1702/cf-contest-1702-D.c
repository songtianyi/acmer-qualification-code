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

void swapc(char *a, char *b)
{
    char t = *a;
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
void sortc(char *a, int n)
{
    for (int i = 0; i < n; i++)
    {
        for (int j = i + 1; j < n; j++)
        {
            if (a[i] > a[j])
            {
                swapc(&a[i], &a[j]);
            }
        }
    }
}

void merge_sort(char *a, int *b, int n)
{
    if (n <= 1)
    {
        return;
    }
    int m = n / 2;
    merge_sort(a, b, m);
    merge_sort(a + m, b + m, n - m);
    char *c = (char *)malloc(sizeof(char) * n);
    int *d = (int *)malloc(sizeof(int) * n);
    for (int i = 0; i < n; i++)
    {
        c[i] = a[i];
        d[i] = b[i];
    }
    int i = 0, j = m, k = 0;
    while (i < m && j < n)
    {
        if (c[i] <= c[j])
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

void merge_sorti(int *a, int n)
{
    if (n <= 1)
    {
        return;
    }
    int m = n / 2;
    merge_sorti(a, m);
    merge_sorti(a + m, n - m);
    int *c = (int *)malloc(sizeof(int) * n);
    for (int i = 0; i < n; i++)
    {
        c[i] = a[i];
    }
    int i = 0, j = m, k = 0;
    while (i < m && j < n)
    {
        if (c[i] <= c[j])
        {
            a[k] = c[i];
            i++;
        }
        else
        {
            a[k] = c[j];
            j++;
        }
        k++;
    }
    while (i < m)
    {
        a[k] = c[i];
        i++;
        k++;
    }
    while (j < n)
    {
        a[k] = c[j];
        j++;
        k++;
    }
    free(c);
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
        char w[2 * 100000];
        scanf("%s", w);
        char *a = (char *)malloc(sizeof(char) * strlen(w));
        strcpy(a, w);
        int p;
        scanf("%d", &p);
        int index[strlen(w)];
        int sum = 0;
        for (int i = 0; i < strlen(w); i++)
        {
            index[i] = i;
            sum += w[i] - 'a' + 1;
        }
        merge_sort(w, index, strlen(w));
        // for(int r = 0;r < strlen(w);r++) {
        //////// printf("%d %ld\n", index[r], routes[r]);
        // printf("%d ", index[r]);
        // }
        // printf("\n");
        // for(int r = 0;r < strlen(w);r++) {
        // printf("%c ", w[r]);
        // }
        // printf("\n");
        int i = 0;
        int c = 0;
        int len = strlen(w);
        int ans[len];
        while (i < len)
        {
            int current = w[i] - 'a' + 1;
            if (c + current > p)
            {
                break;
            }
            c += current;
            ans[i] = index[i];
            i++;
        }
        if (c != 0 && i > 0)
        {
            merge_sorti(ans, i);
            for (int j = 0; j < i; j++)
            {
                printf("%c", a[ans[j]]);
            }
        }
        printf("\n");
    }
}