"use client";

import { useState, useEffect } from "react";
import { verifyProof } from "../lib/stwo";

export default function Home() {
  const [logSize, setLogSize] = useState<number>(5);
  const [claim, setClaim] = useState<number>(443693538);
  const [jobId, setJobId] = useState<string | null>(null);
  const [proofStatus, setProofStatus] = useState<
    "idle" | "pending" | "received" | "verified"
  >("idle");
  const [proof, setProof] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState<boolean>(false);

  useEffect(() => {
    const initNostr = async () => {
      // TODO: load the wasm module
      //await (window as any).loadWasmAsync();
    };
    initNostr();
  }, []);

  const submitJob = async () => {
    setIsLoading(true);
    setProofStatus("pending");

    // Mock event id
    const eventId = Math.random().toString(36).substring(7);
    setJobId(eventId);

    // Simulate waiting for job result
    setTimeout(() => {
      const mockProof = {
        proof: "mocked_proof_data",
        public_inputs: [logSize, claim],
      };
      setProof(JSON.stringify(mockProof));
      setProofStatus("received");
      setIsLoading(false);
    }, 5000);
  };

  const verifyProofHandler = async () => {
    if (proof) {
      setIsLoading(true);
      const isValid = await verifyProof(proof);
      setProofStatus(isValid ? "verified" : "idle");
      setIsLoading(false);
    }
  };

  return (
    <main className="min-h-screen bg-black text-green-400 font-mono p-8">
      <h1 className="text-4xl mb-8 text-center">Askeladd DVM Marketplace</h1>
      <p className="text-center">Censorship global proving network</p>
      <p className="text-center mb-8">Powered by Nostr and Circle STARKs.</p>

      <div className="max-w-md mx-auto bg-gray-900 p-6 rounded-lg shadow-lg">
        <div className="mb-4">
          <label className="block mb-2">Log Size</label>
          <input
            type="number"
            value={logSize}
            onChange={(e) => setLogSize(Number(e.target.value))}
            className="w-full bg-gray-800 text-green-400 px-3 py-2 rounded"
          />
        </div>

        <div className="mb-4">
          <label className="block mb-2">Claim</label>
          <input
            type="number"
            value={claim}
            onChange={(e) => setClaim(Number(e.target.value))}
            className="w-full bg-gray-800 text-green-400 px-3 py-2 rounded"
          />
        </div>

        <button
          onClick={submitJob}
          disabled={isLoading}
          className={`w-full bg-green-600 hover:bg-green-700 text-white font-bold py-2 px-4 rounded ${
            isLoading ? "opacity-50 cursor-not-allowed" : ""
          }`}
        >
          {isLoading ? "Processing..." : "Submit Job"}
        </button>
      </div>

      {jobId && (
        <div className="mt-8 text-center">
          <p>Job ID: {jobId}</p>
          <p>Status: {proofStatus}</p>
          {isLoading && <div className="spinner mt-4 mx-auto"></div>}
          {proof && (
            <div>
              <p className="mt-4">Proof received:</p>
              <pre className="bg-gray-800 p-4 rounded mt-2 overflow-x-auto">
                {proof}
              </pre>
              <button
                onClick={verifyProofHandler}
                disabled={isLoading}
                className={`mt-4 bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded ${
                  isLoading ? "opacity-50 cursor-not-allowed" : ""
                }`}
              >
                {isLoading ? "Verifying..." : "Verify Proof"}
              </button>
            </div>
          )}
        </div>
      )}
    </main>
  );
}
