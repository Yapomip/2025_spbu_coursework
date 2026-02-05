
#ifndef __WRAP_H__
#define __WRAP_H__

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

#include <stdbool.h>

typedef enum {
    KWC_OK = 0,
    KWC_NOT_OK = 1,
} KCW_Answer;

/*
 * WRAP ABOUT CONTEXT
 */

typedef void* KCW_Context;

/* 
 * Create object to data files
 * defoult files name:
 * particles.yaml
 * interaction.yaml
 * path - path to dirrectore with filesэ
 */
KCW_Answer KCW_CreateContext(const char* path, KCW_Context* result);

/* 
 * Create object to data files
 * path_particles - path to particles file
 * path_interaction - path to interactions file
 */
KCW_Answer KCW_CreateContextFromPaths(const char* path_particles, const char* path_interaction, KCW_Context* result);

void KCW_DestroyContext(KCW_Context context);

/*
 * WRAP ABOUT MOLECULE
 */

typedef void* KCW_Molecula;
typedef void* KCW_MoleculeDistribution;
/* 
 * Constructor for molecula
 * name - name of atom in context files
 * anharmonic_spectrum, rigid_rotator - parametr for kappa molecule
 */
typedef struct KCW_MoleculaBuildParams {
    const char* name;
    bool anharmonic_spectrum;
    bool rigid_rotator;
} KCW_MoleculaBuildParams;

/* Create raw molecula */
KCW_Answer KCW_CreateMolecula(KCW_MoleculaBuildParams molecula_build, const void* context, KCW_Molecula* result);
void KCW_DestroyMolecula(KCW_Molecula molecula);

/*
 * WRAP ABOUT ATOMS
 */

typedef void* KCW_Atom;

/* 
 * Constructor for atom
 * name - name of atom in context files
 */
typedef struct KCW_AtomBuildParams {
    const char* name;
} KCW_AtomBuildParams;
/* Create raw atom */
KCW_Answer KCW_CreateAtom(KCW_AtomBuildParams atom_build, const void* context, KCW_Atom* result);
void KCW_DestroyAtom(KCW_Atom atom);

/* 
 * Energy distribution of atom  
 * energy - energy of atom
 */
typedef struct KCW_AtomDistribution {
    double energy;
} KCW_AtomDistribution;

/*
 * WRAP ABOUT MIXTURE
 */

typedef void* KCW_Mixture;
typedef void* KCW_MixtureDistribution;

/* 
 * Constructor for mixture
 * molecules - array of molecules
 * atoms - array of atoms
 */
KCW_Answer KCW_CreateMixtureFromExists(
    const KCW_Molecula* molecules, unsigned long count_molecules,
    const KCW_Atom* atoms, unsigned long count_atoms,
    const void* context, KCW_Mixture* result
);
/* 
 * Constructor for mixture
 */
KCW_Answer KCW_CreateMixture(
    const KCW_MoleculaBuildParams* molecules, unsigned long count_molecules,
    const KCW_AtomBuildParams* atoms, unsigned long count_atoms,
    const void* context, KCW_Mixture* result
);
void KCW_DestroyMixture(KCW_Mixture mixture);

/* 
* Boltzman distribution for mixture for several values
* count - common size of input arrays 
* n - an array of fractions of elements in a mixture((molecules[0..n], atoms[0..m]) in order of mixture creation)
*/
KCW_Answer KCW_MixtureCreateBoltzmanDistribution(
    KCW_Mixture mixture, unsigned long count, const double* T, const double* pressure, const double* const* n, KCW_MixtureDistribution* result
);

typedef struct KCW_CalculateParams {
    double T;
    double p;
    const double* n;
} KCW_CalculateParams;

KCW_Answer KCW_MixtureCreateBoltzmanDistributionWithCallback(
    KCW_Mixture _mixture, unsigned long count, KCW_CalculateParams (*Next)(void*), void* state, KCW_MixtureDistribution* result
);

/* 
 * Boltzman distribution for mixture
 * n - an array of fractions of elements in a mixture((molecules[0..n], atoms[0..m]) in order of mixture creation)
 */
KCW_Answer KCW_MixtureCreateBoltzmanDistributionFromOne(
    KCW_Mixture mixture, double T, double pressure, const double* n, KCW_MixtureDistribution* result
);

void KCW_DestroyMixtureDistribution(KCW_MixtureDistribution batch);

typedef struct KCW_TransportCoefficient  {
    double thermal_conductivity;
    double shear_viscosity;
    double bulk_viscosity;
} KCW_TransportCoefficient;

typedef struct KCW_TransportCoefficientArray{
    const KCW_TransportCoefficient* data;
    unsigned long size;
} KCW_TransportCoefficientArray;

/* 
 * Compute transport coeficient using kappa::models_omega::model_omega_rs model
 * 
 * null pointer in n_electrons/perturbation if it all is 0.0
 * UB if mixture was destroyed before
 */
KCW_Answer KCW_MixtureComputeTransportCoefficients(
    KCW_MixtureDistribution distribution, const double* n_electrons, const double* perturbation, KCW_TransportCoefficientArray* result
);

/* 
 * Compute transport coeficient using kappa::models_omega::model_omega_rs model
 * 
 * UB if mixture was destroyed before
 */
KCW_Answer KCW_MixtureComputeTransportCoefficientsFromOne(
    KCW_MixtureDistribution distribution, double n_electrons, double perturbation, KCW_TransportCoefficient* result
);


void KCW_DestroyTransportCoefficientArray(KCW_TransportCoefficientArray batch);

#ifdef __cplusplus
}
#endif // __cplusplus

#endif // __WRAP_H__