"use client";

import { useState, useEffect, useMemo } from "react";
import { NDKEvent, NDKKind } from '@nostr-dev-kit/ndk';
import { useNostrContext } from "@/context/NostrContext";
import { useSendNote } from "@/hooks/useSendNote";
import { ContractUploadType, IGenerateZKPRequestDVM, JobResultProver, KIND_JOB_ADD_PROGRAM, KIND_JOB_REQUEST, KIND_JOB_RESULT, ProgramInternalContractName } from "@/types";
import init, { verify_stark_proof, prove_and_verify } from "../../pkg/stwo_wasm";
import { useFetchEvents } from "@/hooks/useFetchEvents";
import { ASKELADD_RELAY } from "@/constants/relay";
import { Relay } from 'nostr-tools/relay';
import { Event as EventNostr, SimplePool } from "nostr-tools";
import { PROGRAM_INTERAL_REQUEST } from "@/constants/program";
import ProgramCard from "../components/ProgramCard";
import InternalProgram from "../components/InternalProgram";
import { useDVMState } from "@/hooks/useDVMState";
export default function StwoProgramMarketplace() {
  const [jobEventResult, setJobEventResult] = useState<EventNostr | undefined | NDKEvent>()
  const [events, setEvents] = useState<EventNostr[] | NDKEvent[]>([])
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [isInitialized, setIsInitialized] = useState(false);
  const [isFetchJob, setIsFetchJob] = useState(false);
  const [isLoadingJobResult, setIsLoadingJobResult] = useState(false);
  const [isWaitingJob, setIsWaitingJob] = useState(false);

  const [internalProgram, setInternalProgram] = useState<IGenerateZKPRequestDVM[]>(PROGRAM_INTERAL_REQUEST)

  const { ndk, pool } = useNostrContext()
  const { fetchEvents, fetchEventsTools, setupSubscriptionNostr } = useFetchEvents()
  const {fetchEventsProof, jobId} = useDVMState()
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
    if(!jobId) return;
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
    fetchPrograms()
    if (jobId && !jobEventResult) {
      waitingForJobResult()
    }
  }, [jobId, isFetchJob, jobEventResult])



  const fetchPrograms = async () => {
    console.log("fetch events program")
    // if(jobEventResult && jobId)return;
    setIsFetchJob(false);
    setIsLoadingJobResult(true);
    const { events } = await fetchEvents({
      kind: KIND_JOB_ADD_PROGRAM,
      // kinds:[KIND_JOB_ADD_PROGRAM as NDKKind]
      // since: timestampJob,
      // search: jobId
      // search: `#${jobId}`,
    })
    console.log("events job program", events);
    setEvents(events)
    if (!events) return;
    let lastEvent = events[events?.length - 1]
    if (!lastEvent) return;

  }



  return (
    <main className="min-h-screen bg-black text-neon-green font-arcade p-4 pb-16 relative overflow-hidden">
      <div className="crt-overlay"></div>
      <div className="scanlines"></div>
      <div className="crt-curve"></div>
      <button className="secondary-button" onClick={fetchPrograms}>Load programs</button>

      <div className="arcade-cabinet">
        <h1 className="text-4xl mb-4 text-center glitch neon-text" data-text="Askeladd DVM Arcade">Askeladd DVM</h1>
        <p className="text-1xl mb-2 text-center glitch neon-text" data-text="Askeladd DVM Arcade">STWO ZK Program Marketplace</p>
        <p className="text-center blink neon-text-sm">Check the STWO Prover ready to use!</p>


        <div className="gap-3 flex flex-direction col-2 grid">      {internalProgram?.map((p, i) => {
          return (
            <InternalProgram key={i} zkp_request={p}></InternalProgram>
          )
        })}

        </div>
      </div>
      <button className="secondary-button" onClick={fetchPrograms}>Load programs</button>
      <div>
        <div className="grid gap-3 flex md:grid-flow-row">      {events?.map((e, i) => {
          const p: IGenerateZKPRequestDVM = JSON.parse(e.content)
          return (
            <ProgramCard key={i} zkp_request={p}></ProgramCard>
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
