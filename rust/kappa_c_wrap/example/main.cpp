 
#include <cstdio>

#include "wrap.h"
#include <iostream>
#include <cstdlib>
#include <string>
#include <fstream>

int main(int argc, char **argv, char **envp) {
    printf("HEELO FROM EXE 1.0.0\n");

    printf("%s\n", KAPPA_RESOURCES_PATH);
    // const char* path = "/home/yapomip/project/2025_spbu_coursework/rust/kappa/data/";
    const char* path = KAPPA_RESOURCES_PATH;
    
    KCW_Context c;
    KCW_Mixture mixture;
    KCW_MixtureDistribution d;
    KCW_TransportCoefficient res;
    
    KCW_CreateContext(path, &c);
    auto m = KCW_MoleculaBuildParams {"N2", 0, 0};
    auto a = KCW_AtomBuildParams {"N"};
    KCW_CreateMixture(&m, 1, &a, 1, c, &mixture);

    auto T = 5000.0;
    auto p = 100000.0;
    double n[] = {0.5, 0.5};
    KCW_MixtureCreateBoltzmanDistributionFromOne(mixture, T, p, n, &d);
    KCW_MixtureComputeTransportCoefficientsFromOne(d, 0, 0, &res);

    printf("res {%f %f %f}\n", res.thermal_conductivity, res.shear_viscosity, res.bulk_viscosity);

    KCW_DestroyMixtureDistribution(d);
    KCW_DestroyMixture(mixture);
    KCW_DestroyContext(c);

    return 0;
}


