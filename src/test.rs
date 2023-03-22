#[cfg(test)]
mod tests {
    use crate::{
        ds::{
            CanonicalizationMethod, DigestMethod, Reference, SignatureCopy, SignatureMethod,
            SignedInfo, Transform, Transforms,
        },
        to_events,
    };

    #[test]
    fn to_event() {
        let sign_info = SignedInfo {
            canonicalization_method: CanonicalizationMethod {
                algorithm: "http://www.w3.org/TR/2001/REC-xml-c14n-20010315".to_string(),
            },
            signature_method: SignatureMethod {
                algorithm: "http://www.w3.org/2000/09/xmldsig#rsa-sha1".to_string(),
            },
            reference: [Reference {
                transforms: Some(Transforms {
                    transforms: [Transform {
                        algorithm: "http://www.w3.org/2000/09/xmldsig#enveloped-signature"
                            .to_string(),
                    }]
                    .to_vec(),
                }),
                digest_method: DigestMethod {
                    algorithm: "http://www.w3.org/2000/09/xmldsig#sha1".to_string(),
                },
                digest_value: "iEauNqezvNrpcl8YJBj9OhziUXQ=".to_string(),
                id: None,
                uri: Some("".to_string()),
                ref_type: None,
            }]
            .to_vec(),
            id: None,
        };

        let copy_singin = SignatureCopy {
            signed_info: sign_info,
        };

        let result = to_events(&copy_singin).unwrap();

        println!("result : {result:?}");
    }
}
