// To generate test vectors:
// cargo run --example issue-revocation-list > tests/revocationList.json

#[async_std::main]
async fn main() {
    let key_str = include_str!("../tests/rsa2048-2020-08-25.json");
    let key: ssi::jwk::JWK = serde_json::from_str(key_str).unwrap();
    let resolver = &ssi::did::example::DIDExample;
    let vc = serde_json::json!({
      "@context": [
        "https://www.w3.org/2018/credentials/v1",
        "https://w3id.org/vc-revocation-list-2020/v1"
      ],
      "id": "https://example.test/revocationList.json",
      "type": [
        "VerifiableCredential",
        "RevocationList2020Credential"
      ],
      "credentialSubject": {
        "type": "RevocationList2020",
        "encodedList": "H4sIAAAAAAAAA-3BMQ0AAAACIGf_0MbwARoAAAAAAAAAAAAAAAAAAADgbbmHB0sAQAAA"
      },
      "issuer": "did:example:foo",
      "issuanceDate": ssi::ldp::now_ms()
    });
    let mut vc: ssi::vc::Credential = serde_json::from_value(vc).unwrap();
    let mut proof_options = ssi::vc::LinkedDataProofOptions::default();
    let verification_method = "did:example:foo#key1".to_string();
    proof_options.verification_method = Some(ssi::vc::URI::String(verification_method));
    let proof = vc
        .generate_proof(&key, &proof_options, resolver)
        .await
        .unwrap();
    vc.add_proof(proof);
    let result = vc.verify(None, resolver).await;
    if result.errors.len() > 0 {
        panic!("verify failed: {:#?}", result);
    }
    let stdout_writer = std::io::BufWriter::new(std::io::stdout());
    serde_json::to_writer_pretty(stdout_writer, &vc).unwrap();
}
