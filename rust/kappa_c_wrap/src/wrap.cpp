
#include "wrap.h"

#define ARMA_WARN_LEVEL 0
#include "kappa.hpp"

#include <cstddef>
#include <cstdio>
#include <filesystem>
#include <vector>
#include <tuple>


void testcall(float value)
{
    printf("HELLO FROM LIB\n");

    printf("%f\n",value);
}

 /*
   \file basicTest.cpp
   \brief some basic tests - YAML parsing, linear algebra, utility numeric functions
*/

#include <iostream>
#include <fstream>
#include <string>

#define KAPPA_STRICT_CHECKS
#include <kappa.hpp>

struct measure_data {
  const double d_T = 5.0;
  const double start_T = 5.0;
  const double end_T = 10000.0 + d_T / 2.0;

  const double d_n = 0.40;
  const double start_n = 0.10;
  const double end_n = 0.90 + d_n / 2.0;

  const double pressure_1 = 101325.0;
  const double d_pressure = pressure_1 * (3.0 / 4.0);
  const double start_pressure = pressure_1 * (1.0 / 4.0);
  const double end_pressure = pressure_1 * (7.0 / 4.0) + d_pressure / 2.0;

  const std::string dir_path = "./out/";

  kappa::Mixture& mixture;
  kappa::Molecule& molecule;

  double T = 1;
  double x_N3 = 0;
  double pressure = 1;

  measure_data(kappa::Mixture& mixture, kappa::Molecule& molecule)
    : mixture(mixture), molecule(molecule)
  {}

  std::ofstream open_file(const std::string& file_name) {
    if (!std::filesystem::exists(dir_path)) {
      auto result = std::filesystem::create_directory(dir_path);
      if (!result) {
        std::cout << "FILE " << dir_path + file_name << " DONT OPEN!!!\nTERMINATE!!!";
        std::terminate();
      }
    }
    std::ofstream file_out(dir_path + file_name + ".csv");

    file_out
      << "T;"
      << "pressure;"
      << "atom_n;";
    for (int i = 0; i < 48; ++i)
    {
      file_out << "n" << i << ";";
    }
    file_out
      << "thermal_conductivity;"
      << "shear_viscosity;"
      << "bulk_viscosity" << "\n"
      ;

    return file_out;
  }

  void print_to_file(
    std::ostream& file_out, double T, double pressure,
    std::vector<arma::vec> mol_ndens, arma::vec atom_ndens,
    double thermal_conductivity, double shear_viscosity, double bulk_viscosity
  ) {
    file_out
      << T << ";"
      << pressure << ";"
      << atom_ndens[0] << ";"
      ;
    for (auto n: mol_ndens[0]) {
      file_out << n << ";";
    }
    file_out
      << thermal_conductivity << ";"
      << shear_viscosity << ";"
      << bulk_viscosity << "\n"
      ;
    file_out.flush();
  }

  void do_all_measure() {
    // auto file_global_measure = open_file("all");
    auto& file_global_measure = std::cout;

    std::vector<arma::vec> mol_ndens(2);
    arma::vec atom_ndens(2);
    double tot_ndens = 0;
    double calculated_pressure = 0;
    double thermal_conductivity = 0;
    double shear_viscosity = 0;
    double bulk_viscosity = 0;


    for (double pressure = start_pressure; pressure < end_pressure; pressure += d_pressure) {
      auto file_intermediate_measure = open_file("measure-" + std::to_string(pressure));
      for (double n = start_n; n < end_n; n += d_n) {
        auto file_local_measure = open_file("measure-" + std::to_string(pressure) + "-" + std::to_string(n));

        for (double T = start_T; T < end_T; T += d_T) {
          tot_ndens = pressure / (kappa::K_CONST_K * T); // p T
          mol_ndens[0] = mixture.Boltzmann_distribution(T, n * tot_ndens, molecule); // T n tot_ndens
          atom_ndens[0] = (1 - n) * tot_ndens; // n tot_ndens
          mixture.compute_transport_coefficients(T, mol_ndens, atom_ndens, 0, kappa::models_omega::model_omega_rs, 0.0); // T mol_ndens atom_ndens
          calculated_pressure = mixture.compute_pressure(T, mol_ndens, atom_ndens); // T n
          thermal_conductivity = mixture.get_thermal_conductivity();
          shear_viscosity = mixture.get_shear_viscosity();
          bulk_viscosity = mixture.get_bulk_viscosity();

          // mol_ndens[0] = mixture.compute_density_array(mol_ndens, atom_ndens);
          mol_ndens[0] = mixture.compute_n_molecule(mol_ndens);

          atom_ndens[0] = 1 - n;
          print_to_file(
            file_global_measure, T, calculated_pressure,
            mol_ndens, atom_ndens,
            thermal_conductivity, shear_viscosity, bulk_viscosity
          );
          print_to_file(
            file_intermediate_measure, T, calculated_pressure,
            mol_ndens, atom_ndens,
            thermal_conductivity, shear_viscosity, bulk_viscosity
          );
          print_to_file(
            file_local_measure, T, calculated_pressure,
            mol_ndens, atom_ndens,
            thermal_conductivity, shear_viscosity, bulk_viscosity
          );
        }
      }
      break;
    }

  }
};

int a(const char* path) {
  std::string m_source = path;
  std::string particle_source    = m_source + "particles.yaml";
  std::string interaction_source = m_source + "interaction.yaml";

  std::cout << particle_source << "\n" << interaction_source << "\n";

  try {
    kappa::Molecule molecule("N2", true, true, particle_source);
    kappa::Atom atom("N", particle_source);

    std::cout << "Create elements" << std::endl;

    std::vector<kappa::Molecule> molecules({molecule});
    std::vector<kappa::Atom> atoms({atom});

    std::cout << "Create vector of elements" << std::endl;

    kappa::Mixture mixture(molecules, atoms, interaction_source, particle_source);
    std::cout << "Create mixture" << std::endl;

    measure_data md(mixture, molecule);
    md.do_all_measure();

    std::cout << "End measures" << std::endl;
  } catch(const std::exception& e) {
    std::cout << e.what();
  }
  return 0;
}

#include <fstream>

using context_type = std::pair<std::string, std::string>;

extern "C" {

/* 
 * Create object to data files
 * defoult files name:
 *  particles.yaml
 *  interaction.yaml
 * path - path to dirrectore with files
 */
KCW_Answer KCW_CreateContext(const char* path, KCW_Context* result) {
    std::string m_source = path;
    std::string particle_source    = m_source + "/particles.yaml";
    std::string interaction_source = m_source + "/interaction.yaml";

    *result = reinterpret_cast<KCW_Context>(new context_type(particle_source, interaction_source));
    return KCW_Answer::KWC_OK;
}

/* 
 * Create object to data files
 * path_particles - path to particles file
 * path_interaction - path to interactions file
 */
KCW_Answer KCW_CreateContextFromPaths(const char* path_particles, const char* path_interaction, KCW_Context* result) {
    *result = reinterpret_cast<KCW_Context>(new context_type(path_particles, path_interaction));
    return KCW_Answer::KWC_OK;
}

void KCW_DestroyContext(KCW_Context context) {
    delete reinterpret_cast<const context_type*>(context);
}
/*
 * WRAP ABOUT MIXTURE
 */

using mixture_type = std::tuple<kappa::Mixture, std::vector<kappa::Molecule>, std::vector<kappa::Atom>>;
using mixture_distribution_type = std::tuple<
    mixture_type*, 
    std::vector<
        std::tuple<
            double, // T
            std::vector<arma::vec>, // for molecules
            arma::vec // for atoms
        >
    >
>;

/* 
 * Constructor for mixture
 * molecules - array of molecules
 * atoms - array of atoms
 */
KCW_Answer KCW_CreateMixture(
    const KCW_MoleculaBuildParams* molecules_build, unsigned long count_molecules,
    const KCW_AtomBuildParams* atoms_build, unsigned long count_atoms,
    const void* context, KCW_Mixture* result
) {
    *result = nullptr;
    try {
        const auto& [particle_source, interaction_source] = *reinterpret_cast<const context_type*>(context);
        std::vector<kappa::Molecule> molecules;
        std::vector<kappa::Atom> atoms;

        for (size_t i = 0; i < count_molecules; ++i) {
            molecules.push_back(kappa::Molecule(molecules_build[i].name, 
                molecules_build[i].anharmonic_spectrum, 
                molecules_build[i].rigid_rotator, 
                particle_source));
        }
        for (size_t i = 0; i < count_atoms; ++i) {
            atoms.push_back(kappa::Atom(atoms_build[i].name, particle_source));
        }

        kappa::Mixture mixture(molecules, atoms, interaction_source, interaction_source);
        *result = reinterpret_cast<KCW_Mixture>(
            new mixture_type {
                std::move(mixture), 
                std::move(molecules),
                std::move(atoms)
            }
        );
    } catch(...) {
        return KCW_Answer::KWC_NOT_OK;
    }
    return KCW_Answer::KWC_OK;
}

void KCW_DestroyMixture(KCW_Mixture mixture) {
    delete reinterpret_cast<mixture_type*>(mixture);
}

/* 
 * Energy distribution of mixture  
 * MoleculeDistribution - array of distribution for all molecules in mixture
 * ElectronsDistrinbution - array of distribution for all atoms in mixture
 */
KCW_Answer KCW_MixtureCreateBoltzmanDistribution(
    KCW_Mixture _mixture, unsigned long count, const double* _T, const double* _pressure, const double* const* _n, KCW_MixtureDistribution* result
) {
    *result = nullptr;
    try {
        auto& [mixture, molecules, atoms] = *reinterpret_cast<mixture_type*>(_mixture);
        
        std::vector<
            std::tuple<
                double,
                std::vector<arma::vec>,
                arma::vec
            >
        > batch_result;
        batch_result.reserve(count);

        for (unsigned long  k = 0; k < count; ++k) {
            const double T = _T[k];
            const double pressure = _pressure[k];
            const double* n = _n[k];

            std::vector<arma::vec> mol_ndens(molecules.size());
            arma::vec atom_ndens(atoms.size());
            
            double tot_ndens = pressure / (kappa::K_CONST_K * T); // p T

            int i = 0;
            for (; i < molecules.size(); ++i) {
                mol_ndens[i] = mixture.Boltzmann_distribution(T, n[i] * tot_ndens, molecules[i]);
            }
            for (int j = 0; j < atoms.size(); ++j) {
                atom_ndens[j] = n[i + j] * tot_ndens;
            }

            batch_result.emplace_back(std::tuple{T, std::move(mol_ndens), std::move(atom_ndens)});
        }

        *result = reinterpret_cast<KCW_MixtureDistribution>(
            new mixture_distribution_type {
                reinterpret_cast<mixture_type*>(_mixture), std::move(batch_result)
            }
        );
    } catch(...) {
        return KCW_Answer::KWC_NOT_OK;
    }
    return KCW_Answer::KWC_OK;
}

KCW_Answer KCW_MixtureCreateBoltzmanDistributionWithCallback(
    KCW_Mixture _mixture, unsigned long count, KCW_CalculateParams (*Next)(void*), void* state, KCW_MixtureDistribution* result
) {
    *result = nullptr;
    try {
        auto& [mixture, molecules, atoms] = *reinterpret_cast<mixture_type*>(_mixture);
        
        std::vector<
            std::tuple<
                double,
                std::vector<arma::vec>,
                arma::vec
            >
        > batch_result;
        batch_result.reserve(count);

        for (unsigned long  k = 0; k < count; ++k) {
            KCW_CalculateParams params = Next(state);
            const double T = params.T;
            const double pressure = params.p;
            const double* n = params.n;

            std::vector<arma::vec> mol_ndens(molecules.size());
            arma::vec atom_ndens(atoms.size());
            
            double tot_ndens = pressure / (kappa::K_CONST_K * T); // p T

            int i = 0;
            for (; i < molecules.size(); ++i) {
                mol_ndens[i] = mixture.Boltzmann_distribution(T, n[i] * tot_ndens, molecules[i]);
            }
            for (int j = 0; j < atoms.size(); ++j) {
                atom_ndens[j] = n[i + j] * tot_ndens;
            }

            batch_result.emplace_back(std::tuple{T, std::move(mol_ndens), std::move(atom_ndens)});
        }

        *result = reinterpret_cast<KCW_MixtureDistribution>(
            new mixture_distribution_type {
                reinterpret_cast<mixture_type*>(_mixture), std::move(batch_result)
            }
        );
    } catch(...) {
        return KCW_Answer::KWC_NOT_OK;
    }
    return KCW_Answer::KWC_OK;
}

/* 
 * Boltzman distribution for mixture
 * n - an array of fractions of elements in a mixture((molecules[0..n], atoms[0..m]) in order of mixture creation)
 */
KCW_Answer KCW_MixtureCreateBoltzmanDistributionFromOne(
    KCW_Mixture mixture, double T, double pressure, const double* n, KCW_MixtureDistribution* result
) {
    return KCW_MixtureCreateBoltzmanDistribution(mixture, 1, &T, &pressure, &n, result);
}

void KCW_DestroyMixtureDistribution(KCW_MixtureDistribution distribution) {
    delete reinterpret_cast<mixture_distribution_type*>(distribution);
}

/* 
 * Compute transport coeficient using kappa::models_omega::model_omega_rs model
 * 
 * null pointer in n_electrons/perturbation if it all is 0.0
 * UB if mixture was destroyed before
 */
KCW_Answer KCW_MixtureComputeTransportCoefficients(
    KCW_MixtureDistribution distribution, const double* _n_electrons, const double* _perturbation, KCW_TransportCoefficientArray* result
) {
    KCW_TransportCoefficient* res = nullptr;
    unsigned long i;

    try {
        auto& [_mixture, data_array] = *reinterpret_cast<mixture_distribution_type*>(distribution);
        auto& [mixture, molecules, atoms] = *reinterpret_cast<mixture_type*>(_mixture);

        res = new KCW_TransportCoefficient[data_array.size()];
        i = 0;
        for (auto& [T, mol_ndens, atom_ndens]: data_array) {
            double n_electrons = _n_electrons ? _n_electrons[i] : 0.0;
            double perturbation = _perturbation ? _perturbation[i] : 0.0;
            mixture.compute_transport_coefficients(T, mol_ndens, atom_ndens, n_electrons, kappa::models_omega::model_omega_rs, perturbation);
            
            res[i] = KCW_TransportCoefficient {
                mixture.get_thermal_conductivity(),
                mixture.get_shear_viscosity(),
                mixture.get_bulk_viscosity()
            };
            
            ++i;
        }
    } catch (...) {
        delete[] res;
        return KCW_Answer::KWC_NOT_OK;
    }
    *result = KCW_TransportCoefficientArray {res, i};

    return KCW_Answer::KWC_OK;
}

/* 
 * Compute transport coeficient using kappa::models_omega::model_omega_rs model
 * 
 * UB if mixture was destroyed before
 */
KCW_Answer KCW_MixtureComputeTransportCoefficientsFromOne(
    KCW_MixtureDistribution distribution, double n_electrons, double perturbation, KCW_TransportCoefficient* result
) {
    try {
        auto& [_mixture, data_array] = *reinterpret_cast<mixture_distribution_type*>(distribution);
        auto& [mixture, molecules, atoms] = *reinterpret_cast<mixture_type*>(_mixture);

        auto& [T, mol_ndens, atom_ndens] = data_array[0];
        mixture.compute_transport_coefficients(T, mol_ndens, atom_ndens, n_electrons, kappa::models_omega::model_omega_rs, perturbation);

        *result = KCW_TransportCoefficient {
            mixture.get_thermal_conductivity(),
            mixture.get_shear_viscosity(),
            mixture.get_bulk_viscosity()
        };
    } catch (...) {
        return KCW_Answer::KWC_NOT_OK;
    }
    return KCW_Answer::KWC_OK;
}

void KCW_DestroyTransportCoefficientArray(KCW_TransportCoefficientArray array) {
    delete[] array.data;
}

}