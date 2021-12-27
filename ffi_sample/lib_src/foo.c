#include <stdio.h>

int func_a(int num) {
    printf("This is C func_a! [%d]\n", num);
    return num;
}

const char* func_b(int num) {
    printf("This is C func_b! [%d]\n", num);
    return "C func str";
}
