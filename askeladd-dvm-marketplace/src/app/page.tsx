"use client";

import { useState, useEffect, useMemo } from "react";
import { NDKEvent } from '@nostr-dev-kit/ndk';
import { ContractUploadType, ProgramInternalContractName } from "@/types";
import init, { stark_proof_wide_fibo, verify_stark_proof_wide_fibo } from "../pkg/stwo_wasm";
import { Event as EventNostr } from "nostr-tools";
import { useDVMState } from "@/hooks/useDVMState";
export default function Home() {
  const [log_n_instances, setLogNInstances] = useState<number>(0);
  const [log_fibonacci_size, setLogFibonacciSize] = useState<number>(5);
  const [error, setError] = useState<string | undefined>()
  const [jobEventResult, setJobEventResult] = useState<EventNostr | undefined | NDKEvent>()
  const [proofStatus, setProofStatus] = useState<
    "idle" | "pending" | "received" | "verified"
  >("idle");
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [isInitialized, setIsInitialized] = useState(false);
  const [isFetchJob, setIsFetchJob] = useState(false);
  const { eventIdRequest, jobId, setJobId, setIsWaitingJob, fetchJobRequest, proof, fetchEventsProof,
    starkProof,
    submitJob: submitJobModular,
    publicKey } = useDVMState()
  // Init wasm module to run_fibonacci_verify
  useEffect(() => {
    init()
      .then(() => setIsInitialized(true))
      .catch((error) => {
        console.error("Failed to initialize WASM module:", error);

      });
  }, []);

  /** Effect to fetch the job result when a job request is sent */
  const waitingForJobResult = async () => {
    if (jobEventResult && jobId) return;
    fetchEventsProof()
    setIsLoading(false);
    setIsWaitingJob(false)
  }
  const timeoutWaitingForJobResult = async () => {
    console.log("waiting timeout job result")
    setTimeout(() => {
      waitingForJobResult()
    }, 5000);
  }

  useEffect(() => {
    if (jobId && !jobEventResult) {
      waitingForJobResult()
      timeoutWaitingForJobResult()
    }
  }, [jobId, isFetchJob, jobEventResult])


  /** Submit job with JOB_REQUEST 5600
   * - Use extension NIP-7
   * - Default public key demo
   * - NDK generate key or import later
  */
  const submitJob = async () => {
    try {
      setIsLoading(true);
      setIsFetchJob(false);
      setJobId(undefined)
      setProofStatus("pending");
      setJobEventResult(undefined);
      setError(undefined);
      const tags = [
        ['param', 'log_n_instances', log_n_instances.toString()],
        ['param', 'log_fibonacci_size', log_fibonacci_size.toString()],
        ['output', 'text/json']
      ];

      const tags_values = [
        ['param', 'log_n_instances', log_n_instances.toString()],
        ['param', 'log_fibonacci_size', log_fibonacci_size.toString()],
      ];


      const inputs: Map<string, string> = new Map<string, string>();

      for (let tag of tags_values) {
        inputs.set(tag[1], tag[2])
      }
      console.log("parent inputs", Object.fromEntries(inputs))

      const zkp_request = {
        request: {
          log_n_instances: log_n_instances.toString(),
          log_fibonacci_size: log_fibonacci_size.toString(),
        },
        program: {
          contract_name: ProgramInternalContractName.WideFibonacciProvingRequest.toString(),
          internal_contract_name: ProgramInternalContractName.WideFibonacciProvingRequest,
          contract_reached: ContractUploadType.InternalAskeladd,
          inputs: inputs,
        }
      }

      let res = await submitJobModular(5600, {
        log_n_instances,
        log_fibonacci_size
      },
        zkp_request,
        tags

      )
      fetchJobRequest(undefined, publicKey)
      waitingForJobResult()
      timeoutWaitingForJobResult()

    } catch (e) {
    } finally {
      setIsLoading(false);
    }

  };

  const verifyProofHandler = async () => {
    try {
      if (proof) {
        setIsLoading(true);
        /** Change Wide fibo to default */
        const serialised_proof_from_nostr_event = JSON.stringify(starkProof);
        if (!log_n_instances && !log_fibonacci_size && !serialised_proof_from_nostr_event) return;
        const prove_result = stark_proof_wide_fibo(Number(log_fibonacci_size), Number(log_n_instances));
        console.log("wide fibo prove_result", prove_result);
        console.log("serialised_proof_from_nostr_event", serialised_proof_from_nostr_event);
        const verify_result = verify_stark_proof_wide_fibo(Number(log_fibonacci_size), Number(log_n_instances), serialised_proof_from_nostr_event);
        console.log("verify result", verify_result);
        console.log("verify message", verify_result.message);
        console.log("verify success", verify_result.success);
        if (verify_result?.success) {
          console.log("is success verify result")
          setProofStatus("verified");
        } else {
          setError(verify_result?.message)
        }
        setIsLoading(false);
        setIsFetchJob(true)
      }
    } catch (e) {
      console.log("Verify error", e);
    } finally {
      setIsLoading(false);
      setIsFetchJob(true)

    }
  };

  return (
    <main className="min-h-screen bg-black text-neon-green font-arcade p-4 pb-16 relative overflow-hidden">
      <div className="crt-overlay"></div>
      <div className="scanlines"></div>
      <div className="crt-curve"></div>
      <div className="arcade-cabinet">
        <div className="cabinet-content">
          <h1 className="text-4xl mb-4 text-center glitch neon-text" data-text="Askeladd DVM Arcade">Askeladd DVM Marketplace</h1>
          <p className="text-center blink neon-text-sm">Censorship resistant global proving network</p>
          <p className="text-center blink neon-text-sm">Verifiable computation for DVMs</p>
          <div className="max-w-md mx-auto bg-dark-purple p-6 rounded-lg shadow-neon mt-8 relative game-screen">
            {/* <p>Prove poseidon</p> */}
            <p>Wide Fibonacci</p>
            <div className="mb-4">
              <label className="block mb-2 text-neon-pink">Log Fibonacci Size</label>
              <input
                type="number"
                value={log_fibonacci_size}
                onChange={(e) => setLogFibonacciSize(Number(e.target.value))}
                className="w-full bg-black text-neon-green px-3 py-2 rounded border-neon-green border-2"
              />
            </div>
            <div className="mb-4">
              <label className="block mb-2 text-neon-pink">Log N Instances</label>
              <input
                type="number"
                value={log_n_instances}
                onChange={(e) => setLogNInstances(Number(e.target.value))}
                className="w-full bg-black text-neon-green px-3 py-2 rounded border-neon-green border-2"
              />
            </div>
            <button
              onClick={submitJob}
              disabled={isLoading}
              className={`submit-job-button ${isLoading ? "opacity-50 cursor-not-allowed" : ""
                }`}
            >
              {isLoading ? "PROCESSING..." : "SUBMIT JOB"}
            </button>
          </div>
          {isLoading && <div className="pixel-spinner mt-4 mx-auto"></div>}
          {jobId && (
            <div className="mt-8 text-center">
              <p className="text-neon-orange text-sm sm:text-base">Job ID: <span className="break-all">{jobId}</span></p>
              <p className="text-neon-yellow text-sm sm:text-base">Status: {proofStatus}</p>

              {error && <p className="text-neon-red blink">Error: {error}</p>}
              {proof && (
                <div className="proof-container">
                  <p className="mt-4 text-neon-pink">Proof received:</p>
                  <pre className="bg-dark-purple p-4 rounded mt-2 overflow-x-auto text-neon-green text-xs">
                    {proof}
                  </pre>
                  {starkProof && (
                    <p className="text-neon-yellow">
                      Proof of work nonce: {starkProof?.commitment_scheme_proof?.proof_of_work?.nonce}
                    </p>
                  )}
                  <button
                    onClick={verifyProofHandler}
                    disabled={isLoading}
                    className={`verify-proof-button mt-4 ${isLoading ? "opacity-50 cursor-not-allowed" : ""
                      }`}
                  >
                    {isLoading ? "VERIFYING..." : "VERIFY PROOF"}
                  </button>
                </div>
              )}
            </div>
          )}
        </div>
      </div>
      <div className="marquee">
        <span>Prove your claims and conquer the Nostr realm!</span>
      </div>
    </main>
  );
}
