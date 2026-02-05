 
#include <stdio.h>

#include "wrap.h"

int main(int argc, char **argv) {
    printf("HEELO FROM EXE 1.0.0\n");

    printf("%s\n", KAPPA_RESOURCES_PATH);
    // const char* path = "/home/yapomip/project/2025_spbu_coursework/rust/kappa/data/";
    const char* path = KAPPA_RESOURCES_PATH;
    
    KCW_Context c;
    KCW_Mixture mixture;
    KCW_MixtureDistribution d;
    KCW_TransportCoefficient res;
    
    KCW_CreateContext(path, &c);
    KCW_MoleculaBuildParams m = {"N2", 0, 0};
    KCW_AtomBuildParams a = {"N"};
    KCW_CreateMixture(&m, 1, &a, 1, c, &mixture);

    double T = 5000.0;
    double p = 100000.0;
    double n[] = {0.5, 0.5};
    KCW_MixtureCreateBoltzmanDistributionFromOne(mixture, T, p, n, &d);
    KCW_MixtureComputeTransportCoefficientsFromOne(d, 0, 0, &res);

    printf("res {%f %f %f}\n", res.thermal_conductivity, res.shear_viscosity, res.bulk_viscosity);

    KCW_DestroyMixtureDistribution(d);
    KCW_DestroyMixture(mixture);
    KCW_DestroyContext(c);

    return 0;
}


