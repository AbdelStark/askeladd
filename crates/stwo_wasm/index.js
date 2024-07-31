import init, { prove_and_verify, verify_stark_proof } from './pkg/stwo_wasm.js';

async function loadJSON(url) {
  const response = await fetch(url);
  return response.json();
}


async function runTests() {
    await init();

    const resultsDiv = document.getElementById('results');
    function addResult(message) {
        const p = document.createElement('p');
        p.textContent = message;
        resultsDiv?.appendChild(p);
        console.log(message);
    }

    console.log("===============================");
    // Test 1: prove_and_verify
    const test1 = prove_and_verify(5, 443693538);
    addResult(`Test 1 (prove_and_verify): ${test1.success ? 'Success' : 'Failure'} - ${test1.message}`);

    console.log("===============================");
    // Test 2: verify_stark_proof with valid proof
    if (test1.success) {
        console.log("Serializing proof from test1");
        const serializedProof = test1.message;
        if (serializedProof) {
            console.log("Verifying proof from test1");
            console.log("Serialized proof length:", serializedProof.length);
            try {
                const test2 = verify_stark_proof(5, 443693538, serializedProof);
                addResult(`Test 2 (verify_stark_proof with valid proof): ${test2.success ? 'Success' : 'Failure'} - ${test2.message}`);
            } catch (error) {
                console.error("Error in verify_stark_proof:", error);
                addResult(`Test 2 (verify_stark_proof with valid proof): Failure - ${error.message}`);
            }
        } else {
            addResult("Test 2 skipped: No serialized proof available");
        }
    }

    //console.log("===============================");
    

    // Test 3: verify_stark_proof with invalid proof
    // const invalidProof = JSON.stringify({ invalid: "proof" });
    // try {
    //     const test3 = verify_stark_proof(5, 443693538, invalidProof);
    //     addResult(`Test 3 (verify_stark_proof with invalid proof): ${test3.success ? 'Success' : 'Failure'} - ${test3.message}`);
    // } catch (error) {
    //     console.error("Error in verify_stark_proof (invalid proof):", error);
    //     addResult(`Test 3 (verify_stark_proof with invalid proof): Failure - ${error.message}`);
    // }

    // console.log("===============================");
    // // Test 4: prove_and_verify with different parameters
    // const test4 = prove_and_verify(6, 123456789);
    // addResult(`Test 4 (prove_and_verify with different parameters): ${test4.success ? 'Success' : 'Failure'} - ${test4.message}`);

    // console.log("===============================");
    // // Test 5: verify_stark_proof with mismatched parameters
    // if (test1.success) {
    //     const serializedProof = test1.message;
    //     if (serializedProof) {
    //         try {
    //             const test5 = verify_stark_proof(6, 123456789, serializedProof);
    //             addResult(`Test 5 (verify_stark_proof with mismatched parameters): ${test5.success ? 'Success' : 'Failure'} - ${test5.message}`);
    //         } catch (error) {
    //             console.error("Error in verify_stark_proof (mismatched parameters):", error);
    //             addResult(`Test 5 (verify_stark_proof with mismatched parameters): Failure - ${error.message}`);
    //         }
    //     } else {
    //         addResult("Test 5 skipped: No serialized proof available");
    //     }
    // }

    console.log("===============================");
    // Test 6: verify_stark_proof with hardcoded proof in hardcoded_serialised_proof_from_nostr_event.json
    console.log("Verifying hardcoded proof");
    try {
        const hardcoded_serialised_proof_from_nostr_event = await loadJSON('./hardcoded_serialised_proof_from_nostr_event.json');
        const hardcoded_proof = JSON.stringify(hardcoded_serialised_proof_from_nostr_event);
        console.log("Hardcoded proof length:", hardcoded_proof.length);
        const test6 = verify_stark_proof(5, 443693538, hardcoded_proof);
        addResult(`Test 6 (verify_stark_proof with hardcoded proof): ${test6.success ? 'Success' : 'Failure'} - ${test6.message}`);
    } catch (error) {
        console.error("Error in verify_stark_proof (hardcoded proof):", error);
        addResult(`Test 6 (verify_stark_proof with hardcoded proof): Failure - ${error.message}`);
    }
}

runTests();