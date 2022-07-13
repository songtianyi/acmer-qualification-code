#include<stdio.h>
#include<string.h>

char* toLower(char *str)
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

int main() {
    int t;
    scanf("%d",&t);
    while(t--) {
        int n;
        scanf("%d", &n);
        char a[n+1];
        int m[27];
        memset(m, 0, sizeof(m));
        scanf("%s",a);
        int c = 0;
        for(int i = 0; i < n; i++) {
            int v = a[i] - 'A';
            if (m[v] == 0) {
                c+=2;
                m[v]++;
            }else {
                c+=1;
            }
        }
        printf("%d\n", c);
    }
}