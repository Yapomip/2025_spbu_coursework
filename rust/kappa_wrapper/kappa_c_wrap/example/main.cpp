 
#include <cstdio>
#include "wrap.h"

#include <iostream>
#include <cstdlib>
#include <string>
#include <fstream>

int main(int argc, char **argv, char **envp) {
    printf("HEELO FROM EXE\n");
    // for (char **env = envp; *env != 0; env++)
    // {
    //     char *thisEnv = *env;
    //     printf("%s\n", thisEnv);    
    // }
    testcall(10);

    const char* p = "/home/yapomip/project/2025_spbu_coursework/rust/kappa_wrapper/kappa_c_wrap/";
    int b = a(p);
    printf("%i\n", b);

    return 0;
}


