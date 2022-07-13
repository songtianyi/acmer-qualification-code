#include<stdio.h>
#include<string.h>

void toLower(char *str)
{
    for (int i = 0; i < strlen(str); i++)
    {
        if (str[i] >= 'A' && str[i] <= 'Z')
        {
            str[i] += 32;
        }
    }
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
    int t = 0;
    scanf("%d",&t);
    while(t--) {
        char a[4];
        scanf("%s",a);
        toLower(a); 
        if(equals(a,"yes")) {
            printf("Yes\n");
        }else{
            printf("No\n");
        }
        //printf("%d", t);
    }
}