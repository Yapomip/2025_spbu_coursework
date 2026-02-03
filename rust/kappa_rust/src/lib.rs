#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[warn(unused_imports)]
use std::ffi::CString;
use std::marker::PhantomData;
use std::path::Path;
use std::slice;

use kappa_c_wrap::root::*;

extern crate link_cplusplus;

mod kappa_c_wrap {
    #![allow(dead_code)]
    include!("./hellomod.rs");
}

const SOMETHING_WRONG: &str = "Something go wrong";

pub struct Context {
    context: KCW_Context,
}

impl Default for Context {
    fn default() -> Self {
        Context::from_path(std::env!("KAPPA_RESOURCES_PATH")).unwrap()
    }
}

impl Context {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Context, &'static str> {
        let mut context: KCW_Context = std::ptr::null_mut::<std::os::raw::c_void>();
        let path = CString::new(path.as_ref().to_str().unwrap()).unwrap();
        let result;
        unsafe {
            result = KCW_CreateContext(path.as_ptr(), &mut context as *mut KCW_Context);
        }
        match result {
            KCW_Answer_KWC_OK => Ok(Context { context }),
            _ => Err(SOMETHING_WRONG),
        }
    }
    pub fn build_mixture<'a>(&'a self) -> MixtureBuilder<'a> {
        MixtureBuilder {
            context: self,
            molecula_build_params: Vec::new(),
            atom_build_params: Vec::new(),
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            KCW_DestroyContext(self.context);
        }
    }
}

pub struct MixtureBuilder<'a> {
    context: &'a Context,
    molecula_build_params: Vec<(String, bool, bool)>,
    atom_build_params: Vec<String>,
}

impl<'a> MixtureBuilder<'a> {
    pub fn add_molecula<S: ToString>(
        &'a mut self,
        name: S,
        anharmonic_spectrum: bool,
        rigid_rotator: bool,
    ) -> &'a mut Self {
        self.molecula_build_params
            .push((name.to_string(), anharmonic_spectrum, rigid_rotator));
        self
    }
    pub fn add_atom<S: ToString>(&'a mut self, name: S) -> &'a mut Self {
        self.atom_build_params.push(name.to_string());
        self
    }
    pub fn build(&self) -> Result<Mixture, &'static str> {
        let mut molecula_build_params =
            Vec::<KCW_MoleculaBuildParams>::with_capacity(self.molecula_build_params.len());
        let mut atom_build_params =
            Vec::<KCW_AtomBuildParams>::with_capacity(self.atom_build_params.len());
        let mut all_c_stings = Vec::<CString>::with_capacity(
            molecula_build_params.capacity() + atom_build_params.capacity(),
        );

        for b in &self.molecula_build_params {
            all_c_stings.push(CString::new(b.0.clone()).unwrap());
            molecula_build_params.push(KCW_MoleculaBuildParams {
                name: all_c_stings.last().unwrap().as_ptr(),
                anharmonic_spectrum: b.1,
                rigid_rotator: b.2,
            });
        }

        for b in &self.atom_build_params {
            all_c_stings.push(CString::new(b.clone()).unwrap());
            atom_build_params.push(KCW_AtomBuildParams {
                name: all_c_stings.last().unwrap().as_ptr(),
            });
        }

        let mut mixture: KCW_Mixture = std::ptr::null_mut::<std::os::raw::c_void>();
        let result;
        unsafe {
            result = KCW_CreateMixture(
                molecula_build_params.iter().as_ref().as_ptr(),
                molecula_build_params.len() as u64,
                atom_build_params.iter().as_ref().as_ptr(),
                atom_build_params.len() as u64,
                self.context.context,
                &mut mixture as *mut KCW_Mixture,
            );
        }
        match result {
            KCW_Answer_KWC_OK => Ok(Mixture {
                mixture,
                components_count: all_c_stings.len(),
            }),
            _ => Err(SOMETHING_WRONG),
        }
    }
}

pub struct Mixture {
    mixture: KCW_Mixture,
    components_count: usize,
}

pub type CalculateParams = kappa_c_wrap::root::KCW_CalculateParams;

impl Mixture {
    /// if len size difficults all cuts by min
    pub fn bozman_distribution<'a>(
        &'a self,
        T: &[f64],
        p: &[f64],
        n: &[&[f64]],
    ) -> Result<MixtureDistribution<'a>, &'static str> {
        let count = usize::min(T.len(), usize::min(p.len(), n.len()));
        let mut all_n = Vec::new();
        for n_i in n {
            if n_i.len() != self.components_count {
                return Err(SOMETHING_WRONG);
            }
            all_n.push(n_i.as_ptr());
        }
        let mut distribution: KCW_MixtureDistribution =
            std::ptr::null_mut::<std::os::raw::c_void>();
        let result;
        unsafe {
            result = KCW_MixtureCreateBoltzmanDistribution(
                self.mixture,
                count as u64,
                T.as_ptr(),
                p.as_ptr(),
                all_n.as_ptr(),
                &mut distribution as *mut KCW_MixtureDistribution,
            )
        }

        match result {
            KCW_Answer_KWC_OK => Ok(MixtureDistribution {
                mixture: PhantomData,
                count,
                distribution,
            }),
            _ => Err(SOMETHING_WRONG),
        }
    }

    fn bozman_distribution_with_function_callback_inner<'a>(
        &'a self,
        count: usize,
        next: extern "C" fn(*mut std::os::raw::c_void) -> CalculateParams,
        state: *mut std::os::raw::c_void,
    ) -> Result<MixtureDistribution<'a>, &'static str> {
        let mut distribution: KCW_MixtureDistribution =
            std::ptr::null_mut::<std::os::raw::c_void>();
        let result;
        unsafe {
            result = KCW_MixtureCreateBoltzmanDistributionWithCallback(
                self.mixture,
                count as u64,
                Some(next),
                state,
                &mut distribution as *mut KCW_MixtureDistribution,
            );
        }

        match result {
            KCW_Answer_KWC_OK => Ok(MixtureDistribution {
                mixture: PhantomData,
                count,
                distribution,
            }),
            _ => Err(SOMETHING_WRONG),
        }
    }
    extern "C" fn from_function_to_function<T>(
        state: *mut std::os::raw::c_void,
    ) -> CalculateParams {
        unsafe {
            let (f, s) = *(state as *mut (fn(&mut T) -> CalculateParams, *mut T));
            let s = &mut *s;
            f(s)
        }
    }
    pub fn bozman_distribution_with_function_callback<'a, T>(
        &'a self,
        count: usize,
        next: fn(&mut T) -> CalculateParams,
        state: &mut T,
    ) -> Result<MixtureDistribution<'a>, &'static str> {
        self.bozman_distribution_with_function_callback_inner(
            count,
            Self::from_function_to_function::<T>,
            &mut (next, state) as *mut (fn(&mut T) -> CalculateParams, &mut T)
                as *mut std::os::raw::c_void,
        )
    }
    fn from_closure_to_function<F: FnMut() -> CalculateParams>(f: &mut F) -> CalculateParams {
        (*f)()
    }
    pub fn bozman_distribution_with_closure_callback<'a, F: FnMut() -> CalculateParams>(
        &'a self,
        count: usize,
        mut next: F,
    ) -> Result<MixtureDistribution<'a>, &'static str> {
        self.bozman_distribution_with_function_callback(
            count,
            Self::from_closure_to_function::<F>,
            &mut next,
        )
    }
}

impl Drop for Mixture {
    fn drop(&mut self) {
        unsafe {
            KCW_DestroyMixture(self.mixture);
        }
    }
}

pub struct MixtureDistribution<'a> {
    mixture: PhantomData<&'a Mixture>,
    count: usize,
    distribution: KCW_MixtureDistribution,
}

impl<'a> MixtureDistribution<'a> {
    /// Some if len equal with previous step size
    /// None if it 0.0 for all
    pub fn compute_transport_coefficient(
        &'a self,
        n_electrons: Option<&[f64]>,
        perturbation: Option<&[f64]>,
    ) -> Result<TransportCoefficientWrap, &'static str> {
        let result;
        let mut coefficients = KCW_TransportCoefficientArray {
            data: std::ptr::null_mut::<KCW_TransportCoefficient>(),
            size: 0,
        };
        let n_electrons_ptr = match n_electrons {
            Some(n_electrons) => {
                if n_electrons.len() != self.count {
                    return Err(SOMETHING_WRONG);
                }
                n_electrons.as_ptr()
            }
            None => std::ptr::null::<f64>(),
        };
        let perturbation_ptr = match perturbation {
            Some(perturbation) => {
                if perturbation.len() != self.count {
                    return Err(SOMETHING_WRONG);
                }
                perturbation.as_ptr()
            }
            None => std::ptr::null::<f64>(),
        };

        unsafe {
            result = KCW_MixtureComputeTransportCoefficients(
                self.distribution,
                n_electrons_ptr,
                perturbation_ptr,
                &mut coefficients as *mut KCW_TransportCoefficientArray,
            );
        }
        
        match result {
            KCW_Answer_KWC_OK => Ok(TransportCoefficientWrap {
                array: coefficients,
            }),
            _ => Err(SOMETHING_WRONG),
        }
    }
}

impl<'a> Drop for MixtureDistribution<'a> {
    fn drop(&mut self) {
        unsafe {
            KCW_DestroyMixtureDistribution(self.distribution);
        }
    }
}

pub type TransportCoefficient = kappa_c_wrap::root::KCW_TransportCoefficient;
type TransportCoefficientArray = kappa_c_wrap::root::KCW_TransportCoefficientArray;
pub struct TransportCoefficientWrap {
    array: TransportCoefficientArray,
}

impl TransportCoefficientWrap {
    pub fn as_slice(&self) -> &[TransportCoefficient] {
        unsafe { slice::from_raw_parts(self.array.data, self.array.size as usize) }
    }
}

impl Drop for TransportCoefficientWrap {
    fn drop(&mut self) {
        unsafe {
            KCW_DestroyTransportCoefficientArray(self.array);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_usage_test() {
        println!("0");
        
        let p = "/home/yapomip/project/2025_spbu_coursework/rust/kappa/data/";
        let context = Context::from_path(p).expect("1");
        
        println!("1");
        
        let mixture = context
            .build_mixture()
            .add_molecula("N2", false, false)
            .add_atom("N")
            .build()
            .expect("2");
        
        println!("2");
        
        let T = [5000.0];
        let p = [100000.0];
        let n = [[0.5, 0.5].as_slice()];
        
        let res = mixture
            .bozman_distribution(&T, &p, n.as_slice())
            .expect("3");
        let res = res
            .compute_transport_coefficient(None, None)
            .expect("4");
    
        let a = res.as_slice()[0];
        println!(
            "res {{{} {} {}}}",
            a.thermal_conductivity, a.shear_viscosity, a.bulk_viscosity
        );
    }
}
