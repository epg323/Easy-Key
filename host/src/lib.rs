use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
use EasyKey::EASYKEYGEN_ELF; // Using Tokio for async handling

pub fn keygen(a: String, b: String) -> (Receipt, String) {
    let env = ExecutorEnv::builder()
        .write(&a)
        .unwrap()
        .write(&b)
        .unwrap()
        .build()
        .unwrap();

    let prover = default_prover();
    let mut pub_address: String = "test".to_string();

    let receipt = prover.prove(env, EASYKEYGEN_ELF).expect("error");
    pub_address = receipt.journal.decode().expect(
        "Journal output should deserialize into the same ypes (& order) that it was written",
    );

    return (receipt, pub_address);
}
