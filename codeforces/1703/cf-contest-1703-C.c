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
        int wheels;
        scanf("%d", &wheels);
        int final[wheels];
        for(int i = 0; i < wheels; i++) {
            scanf("%d", &final[i]);
        }
        int m = 0;
        while(m<wheels) {

            int moves = 0;
            scanf("%d", &moves);
            char ud[moves+1];
            scanf("%s", ud);
            for(int i = 0; i < moves; i++) {
                if(ud[i] == 'U') {
                    final[m] = (final[m]-1 +10)%10;
                }else {
                    final[m] = (final[m]+1)%10;
                }
            }
            m++;
        }
        for(int i = 0; i < wheels; i++) {
            printf("%d ", final[i]);
        }
        printf("\n");
    }
}