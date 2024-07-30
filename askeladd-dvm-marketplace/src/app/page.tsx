"use client";

import { useState, useEffect } from "react";
import { NDKEvent} from '@nostr-dev-kit/ndk';
import { useNostrContext } from "@/context/NostrContext";
import { useSendNote } from "@/hooks/useSendNote";
import { JobResultProver, StarkProof } from "@/types";
import init, { run_fibonacci_example, run_fibonacci_verify_exemple } from "../pkg/program_wasm";
import { useFetchEvents } from "@/hooks/useFetchEvents";
import { ASKELADD_RELAY } from "@/constants/relay";
import { Relay } from 'nostr-tools/relay'
import { verifyEvent, finalizeEvent, Event as EventNostr } from "nostr-tools";
export default function Home() {
  const [logSize, setLogSize] = useState<number>(5);
  const [claim, setClaim] = useState<number>(443693538);
  const [jobId, setJobId] = useState<string | null>(null);
  const [error, setError] = useState<string | undefined>()
  const [starkProof, setStarkProof] = useState<any | undefined>()
  // const [starkProof, setStarkProof] = useState<StarkProof | undefined>()
  const [events, setEvents] = useState<NDKEvent[]>([])
  const [selectedEvent, setSelectedEvent] = useState<NDKEvent | undefined>()
  const [proofStatus, setProofStatus] = useState<
    "idle" | "pending" | "received" | "verified"
  >("idle");
  const [proof, setProof] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [isInitialized, setIsInitialized] = useState(false);
  const [isFetchJob, setIsFetchJob] = useState(false);
  const [timestampJob, setTimestampJob] = useState<number | undefined>();

  const { ndk } = useNostrContext()
  const { fetchEvents } = useFetchEvents()
  const { sendNote, publishNote } = useSendNote()
  useEffect(() => {
    init()
      .then(() => setIsInitialized(true))
      .catch((error) => {
        console.error("Failed to initialize WASM module:", error);

      });
  }, []);

  useEffect(() => {

    if (jobId && !isFetchJob) {
      waitingForJobResult()
    }
  }, [jobId, isFetchJob])

  const submitJob = async () => {
    setIsLoading(true);
    setProofStatus("pending");

    // Mock event id
    const eventId = Math.random().toString(36).substring(7);
    setJobId(eventId);

    const tags = [
      ['param', 'log_size', logSize.toString()],
      ['param', 'claim', claim.toString()],
      ['output', 'text/json']
    ];

    const content = JSON.stringify({
      request: {
        log_size: logSize.toString(),
        claim: claim.toString()
      }

    })

    // Define the timestamp before which you want to fetch events
    setTimestampJob(new Date().getTime())
    /** Use Nostr extension to send event */
    if (typeof window !== "undefined" && window.nostr) {
      const pubkey = await window.nostr.getPublicKey();
      let created_at = new Date().getTime();
      const event = await window.nostr.signEvent({
        pubkey: pubkey,
        created_at: created_at,
        kind: 5600,
        tags: tags,
        content: content
      }) // takes an event object, adds `id`, `pubkey` and `sig` and returns it
      if (event?.sig) {
        setJobId(event?.sig);
      }
      const relay = await Relay.connect(ASKELADD_RELAY[0])
      let eventID = await relay.publish(event as EventNostr)
      console.log("eventID published", eventID)
      if (eventID) {
        setJobId(eventID);
      }

      /** check if events is sent */
      const { events } = await fetchEvents(5600);
      console.log("events job request", events);


    }
    // let { result, event } = await sendNote({ content, tags, kind: 5600 })
    // console.log("event", event)
    // if (event?.sig) {
    //   setJobId(event?.sig);
    // }
    /** NDK event
     * Generate or import private key after
     */

  };

  /** TODO fetch subscribed event */
  // Fetch Job result from the Prover
  const fetchEventsProof = async () => {

    // ndk.subscribe({ kinds: [0], authors: ["pubkey-2"] });

    setIsFetchJob(false)

    const { events } = await fetchEvents()
    if (!events) return;
    console.log("events", events);
    setEvents(events)
    /** @TODO fetch the correct event
     * - Tags: By reply of the event_id of the job request?
     * - By author
     * - Timestamp since/until (doesn't work as expected for me)
     */
    let lastEvent = events[events?.length - 1]
    // let lastEvent= events.find((e) => e?.id == "48b273cee7d08538604f1797c92685a4638d53a8fea56ff9fe48a436ad4a2e73")
    if (!lastEvent) return;
    setSelectedEvent(lastEvent)
    setProof(lastEvent?.content)

    const jobProofSerialize: JobResultProver = JSON.parse(lastEvent?.content)
    console.log('jobProofSerialize serialize', jobProofSerialize);

    const proofSerialize = jobProofSerialize?.response?.proof;
    console.log('proof serialize', proofSerialize);
    setStarkProof(proofSerialize);
    setProofStatus("received");
  }

  const waitingForJobResult = async () => {
    setTimeout(() => {
      console.log("waiting timeout")
      fetchEventsProof()
      setIsLoading(false);

    }, 5000);
  }

  const verifyProofHandler = async () => {
    try {
      if (proof) {
        setIsLoading(true);
        const prove_result = run_fibonacci_example(logSize, claim);
        console.log("prove_result", prove_result);
        const verify_result = run_fibonacci_verify_exemple(logSize, claim, JSON.stringify(starkProof));
        console.log("verify result", verify_result);
        console.log("verify message", verify_result.message);
        console.log("verify success", verify_result.success);

        if (verify_result?.success) {
          console.log("is success verify result")
          setProofStatus("verified");
        } else {
          setError(verify_result?.message)
        }

        /** @TODO fix ERROR verify loop between all stark proof*/
        for (let event of events) {
          const jobProofSerialize: JobResultProver = JSON.parse(event?.content)
          const proofSerialize = jobProofSerialize?.response?.proof;
          const verify_result = run_fibonacci_verify_exemple(logSize, claim, JSON.stringify(proofSerialize));
          console.log("loop verify result", verify_result.message);
          console.log("loop verify success", verify_result.success);
          if (verify_result?.success) {
            console.log("is success verify result")
            setProofStatus("verified");
          } else {
            setError(verify_result?.message)
          }
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
          className={`w-full bg-green-600 hover:bg-green-700 text-white font-bold py-2 px-4 rounded ${isLoading ? "opacity-50 cursor-not-allowed" : ""
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

          {error && <p>Error: {error}</p>}
          {proof && (
            <div>
              <p className="mt-4">Proof received:</p>
              <pre className="bg-gray-800 p-4 rounded mt-2 overflow-x-auto">
                {proof}
              </pre>
              {starkProof &&

                <>
                  <p>Proof of work nonce: {starkProof?.commitment_scheme_proof?.proof_of_work?.nonce}</p>
                </>

              }
              <button
                onClick={verifyProofHandler}
                disabled={isLoading}
                className={`mt-4 bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded ${isLoading ? "opacity-50 cursor-not-allowed" : ""
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
