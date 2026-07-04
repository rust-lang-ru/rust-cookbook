#include "mylibrary.h"

#include <iostream>

extern "C" {

int add_numbers(int a, int b) {
    return a + b;
}

void greet(const char* name) {
    std::cout << "Hello from a C++ shared library, " << name << "!" << std::endl;
}

}  // extern "C"
