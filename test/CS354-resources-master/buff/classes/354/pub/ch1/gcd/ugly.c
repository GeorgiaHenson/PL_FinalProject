// gcc -o gcd ugly.c && ./gcd
#include <stdio.h>
int gcd(int a,int b){while(a!=b){if(a>b)a=a-b;else b=b-a;}return a;}int main(){printf("gcd(15,25)=%d\n",gcd(15,25));return 0;}
