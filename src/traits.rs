use crate::marlin::MultiPC;
use ark_bls12_381::Fr;
use ark_marlin::{IndexVerifierKey, Proof};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Read, SerializationError, Write};
use evm::ExitError;
use evm_precompile_utils::{EvmData, EvmDataReader, EvmDataWriter, EvmResult};

#[derive(CanonicalSerialize, CanonicalDeserialize)]
pub struct IndVerifierKey(pub IndexVerifierKey<Fr, MultiPC>);

#[derive(CanonicalSerialize, CanonicalDeserialize)]
pub struct Proof1(pub Proof<Fr, MultiPC>);

#[derive(CanonicalSerialize, CanonicalDeserialize, Clone)]
pub struct Field(pub Fr);

impl EvmData for IndVerifierKey {
    fn read(reader: &mut EvmDataReader) -> EvmResult<Self> {
        let result = IndexVerifierKey::<_, _>::deserialize(reader);
        let ivk = match result {
            Ok(data) => data,
            _ => return Err(ExitError::Other("De-Serialize Error".into())),
        };

        Ok(IndVerifierKey(ivk))
    }

    fn write(writer: &mut EvmDataWriter, value: Self) {
        let _ = value.0.serialize(writer);
    }
}

impl EvmData for Proof1 {
    fn read(reader: &mut EvmDataReader) -> EvmResult<Self> {
        let result = Proof::<_, _>::deserialize(reader);
        let proof = match result {
            Ok(data) => data,
            _ => return Err(ExitError::Other("De-Serialize Error".into())),
        };

        Ok(Proof1(proof))
    }

    fn write(writer: &mut EvmDataWriter, value: Self) {
        let _ = value.0.serialize(writer);
    }
}

impl EvmData for Field {
    fn read(reader: &mut EvmDataReader) -> EvmResult<Self> {
        let result = Fr::deserialize(reader);
        let fr = match result {
            Ok(data) => data,
            _ => return Err(ExitError::Other("De-Serialize Error".into())),
        };

        Ok(Field(fr))
    }

    fn write(writer: &mut EvmDataWriter, value: Self) {
        let _ = value.0.serialize(writer);
    }
}

#[test]
fn test_ind_verifier_key() {
    let (ivk, _, _) = crate::marlin::get_proof_data();
    let writer_output = EvmDataWriter::new().write(IndVerifierKey(ivk)).build();

    let mut reader = EvmDataReader::new(&writer_output);
    let parsed: IndVerifierKey = reader
        .read()
        .expect("to correctly parse IndexVerifierKey<Fr, MultiPC>");

    let second_output = EvmDataWriter::new().write(parsed).build();

    assert_eq!(writer_output, second_output);
}

#[test]
fn test_proof_serialization() {
    let (_, proof, _) = crate::marlin::get_proof_data();
    let writer_output = EvmDataWriter::new().write(Proof1(proof)).build();

    let mut reader = EvmDataReader::new(&writer_output);
    let parsed: Proof1 = reader
        .read()
        .expect("to correctly parse Proof<Fr, MultiPC>");

    let second_output = EvmDataWriter::new().write(parsed).build();

    assert_eq!(writer_output, second_output);
}

#[test]
fn test_field_serialization() {
    let (_, _, field_arr) = crate::marlin::get_proof_data();
    let input: [Field; 2] = [Field(field_arr[0]), Field(field_arr[1])];
    let writer_output = EvmDataWriter::new().write(input.to_vec()).build();

    let mut reader = EvmDataReader::new(&writer_output);
    let parsed: Vec<Field> = reader
        .read()
        .expect("to correctly parse Proof<Fr, MultiPC>");

    let second_output = EvmDataWriter::new().write(parsed).build();

    assert_eq!(writer_output, second_output);
}
