"use client";

import { useState, useEffect } from "react";
import { IGenerateZKPRequestDVM } from "@/types";
import { PROGRAM_INTERAL_REQUEST } from "@/constants/program";
import ProgramCard from "../components/ProgramCard";
import InternalProgram from "../components/InternalProgram";
import { useDVMState } from "@/hooks/useDVMState";
export default function StwoProgramMarketplace() {
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [isFetchJob, setIsFetchJob] = useState(false);
  const [internalsPrograms, setInternalPrograms] = useState<IGenerateZKPRequestDVM[]>(PROGRAM_INTERAL_REQUEST)
  const {fetchEventsProof, jobId, jobEventResult, fetchPrograms, events, eventsPrograms} = useDVMState()
  /** Effect to fetch the job result when a job request is sent */
  const waitingForJobResult = async () => {
    if (jobEventResult && jobId) return;
    if(!jobId) return;
    fetchEventsProof()
    setIsLoading(false);
  }

  useEffect(() => {
    fetchPrograms()
    if (jobId && !jobEventResult) {
      waitingForJobResult()
    }
  }, [jobId, isFetchJob, jobEventResult])

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
        <div className="gap-3 flex flex-direction col-2 grid">      {internalsPrograms?.map((p, i) => {
          return (
            <InternalProgram key={i} zkp_request={p}></InternalProgram>
          )
        })}
        </div>
      </div>
      <button className="secondary-button" onClick={fetchPrograms}>Load programs</button>
      <div>
        <div className="grid gap-3 flex md:grid-flow-row">      {eventsPrograms?.map((e, i) => {
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
