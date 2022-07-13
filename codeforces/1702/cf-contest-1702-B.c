#include <stdio.h>
#include <stdlib.h>
#include <string.h>
int main()
{
    int t = 0;
    scanf("%d", &t);
    char a[2 * 100000];
    while (t--)
    {
        scanf("%s", a);
        int m[26];
        memset(m, 0, sizeof(m));
        int new_letter = 0;
        int days = 0;
        for (int i = 0; i < strlen(a); i++)
        {
            if (m[a[i] - 'a'] == 0)
            {
                if (new_letter == 3)
                {
                    // one day
                    // printf("new day\n");
                    memset(m, 0, sizeof(m));
                    days++;
                    new_letter = 0;
                }
                new_letter++;
                m[a[i] - 'a'] = 1;
            }
        }
        printf("%d\n", days + (new_letter == 0 ? 0 : 1));
    }
}