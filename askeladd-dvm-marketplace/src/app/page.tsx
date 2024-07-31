"use client";

import { useState, useEffect, useMemo } from "react";
import { NDKEvent, NDKKind } from '@nostr-dev-kit/ndk';
import { useNostrContext } from "@/context/NostrContext";
import { useSendNote } from "@/hooks/useSendNote";
import { JobResultProver, KIND_JOB_REQUEST, KIND_JOB_RESULT } from "@/types";
import init, { verify_stark_proof, prove_and_verify } from "../pkg/stwo_wasm";
import { useFetchEvents } from "@/hooks/useFetchEvents";
import { ASKELADD_RELAY } from "@/constants/relay";
import { Relay } from 'nostr-tools/relay';
import { Event as EventNostr, SimplePool } from "nostr-tools";
export default function Home() {
  const [logSize, setLogSize] = useState<number>(5);
  const [claim, setClaim] = useState<number>(443693538);
  const [publicKey, setPublicKey] = useState<string | undefined>();
  const [jobId, setJobId] = useState<string | undefined>();
  // const [jobId, setJobId] = useState<string | undefined>("78e3026c35d08ab8345b4efa49e0fe27c74f3849589720e01286cda69c36cc39");
  // Event ID test : "f708c6ba3c078a364ef7d5222310c14288841a63956b10186959b48e3284c4bb"
  // 191ade3aa99bdbb7d6781e1149cf0ec4205db1ac097df9f83a6d7a10d88712c0
  const [error, setError] = useState<string | undefined>()
  const [starkProof, setStarkProof] = useState<any | undefined>()
  const [jobEventResult, setJobEventResult] = useState<EventNostr | undefined | NDKEvent>()
  // const [starkProof, setStarkProof] = useState<StarkProof | undefined>()
  const [events, setEvents] = useState<EventNostr[] | NDKEvent[]>([])
  const [selectedEvent, setSelectedEvent] = useState<EventNostr | undefined | NDKEvent>()
  const [proofStatus, setProofStatus] = useState<
    "idle" | "pending" | "received" | "verified"
  >("idle");
  const [proof, setProof] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [isInitialized, setIsInitialized] = useState(false);
  const [isFetchJob, setIsFetchJob] = useState(false);
  const [isLoadingJobResult, setIsLoadingJobResult] = useState(false);
  const [isWaitingJob, setIsWaitingJob] = useState(false);
  const [timestampJob, setTimestampJob] = useState<number | undefined>();

  let eventIdRequest = useMemo(() => {
    return jobId
  }, [jobId])
  const { ndk, pool } = useNostrContext()
  const { fetchEvents, fetchEventsTools, setupSubscriptionNostr } = useFetchEvents()
  const { sendNote, publishNote } = useSendNote()

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
    }
  }, [jobId, isFetchJob, jobEventResult])

  const runSubscriptionEvent = (pool: SimplePool, pubkey?: string) => {
    let poolRequest = pool.subscribeMany(
      ASKELADD_RELAY,
      [
        {
          kinds: [KIND_JOB_REQUEST as NDKKind],
          // since:timestampJob
          // authors: pubkey ? [pubkey] : []
        },
        {
          kinds: [KIND_JOB_RESULT as NDKKind],
          // since:timestampJob
        },
      ],
      {
        onevent(event) {
          if (event?.kind == KIND_JOB_REQUEST) {
            console.log("Event job request received: ", event?.id);
            if (!jobId) return;
            if (pubkey && event?.pubkey == pubkey) {
              setJobId(event?.id)
            }
            poolRequest.close();

          }
          if (event?.kind == KIND_JOB_RESULT) {
            console.log("Event job request received: ", event?.id);
            if (!jobId) return;
            if (pubkey && event?.pubkey == pubkey) {
              setJobId(event?.id)
            }
            poolRequest.close();
          }
        },
        onclose: () => {
          poolRequest.close()
        },
        oneose() {
          poolRequest.close()
        }
      }
    )
  }

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
      setProof(null);
      setJobEventResult(undefined);
      setError(undefined);
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
      // setTimestampJob(new Date().getTime() / 1000)
      setTimestampJob(new Date().getTime())
      /** Use Nostr extension to send event */
      const pool = new SimplePool();
      const poolJob = new SimplePool();
      const relay = await Relay.connect(ASKELADD_RELAY[0])
      if (typeof window !== "undefined" && window.nostr) {
        const pubkey = await window.nostr.getPublicKey();
        let created_at = new Date().getTime();
        setPublicKey(pubkey)
        const event = await window.nostr.signEvent({
          pubkey: pubkey,
          created_at: created_at,
          kind: 5600,
          tags: tags,
          content: content
        }) // takes an event object, adds `id`, `pubkey` and `sig` and returns it
        // Setup job request to fetch job id

        /** @TODO why the event id is not return?
         * - get the last event and fetch job_id event
         * - check if events is sent with subscription
         * 
        */
        // let eventID = await relay.publish(event as EventNostr);
        const eventID = await Promise.any(pool.publish(ASKELADD_RELAY, event as EventNostr));
        console.log("eventID", eventID[0])
        await fetchJobRequest(pubkey)
        setIsWaitingJob(true);
        await timeoutWaitingForJobResult()

      } else {

        /** @TODO flow is user doesn't have NIP-07 extension */
        // let { result, event } = await sendNote({ content, tags, kind: 5600 })
        // console.log("event", event)
        // if (event?.sig) {
        //   setJobId(event?.sig);
        // }
        // setIsWaitingJob(true)
        /** NDK event
         * Generate or import private key after
         */
      }
    } catch (e) {
    } finally {
      setIsLoading(false);
    }

  };

  /** TODO fetch subscribed event
 * fix search jobId => check if relayer support NIP-50 
 * Fetch Job result from the Prover
 * - Tags: By reply of the event_id of the job request?
 * - By author
 * - Timestamp since/until (doesn't work as expected for me)
*/
  const fetchJobRequest = async (pubkey?: string) => {

    const { events } = await fetchEventsTools({
      kind: KIND_JOB_REQUEST,
      since: timestampJob,
      // authors: pubkey ? [pubkey] : []
    });
    console.log("events job request", events);
    if (!events) return;
    // const lastEvent = events[events?.length - 1]
    const lastEvent = events[0]
    if (!lastEvent?.id) return;
    const lastEventId = lastEvent?.id;
    if (pubkey && pubkey == lastEvent?.pubkey) {
      console.log("lastEventId", lastEventId)
      setJobId(lastEventId);
      eventIdRequest = lastEventId;
      setIsWaitingJob(true)
    }

  }


  /** TODO fetch subscribed event
    * fix search jobId => check if relayer support NIP-50 
    * Fetch Job result from the Prover
     * - Tags: By reply of the event_id of the job request?
     * - By author
     * - Timestamp since/until (doesn't work as expected for me)
     */
  const fetchEventsProof = async () => {
    console.log("fetch events job result proof")
    // if(jobEventResult && jobId)return;
    setIsFetchJob(false);
    setIsLoadingJobResult(true);
    const { events } = await fetchEventsTools({
      kind: KIND_JOB_RESULT,
      // since: timestampJob,
      // search: jobId
      // search: `#${jobId}`,
    })
    console.log("events job result", events);
    if (!events) return;
    let lastEvent = events[events?.length - 1]
    if (!lastEvent) return;
    let id = jobId ?? eventIdRequest;
    if (jobEventResult && jobEventResult?.id == id && proof && proofStatus != "pending") return;
    if (id && !jobEventResult) {
      let jobEventResultFind = events?.find((e) => e?.content?.includes(id))
      console.log("jobEventResultFind", jobEventResultFind);
      let filterJob = events?.filter((e) => e?.id?.includes(id))
      console.log("filterJob", filterJob);
      if (jobEventResultFind?.id) {
        console.log("Event JOB_RESULT find", jobEventResultFind);
        getDataOfEvent(jobEventResultFind);
        setJobEventResult(jobEventResultFind)
      }
    }
  }

  const getDataOfEvent = (lastEvent?: NDKEvent | EventNostr) => {
    if (!lastEvent || !lastEvent?.content) return;
    setSelectedEvent(lastEvent);
    setProof(lastEvent?.content?.toString())
    const jobProofSerialize: any = JSON.parse(lastEvent?.content)
    console.log('jobProofSerialize serialize', jobProofSerialize);
    const proofSerialize = jobProofSerialize?.response?.proof;
    console.log('proof serialize', proofSerialize);
    setStarkProof(proofSerialize);
    setProofStatus("received");
    return proofSerialize
  }

  const verifyProofHandler = async () => {
    try {
      if (proof) {
        setIsLoading(true);
        const prove_result = prove_and_verify(logSize, claim);
        console.log("prove_result", prove_result);
        const serialised_proof_from_nostr_event = JSON.stringify(starkProof);
        console.log("serialised_proof_from_nostr_event", serialised_proof_from_nostr_event);
        const verify_result = verify_stark_proof(logSize, claim, serialised_proof_from_nostr_event);
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
          const verify_result = verify_stark_proof(logSize, claim, JSON.stringify(proofSerialize));
          if (verify_result?.success) {
            console.log("loop verify result", verify_result.message);
            console.log("loop verify success", verify_result.success);
            console.log("is success verify result")
            setProofStatus("verified");
          } else {
            // setError(verify_result?.message)
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
    <main className="min-h-screen bg-black text-neon-green font-arcade p-4 pb-16 relative overflow-hidden">
      <div className="crt-overlay"></div>
      <div className="scanlines"></div>
      <div className="arcade-cabinet">
        <div className="cabinet-content">
          <h1 className="text-4xl mb-4 text-center glitch neon-text" data-text="Askeladd DVM Arcade">Askeladd DVM Marketplace</h1>
          <p className="text-center blink neon-text-sm">Censorship global proving network</p>
          <p className="text-center blink neon-text-sm">Powered by Nostr and Circle STARKs.</p>
          <div className="max-w-md mx-auto bg-dark-purple p-6 rounded-lg shadow-neon mt-8 relative game-screen">
            <div className="mb-4">
              <label className="block mb-2 text-neon-pink">Log Size</label>
              <input
                type="number"
                value={logSize}
                onChange={(e) => setLogSize(Number(e.target.value))}
                className="w-full bg-black text-neon-green px-3 py-2 rounded border-neon-green border-2"
              />
            </div>

            <div className="mb-4">
              <label className="block mb-2 text-neon-pink">Claim</label>
              <input
                type="number"
                value={claim}
                onChange={(e) => setClaim(Number(e.target.value))}
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
              <p className="text-neon-orange">Job ID: <span className="break-all">{jobId}</span></p>
              <p className="text-neon-yellow">Status: {proofStatus}</p>

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
        <span>Welcome to Askeladd DVM Marketplace! Prove your claims and conquer the blockchain realm!</span>
      </div>
    </main>
  );
}
