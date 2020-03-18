// Copyright (c) 2019-2020, Arm Limited, All Rights Reserved
// SPDX-License-Identifier: Apache-2.0
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may
// not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//          http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use super::generated_ops::psa_verify_hash::{Operation as OperationProto, Result as ResultProto};
use crate::operations::psa_verify_hash::{Operation, Result};
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
                    error!("alg field of psa_verify_hash::Operation message is empty.");
                    ResponseStatus::InvalidEncoding
                })?
                .try_into()?,
            hash: proto_op.hash,
            signature: proto_op.signature,
        })
    }
}

impl TryFrom<Operation> for OperationProto {
    type Error = ResponseStatus;

    fn try_from(op: Operation) -> std::result::Result<Self, Self::Error> {
        Ok(OperationProto {
            key_name: op.key_name,
            alg: Some(op.alg.try_into()?),
            hash: op.hash,
            signature: op.signature,
        })
    }
}

impl TryFrom<ResultProto> for Result {
    type Error = ResponseStatus;

    fn try_from(_proto_result: ResultProto) -> std::result::Result<Self, Self::Error> {
        Ok(Result {})
    }
}

impl TryFrom<Result> for ResultProto {
    type Error = ResponseStatus;

    fn try_from(_result: Result) -> std::result::Result<Self, Self::Error> {
        Ok(ResultProto {})
    }
}

#[cfg(test)]
mod test {
    use super::super::generated_ops::psa_algorithm as algorithm_proto;
    use super::super::generated_ops::psa_verify_hash::{
        Operation as OperationProto, Result as ResultProto,
    };
    use super::super::{Convert, ProtobufConverter};
    use crate::operations::psa_algorithm::AsymmetricSignature;
    use crate::operations::psa_verify_hash::{Operation, Result};
    use crate::operations::{NativeOperation, NativeResult};
    use crate::requests::{request::RequestBody, response::ResponseBody, Opcode};
    use std::convert::TryInto;

    static CONVERTER: ProtobufConverter = ProtobufConverter {};

    #[test]
    fn asym_proto_to_op() {
        let mut proto: OperationProto = Default::default();
        let hash = vec![0x11, 0x22, 0x33];
        let key_name = "test name".to_string();
        let signature = vec![0x11, 0x22, 0x33];
        proto.hash = hash.clone();
        proto.alg = Some(algorithm_proto::algorithm::AsymmetricSignature {
            variant: Some(
                algorithm_proto::algorithm::asymmetric_signature::Variant::RsaPkcs1v15Sign(
                    algorithm_proto::algorithm::asymmetric_signature::RsaPkcs1v15Sign {
                        hash_alg: algorithm_proto::algorithm::Hash::Sha1.into(),
                    },
                ),
            ),
        });
        proto.key_name = key_name.clone();
        proto.signature = signature.clone();

        let op: Operation = proto.try_into().expect("Failed to convert");

        assert_eq!(op.hash, hash);
        assert_eq!(op.key_name, key_name);
        assert_eq!(op.signature, signature);
    }

    #[test]
    fn asym_op_to_proto() {
        let hash = vec![0x11, 0x22, 0x33];
        let key_name = "test name".to_string();
        let signature = vec![0x11, 0x22, 0x33];

        let op = Operation {
            hash: hash.clone(),
            alg: AsymmetricSignature::RsaPkcs1v15SignRaw,
            key_name: key_name.clone(),
            signature: signature.clone(),
        };

        let proto: OperationProto = op.try_into().expect("Failed to convert");

        assert_eq!(proto.hash, hash);
        assert_eq!(proto.key_name, key_name);
        assert_eq!(proto.signature, signature);
    }

    #[test]
    fn asym_proto_to_resp() {
        let proto: ResultProto = Default::default();

        let _result: Result = proto.try_into().expect("Failed to convert");
    }

    #[test]
    fn asym_resp_to_proto() {
        let result = Result {};

        let _proto: ResultProto = result.try_into().expect("Failed to convert");
    }

    #[test]
    fn op_asym_sign_e2e() {
        let op = Operation {
            hash: vec![0x11, 0x22, 0x33],
            alg: AsymmetricSignature::RsaPkcs1v15SignRaw,
            key_name: "test name".to_string(),
            signature: vec![0x11, 0x22, 0x33],
        };
        let body = CONVERTER
            .operation_to_body(NativeOperation::PsaVerifyHash(op))
            .expect("Failed to convert request");

        assert!(CONVERTER
            .body_to_operation(body, Opcode::PsaVerifyHash)
            .is_ok());
    }

    #[test]
    fn resp_asym_sign_e2e() {
        let result = Result {};
        let body = CONVERTER
            .result_to_body(NativeResult::PsaVerifyHash(result))
            .expect("Failed to convert request");

        assert!(CONVERTER
            .body_to_result(body, Opcode::PsaVerifyHash)
            .is_ok());
    }

    #[test]
    fn result_from_mangled_resp_body() {
        let resp_body =
            ResponseBody::from_bytes(vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88]);
        assert!(CONVERTER
            .body_to_result(resp_body, Opcode::PsaVerifyHash)
            .is_err());
    }

    #[test]
    fn op_from_mangled_req_body() {
        let req_body =
            RequestBody::from_bytes(vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88]);

        assert!(CONVERTER
            .body_to_operation(req_body, Opcode::PsaVerifyHash)
            .is_err());
    }
}