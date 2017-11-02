use cose::test_setup as test;
use cose::nss;
use cose::nss::NSSError;

#[test]
fn test_nss_sign_verify() {
    test::setup();
    let payload = b"sample";

    // Sign.
    let signature_result = nss::sign(
        &nss::SignatureAlgorithm::ES256,
        &test::PKCS8_P256_EE,
        payload,
    );
    assert!(signature_result.is_ok());
    let signature_result = signature_result.unwrap();

    // Verify the signature.
    assert!(
        nss::verify_signature(
            &nss::SignatureAlgorithm::ES256,
            &test::P256_EE,
            payload,
            &signature_result,
        ).is_ok()
    );
}

#[test]
fn test_nss_sign_verify_different_payload() {
    test::setup();
    let payload = b"sample";

    // Sign.
    let signature_result = nss::sign(
        &nss::SignatureAlgorithm::ES256,
        &test::PKCS8_P256_EE,
        payload,
    );
    assert!(signature_result.is_ok());
    let signature_result = signature_result.unwrap();

    // Verify the signature with a different payload.
    let payload = b"sampli";
    let verify_result = nss::verify_signature(
        &nss::SignatureAlgorithm::ES256,
        &test::P256_EE,
        payload,
        &signature_result,
    );
    assert!(verify_result.is_err());
    assert_eq!(verify_result, Err(NSSError::SignatureVerificationFailed));
}

#[test]
fn test_nss_sign_verify_wrong_cert() {
    test::setup();
    let payload = b"sample";

    // Sign.
    let signature_result = nss::sign(
        &nss::SignatureAlgorithm::ES256,
        &test::PKCS8_P256_EE,
        payload,
    );
    assert!(signature_result.is_ok());
    let signature_result = signature_result.unwrap();

    // Verify the signature with a wrong cert.
    let verify_result = nss::verify_signature(
        &nss::SignatureAlgorithm::ES256,
        &test::P384_EE,
        payload,
        &signature_result,
    );
    assert!(verify_result.is_err());
    assert_eq!(verify_result, Err(NSSError::SignatureVerificationFailed));
}
