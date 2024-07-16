use askeladd_core::prover_service::ProverService;
use askeladd_core::types::FibonnacciProvingRequest;
use askeladd_core::verifier_service::VerifierService;

fn main() {
    let proving_service: ProverService = Default::default();
    let log_size = 5;
    let claim = 443693538;
    let request = FibonnacciProvingRequest { log_size, claim };
    println!("Generating proof...");
    let response = proving_service.generate_proof(request).unwrap();
    println!("Proof successfully generated.");
    let verifier_service: VerifierService = Default::default();
    println!("Verifying proof...");
    match verifier_service.verify_proof(response) {
        Ok(_) => println!("Proof successfully verified"),
        Err(e) => println!("Proof verification failed: {}", e),
    }
}
