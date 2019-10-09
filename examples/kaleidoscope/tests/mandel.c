#include <stdio.h>

extern double mandel(double, double, double, double);

void putchard(double x) {
    printf("%c", (char) x);
}

int main(void) {
    mandel(-2.3, -1.3, 0.05, 0.07);
    mandel(-2, -1, 0.02, 0.04);
    mandel(-0.9, -1.4, 0.02, 0.03);
}
