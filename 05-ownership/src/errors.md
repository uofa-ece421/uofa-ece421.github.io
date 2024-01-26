# The Problems

<img src="img/bjarne-quote.jpg" alt="C++ vs C">

The following C program compiles without errors or warnings
It also (mostly) "works"

```c
#include <stdio.h>
#include <stdlib.h>

typedef struct foo {
    long field;
} Foo;

Foo* start(int n) {
    return (Foo*) malloc(n*sizeof(Foo));
}

void done(Foo *base) {
    free(base);
}

#define N 3

int main(int argc, char **argv) {
    Foo *array = start(N);
    int i=0;
    Foo *p = &array[0];

    while (i++ <= N) {
        (p++)->field = i;
    }

    done(p);

    while (i >= 0) {
        printf("%ld ", array[--i].field);
    }

    return 0;
}
```

