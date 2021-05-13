// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0
use super::generated_ops::psa_aead_encrypt::{Operation as OperationProto, Result as ResultProto};
use crate::operations::psa_aead_encrypt::{Operation, Result};
use crate::requests::ResponseStatus;
use log::error;
use std::convert::{TryFrom, TryInto};

impl TryFrom<OperationProto> for Operation {
    type Error = ResponseStatus;

    fn try_from(proto_op: OperationProto) -> std::result::Result<Self, Self::Error> {
        Ok(Operation {
            key_name: proto_op.key_name,
            alg: proto_op
                .alg
                .ok_or_else(|| {
                    error!("The alg field of PsaAeadEncrypt::Operation message is not set (mandatory field).");
                    ResponseStatus::InvalidEncoding
                })?
                .try_into()?,
            additional_data: proto_op.additional_data.into(),
            plaintext: proto_op.plaintext.into(),
            nonce: proto_op.nonce.into(),
        })
    }
}

impl TryFrom<Operation> for OperationProto {
    type Error = ResponseStatus;

    fn try_from(op: Operation) -> std::result::Result<Self, Self::Error> {
        let alg = Some(op.alg.try_into()?);
        Ok(OperationProto {
            key_name: op.key_name,
            alg,
            additional_data: op.additional_data.to_vec(),
            plaintext: op.plaintext.to_vec(),
            nonce: op.nonce.to_vec(),
        })
    }
}

impl TryFrom<ResultProto> for Result {
    type Error = ResponseStatus;

    fn try_from(proto_result: ResultProto) -> std::result::Result<Self, Self::Error> {
        Ok(Result {
            ciphertext: proto_result.ciphertext.into(),
        })
    }
}

impl TryFrom<Result> for ResultProto {
    type Error = ResponseStatus;

    fn try_from(result: Result) -> std::result::Result<Self, Self::Error> {
        Ok(ResultProto {
            ciphertext: result.ciphertext.to_vec(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::super::generated_ops::psa_aead_encrypt::{
        Operation as OperationProto, Result as ResultProto,
    };
    use super::super::generated_ops::psa_algorithm as algorithm_proto;
    use super::super::{Convert, ProtobufConverter};
    use crate::operations::psa_aead_encrypt::{Operation, Result};
    use crate::operations::psa_algorithm::AeadWithDefaultLengthTag;
    use std::convert::TryInto;
    static CONVERTER: ProtobufConverter = ProtobufConverter {};
    use crate::operations::{NativeOperation, NativeResult};
    use crate::requests::{request::RequestBody, response::ResponseBody, Opcode};

    #[test]
    fn aead_proto_to_op() {
        let mut proto: OperationProto = Default::default();
        let plaintext = vec![0x11, 0x22, 0x33];
        let additional_data = vec![0x66, 0x77, 0x88];
        let key_name = "test name".to_string();
        let nonce: Vec<u8> = vec![0x55; 12];
        proto.plaintext = plaintext.clone();
        proto.alg = Some(algorithm_proto::algorithm::Aead {
            variant: Some(
                algorithm_proto::algorithm::aead::Variant::AeadWithDefaultLengthTag(
                    algorithm_proto::algorithm::aead::AeadWithDefaultLengthTag::Ccm.into(),
                ),
            ),
        });
        proto.key_name = key_name.clone();
        proto.nonce = nonce.clone();
        proto.additional_data = additional_data.clone();

        let op: Operation = proto.try_into().expect("Failed to convert");

        assert_eq!(*op.plaintext, plaintext);
        assert_eq!(op.key_name, key_name);
        assert_eq!(*op.additional_data, additional_data);
        assert_eq!(*op.nonce, nonce);
        assert_eq!(op.alg, AeadWithDefaultLengthTag::Ccm.into());
    }

    #[test]
    fn aead_op_to_proto() {
        let plaintext = vec![0x11, 0x22, 0x33];
        let additional_data = vec![0x55, 0x66, 0x77];
        let nonce: Vec<u8> = vec![0x55; 12];
        let key_name = "test name".to_string();

        let op = Operation {
            plaintext: plaintext.clone().into(),
            alg: AeadWithDefaultLengthTag::Ccm.into(),
            key_name: key_name.clone(),
            nonce: nonce.clone().into(),
            additional_data: additional_data.clone().into(),
        };

        let proto: OperationProto = op.try_into().expect("Failed to convert");

        assert_eq!(proto.plaintext, plaintext);
        assert_eq!(proto.key_name, key_name);
        assert_eq!(proto.nonce, nonce);
        assert_eq!(proto.additional_data, additional_data);
    }

    #[test]
    fn aead_proto_to_resp() {
        let mut proto: ResultProto = Default::default();
        let ciphertext = vec![0x11, 0x22, 0x33];
        proto.ciphertext = ciphertext.clone();

        let result: Result = proto.try_into().expect("Failed to convert");

        assert_eq!(*result.ciphertext, ciphertext);
    }

    #[test]
    fn aead_resp_to_proto() {
        let ciphertext = vec![0x11, 0x22, 0x33];
        let result = Result {
            ciphertext: ciphertext.clone().into(),
        };

        let proto: ResultProto = result.try_into().expect("Failed to convert");

        assert_eq!(proto.ciphertext, ciphertext);
    }

    #[test]
    fn psa_encrypt_message_op_e2e() {
        let plaintext = vec![0x11, 0x22, 0x33];
        let additional_data = vec![0x55, 0x66, 0x77];
        let nonce: Vec<u8> = vec![0x55; 12];
        let key_name = "test name".to_string();

        let op = Operation {
            plaintext: plaintext.into(),
            alg: AeadWithDefaultLengthTag::Ccm.into(),
            key_name,
            nonce: nonce.into(),
            additional_data: additional_data.into(),
        };

        let body = CONVERTER
            .operation_to_body(NativeOperation::PsaAeadEncrypt(op))
            .expect("Failed to convert to body");

        let _ = CONVERTER
            .body_to_operation(body, Opcode::PsaAeadEncrypt)
            .expect("Failed to convert to operation");
    }

    #[test]
    fn resp_aead_encrypt_e2e() {
        let result = Result {
            ciphertext: vec![0x11, 0x22, 0x33].into(),
        };
        let body = CONVERTER
            .result_to_body(NativeResult::PsaAeadEncrypt(result))
            .expect("Failed to convert request");

        assert!(CONVERTER
            .body_to_result(body, Opcode::PsaAeadEncrypt)
            .is_ok());
    }

    #[test]
    fn result_from_mangled_resp_body() {
        let resp_body =
            ResponseBody::from_bytes(vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88]);
        assert!(CONVERTER
            .body_to_result(resp_body, Opcode::PsaAeadEncrypt)
            .is_err());
    }

    #[test]
    fn op_from_mangled_req_body() {
        let req_body =
            RequestBody::from_bytes(vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88]);

        assert!(CONVERTER
            .body_to_operation(req_body, Opcode::PsaAeadEncrypt)
            .is_err());
    }
}
