#include <stdio.h>
#include <stdlib.h>
#include <math.h>

void swap(int *a, int *b) {
	int temp = *a;
	*a = *b;
	*b = temp;
}

int gcd(int a, int b) {
	int temp, flag;
	a = abs(a);
	b = abs(b);
	while (1) {
		if (a < b) swap(&a, &b);
    	if (!b) break;
		a %= b;
	}
	return a;
}

int main() {
	int det, div;
	int a11 = 0, a12 = 1, a21 = 1, a22 = 0;
	int b11, b12, b21, b22;
	printf("逆行列を求めます。\n");
	printf("a11 a12 a21 a22 をスペース区切りで入力してください。\n");
	scanf("%d %d %d %d", &a11, &a12, &a21, &a22);
	printf("\na = [%3d %3d]\n    [%3d %3d]\n\n", a11, a12, a21, a22);
	if (det = a11 * a22 - a12 * a21) {
		div = gcd(det, gcd(a11, gcd(a12, gcd(a21, a22))));
		if (det < 0) div *= -1;
		det /= div;
		b11 = a22 / div;
		b12 = -a12 / div;
		b21 = -a21 / div;
		b22 = a11 / div;
		printf("a^-1 = %2d/%2d[%3d %3d]\n            [%3d %3d]\n", 1, det, b11, b12, b21, b22);
	} else {
		printf("|a| = a11 * a22 - a12 * a21 = 0 だったので計算できません。\n");
	}
}
