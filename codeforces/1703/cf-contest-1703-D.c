#include <stdio.h>
#include <string.h>
#include <stdlib.h>

char *toLower(char *str)
{
    for (int i = 0; i < strlen(str); i++)
    {
        if (str[i] >= 'A' && str[i] <= 'Z')
        {
            str[i] += 32;
        }
    }
    return str;
}

int equals(char *str1, char *str2)
{
    if (strcmp(str1, str2) == 0)
    {
        return 1;
    }
    return 0;
}

char *concat(char *str1, char *str2)
{
    char *str = malloc(strlen(str1) + strlen(str2) + 1);
    strcpy(str, str1);
    strcat(str, str2);
    return str;
}

struct node {
    int j,k;
};

int main()
{
    int t;
    scanf("%d", &t);
    while (t--)
    {
        int n;
        scanf("%d", &n);
        printf("cat\n");
        char strs[n][18];
        int sums[n];
        int lens[n];
        memset(lens, 0, sizeof(lens));
        memset(sums, 0, sizeof(sums));
        for (int i = 0; i < n; i++)
        {
            scanf("%s", strs[i]);
            printf("%s\n", strs[i]);

            lens[i] = strlen(strs[i]);
            for (int j = 0; j < lens[i]; j++)
            {
                sums[i] += strs[i][j] - '0';
            }

        }
        // struct node twosum[100001][19];
        // memset(twosum, 0, sizeof(twosum));
        //for (int j = 0; j < n; j++)
        //{
        //    for (int k = 0; k < n; k++)
        //    {
        //        int ts = sums[j] + sums[k];
        //        int tsl = lens[j] + lens[k];
        //        if (twosum[ts][tsl] == 0)
        //        {
        //            twosum[ts][tsl]++;
        //            twoj[ts][tsl] = j;
        //            twok[ts][tsl] = k;
        //        }
        //    }
        //}
        //for (int i = 0; i < n; i++)
        //{
        //    int ts = sums[i];
        //    int tsl = lens[i];
        //    if (twosum[ts][tsl] == 0)
        //    {
        //        printf("0");
        //        continue;
        //    }

        //    char *cat = concat(strs[twoj[ts][tsl]], strs[twok[sums[i]][lens[i]]]);
        //    if (equals(cat, strs[i]))
        //    {
        //        printf("1");
        //    }
        //    else
        //    {
        //        printf("0");
        //    }
        //}
        printf("\n");
    }
}