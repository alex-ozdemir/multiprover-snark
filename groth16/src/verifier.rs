use ark_ec::{AffineCurve, PairingEngine, ProjectiveCurve};
use ark_ff::PrimeField;

use super::{PreparedVerifyingKey, Proof, VerifyingKey};

use ark_relations::r1cs::{Result as R1CSResult, SynthesisError};

use core::ops::{AddAssign, Neg};
use ark_std::ops::MulAssign;

/// Prepare the verifying key `vk` for use in proof verification.
pub fn prepare_verifying_key<E: PairingEngine>(vk: &VerifyingKey<E>) -> PreparedVerifyingKey<E> {
    PreparedVerifyingKey {
        vk: vk.clone(),
        alpha_g1_beta_g2: E::pairing(vk.alpha_g1, vk.beta_g2),
        gamma_g2_neg_pc: vk.gamma_g2.neg().into(),
        delta_g2_neg_pc: vk.delta_g2.neg().into(),
    }
}

/// Verify a Groth16 proof `proof` against the prepared verification key `pvk`,
/// with respect to the instance `public_inputs`.
pub fn verify_proof<E: PairingEngine>(
    pvk: &PreparedVerifyingKey<E>,
    proof: &Proof<E>,
    public_inputs: &[E::Fr],
) -> R1CSResult<bool> {
    if (public_inputs.len() + 1) != pvk.vk.gamma_abc_g1.len() {
        return Err(SynthesisError::MalformedVerifyingKey);
    }

    let mut g_ic = pvk.vk.gamma_abc_g1[0].into_projective();
    for (i, b) in public_inputs.iter().zip(pvk.vk.gamma_abc_g1.iter().skip(1)) {
        g_ic.add_assign(&b.mul(i.into_repr()));
    }

    let mut test2 = E::pairing(proof.a, proof.b);
    test2.mul_assign(E::pairing(g_ic.into_affine(), pvk.vk.gamma_g2.neg()));
    test2.mul_assign(E::pairing(proof.c, pvk.vk.delta_g2.neg()));

//    assert_eq!(E::pairing(proof.a, proof.b), E::final_exponentiation(&E::miller_loop([(proof.a.into(), proof.b.into())].iter())).unwrap());
//    assert_eq!(E::pairing(g_ic.into_affine(), pvk.vk.gamma_g2.neg()), E::final_exponentiation(&E::miller_loop([(g_ic.into_affine().into(), pvk.gamma_g2_neg_pc.clone())].iter())).unwrap());
//    assert_eq!(E::pairing(proof.c, pvk.vk.delta_g2.neg()), E::final_exponentiation(&E::miller_loop([(proof.c.into(), pvk.delta_g2_neg_pc.clone())].iter())).unwrap());
//
//    let qap = E::miller_loop(
//        [
//            (proof.a.into(), proof.b.into()),
//            (g_ic.into_affine().into(), pvk.gamma_g2_neg_pc.clone()),
//            (proof.c.into(), pvk.delta_g2_neg_pc.clone()),
//        ]
//        .iter(),
//    );
//
//    let test = E::final_exponentiation(&qap).ok_or(SynthesisError::UnexpectedIdentity)?;

//    assert_eq!(test, test2);
    Ok(test2 == pvk.alpha_g1_beta_g2)
}
