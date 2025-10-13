#include <stdio.h>

void does_not_exist(char name[]) {
	printf("%s\n", name);
}

int main() {
	printf("starting sneklang tools\n");
	does_not_exist("uh oh");
	printf("finished sneklang tools\n");
}
