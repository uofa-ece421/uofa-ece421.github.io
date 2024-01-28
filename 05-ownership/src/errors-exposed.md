# The Problems

The following C program compiles without errors or warnings
It also (mostly) "works"

```c
#include <stdio.h>
#include <stdlib.h>

typedef struct foo {
    long field;
} Foo;

Foo* start(int n) {
    // Should check result of malloc
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

    while (i++ <= N) { // access pass the end of the array
        (p++)->field = i;
    }

    done(p); // p doesn't even point to the malloc base
    
    while (i >= 0) {
        // use after "free" AND access before beginning of array
        printf("%ld ", array[--i].field);
    }

    return 0;
}
```

