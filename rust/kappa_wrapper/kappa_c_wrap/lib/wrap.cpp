
#include "wrap.h"

#include "kappa.hpp"
#include <cstdio>
#include <filesystem>

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
          tot_ndens = pressure / (kappa::K_CONST_K * T);
          mol_ndens[0] = mixture.Boltzmann_distribution(T, n * tot_ndens, molecule);
          atom_ndens[0] = (1 - n) * tot_ndens;
          mixture.compute_transport_coefficients(T, mol_ndens, atom_ndens, 0, kappa::models_omega::model_omega_rs, 0.0);
          calculated_pressure = mixture.compute_pressure(T, mol_ndens, atom_ndens); 
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
