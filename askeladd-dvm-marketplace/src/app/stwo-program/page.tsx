"use client";

import { useState, useEffect, useMemo } from "react";
import { NDKEvent, NDKKind } from '@nostr-dev-kit/ndk';
import { useNostrContext } from "@/context/NostrContext";
import { useSendNote } from "@/hooks/useSendNote";
import { ContractUploadType, IGenerateZKPRequestDVM, JobResultProver, KIND_JOB_REQUEST, KIND_JOB_RESULT, ProgramInternalContractName } from "@/types";
import init, { verify_stark_proof, prove_and_verify } from "../../pkg/stwo_wasm";
import { useFetchEvents } from "@/hooks/useFetchEvents";
import { ASKELADD_RELAY } from "@/constants/relay";
import { Relay } from 'nostr-tools/relay';
import { Event as EventNostr, SimplePool } from "nostr-tools";
import { PROGRAM_INTERAL_REQUEST } from "@/constants/program";
import ProgramCard from "../components/ProgramCard";
export default function StwoProgramMarketplace() {
  const [logSize, setLogSize] = useState<number>(5);
  const [claim, setClaim] = useState<number>(443693538);
  const [publicKey, setPublicKey] = useState<string | undefined>();
  const [jobId, setJobId] = useState<string | undefined>();
  const [error, setError] = useState<string | undefined>()
  const [starkProof, setStarkProof] = useState<any | undefined>()
  const [jobEventResult, setJobEventResult] = useState<EventNostr | undefined | NDKEvent>()
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

  const [internalProgram, setInternalProgram] = useState<IGenerateZKPRequestDVM[]>(PROGRAM_INTERAL_REQUEST)

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

  return (
    <main className="min-h-screen bg-black text-neon-green font-arcade p-4 pb-16 relative overflow-hidden">
      <div className="crt-overlay"></div>
      <div className="scanlines"></div>
      <div className="crt-curve"></div>

      <div className="arcade-cabinet">
        <h1 className="text-4xl mb-4 text-center glitch neon-text" data-text="Askeladd DVM Arcade">Askeladd DVM</h1>
        <p className="text-1xl mb-2 text-center glitch neon-text" data-text="Askeladd DVM Arcade">STWO ZK Program Marketplace</p>
        <p className="text-center blink neon-text-sm">Check the STWO Prover ready to use!</p>


        <div className="gap-3">      {internalProgram?.map((p, i) => {
          return (
            <ProgramCard key={i} program={p}></ProgramCard>
          )
        })}

        </div>

      </div>
      <div className="marquee">
        <span>Prove your claims and conquer the Nostr realm!</span>
      </div>
    </main>
  );
}
