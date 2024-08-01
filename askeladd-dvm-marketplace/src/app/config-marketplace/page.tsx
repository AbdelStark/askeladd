"use client";

import { useState, useEffect } from "react";
import { NDKEvent, NDKKind } from '@nostr-dev-kit/ndk';
import { useSendNote } from "@/hooks/useSendNote";
import { useFetchEvents } from "@/hooks/useFetchEvents";
import { APPLICATION_PUBKEY_DVM, ASKELADD_RELAY } from "@/constants/relay";
import { Event as EventNostr, SimplePool } from "nostr-tools";
import { ASKELADD_KINDS, ConfigHandle } from "@/types";
import EventCard from "../components/EventCard";
import { generateContentAndTags } from "../utils/generateAppHandler";

export default function Home() {
  const [publicKey, setPublicKey] = useState<string | undefined>();
  const [appKind, setAppKind] = useState<ASKELADD_KINDS | undefined>(ASKELADD_KINDS.KIND_JOB_REQUEST)
  const [configKind, setConfigKind] = useState<ConfigHandle>(ConfigHandle.ALL_KIND)
  const [jobId, setJobId] = useState<string | undefined>();
  const [error, setError] = useState<string | undefined>()
  const [lastConfig, setLastConfig] = useState<EventNostr | undefined | NDKEvent>()
  const [events, setEvents] = useState<EventNostr[] | NDKEvent[]>([])
  const [proofStatus, setProofStatus] = useState<
    "idle" | "pending" | "received" | "verified"
  >("idle");
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [isInitialized, setIsInitialized] = useState(false);
  const [openHowItWork, setOpenHowItWork] = useState(false);
  const [isNeedLoadEvents, setIsNeedLoadEvents] = useState(true);
  const [isAdmin, setIsAdmin] = useState(false);
  const [timestampJob, setTimestampJob] = useState<number | undefined>();
  const { fetchEvents, fetchEventsTools, setupSubscriptionNostr } = useFetchEvents()
  const { sendNote, publishNote } = useSendNote()

  useEffect(() => {
    if (isNeedLoadEvents || !isInitialized) {
      fetchEventsApp()
      setIsNeedLoadEvents(false)
      setIsInitialized(true)
    }
  }, [isNeedLoadEvents])

  useEffect(() => {

    if (publicKey) {

      if (process.env.NEXT_PUBLIC_DVM_PUBKEY && process.env.NEXT_PUBLIC_DVM_PUBKEY != publicKey) {
        setIsAdmin(true)
      }
    }
  }, [publicKey])

  const fetchEventsApp = async () => {
    console.log("fetch events config");
    const { events } = await fetchEventsTools({
      kinds: [NDKKind.AppHandler],
      limit: 100,
    })
    console.log("events config NIP-89", events);
    setLastConfig(events[0])
    setEvents(events);
    setIsNeedLoadEvents(false)
  }


  /** Connect you */
  const connectExtension = async () => {
    try {

      if (typeof window !== "undefined" && window.nostr) {
        const pubkey = await window.nostr.getPublicKey();
        let created_at = new Date().getTime();
        setPublicKey(pubkey)
      }

    } catch (e) {
      console.log("connect extension error", e)
    } finally {
      setIsLoading(false);
    }

  };

  /** Submit Recommended App Handler for:
   * JOB_REQUEST 5600: Prove
   * JOB_RESULT 6600: Result
  */
  const submitJob = async () => {
    try {
      setIsLoading(true);
      setJobId(undefined)
      setProofStatus("pending");
      setError(undefined);

      submitApplicationHandler()
      setIsNeedLoadEvents(true)

    } catch (e) {
    } finally {
      setIsLoading(false);

    }

  };

  const submitApplicationHandler = async () => {
    try {
      setIsLoading(true);
      setProofStatus("pending");
      setLastConfig(undefined);
      setError(undefined);

      setTimestampJob(new Date().getTime())
      /** Use Nostr extension to send event */
      const pool = new SimplePool();
      let pubkey;
      if (typeof window !== "undefined" && window.nostr) {
        console.log("pubkey", pubkey)
        if (!pubkey) return;
        const { tags, content } = generateContentAndTags(configKind, appKind, pubkey)
        console.log("tags", tags)
        console.log("content", content)
        if (!content || !tags) return;

        let created_at = new Date().getTime();
        const event = await window.nostr.signEvent({
          pubkey: publicKey ?? pubkey,
          created_at: created_at,
          kind: NDKKind.AppHandler,
          tags: tags,
          content: content
        }) // takes an event object, adds `id`, `pubkey` and `sig` and returns it
        // Setup job request to fetch job id

        // let eventID = await relay.publish(event as EventNostr);
        const eventID = await Promise.any(pool.publish(ASKELADD_RELAY, event as EventNostr));
        console.log("eventID", eventID[0])
        setIsNeedLoadEvents(true)

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

  const handleSetAppKindSpecific = (kind: ASKELADD_KINDS, config: ConfigHandle) => {
    if (kind) {
      setAppKind(kind)
      setConfigKind(ConfigHandle.SPECIFIC_KIND);
    }
    if (config == ConfigHandle?.ALL_KIND) {
      setConfigKind(config);
      setAppKind(undefined)
    }
  }

  return (
    <main className="min-h-screen bg-black text-neon-green font-arcade p-4 pb-16 overflow-hidden">
      <div className="crt-overlay"></div>
      <div className="scanlines"></div>
      <div className="crt-curve"></div>
      <button
        onClick={connectExtension}
        disabled={isLoading}
        className="mb-5 m-5"
      >
        {isLoading ? "PROCESSING..." : "CONNECT"}
      </button>

      {publicKey && publicKey != APPLICATION_PUBKEY_DVM ?
        <div>
          <div className="arcade-cabinet">
            <div className="cabinet-content">
              <div>
                {publicKey &&
                  <p className="mb-5">Connected: {publicKey}</p>}
                <p className="mb-5 text-white">You can't config this if you are not an admin of ASKELADD</p>
              </div>
            </div>
          </div>
        </div>

        : publicKey && publicKey == APPLICATION_PUBKEY_DVM &&
        <div className="arcade-cabinet">
          <div className="cabinet-content">
            <h1 className="text-4xl mb-4 text-center glitch neon-text" data-text="Askeladd DVM Arcade NIP-89">Askeladd DVM Marketplace</h1>
            <div>
              {publicKey == APPLICATION_PUBKEY_DVM &&
                <p> DVM admin</p>
              }
              {publicKey &&
                <p>Connected: {publicKey}</p>}
            </div>

            <div className="max-w-md mx-auto bg-dark-purple p-6 rounded-lg shadow-neon mt-8 relative game-screen">

              <label className="block mb-2 text-neon-pink">Form config: </label>
              <p>Selected config: {configKind == ConfigHandle?.ALL_KIND ? "All" : "Specific"}</p>
              <p className="block font-bold py-2 text-white">Change your config</p>
              <button className="block mb-5 font-bold py-2 px-4 rounded bg-blue-500 hover:bg-blue-700 text-white" onClick={() => setConfigKind(configKind == ConfigHandle.ALL_KIND ? ConfigHandle.SPECIFIC_KIND : ConfigHandle.ALL_KIND)}>{configKind == ConfigHandle?.ALL_KIND ? "Specific" : "All"}</button>

              {configKind == ConfigHandle.SPECIFIC_KIND &&
                <>
                  <label className="block mb-2 text-neon-pink">KIND to add metadata</label>

                  <p>Selected event: {appKind}</p>
                  <div className="mb-4 flex">
                    <button
                      className={appKind == ASKELADD_KINDS.KIND_JOB_REQUEST && configKind == ConfigHandle.SPECIFIC_KIND ? "bg-green" : "bg"}
                      onClick={() => handleSetAppKindSpecific(ASKELADD_KINDS.KIND_JOB_REQUEST, ConfigHandle.SPECIFIC_KIND)}>ZK Job request</button>
                    <button
                      className={appKind == ASKELADD_KINDS.KIND_JOB_RESULT && configKind == ConfigHandle.SPECIFIC_KIND ? "bg-green" : "bg"}
                      onClick={() => handleSetAppKindSpecific(ASKELADD_KINDS.KIND_JOB_RESULT, ConfigHandle.SPECIFIC_KIND)}>ZK Job result</button>
                  </div>
                </>
              }
              <button
                onClick={submitJob}
                disabled={isLoading}
                className={`submit-job-button ${isLoading ? "opacity-50 cursor-not-allowed" : ""
                  }`}
              >
                {isLoading ? "PROCESSING..." : "Submit Application"}
              </button>
            </div>
            {isLoading && <div className="pixel-spinner mt-4 mx-auto"></div>}
          </div>

        </div>
      }



      <button
        className={`block mb-5 font-bold py-2 px-4 rounded bg-blue-500 hover:bg-blue-700 ${isLoading ? "opacity-50 cursor-not-allowed" : ""}`}
        disabled={isLoading}
        onClick={fetchEventsApp}>{events && events?.length ? "Refresh" : "Load"}</button>


      <div onClick={() => setOpenHowItWork(!openHowItWork)}
        className="cursor-pointer my-5 p-5"
      >
        <p className="text-white">How the ASKELADD DVM ZK works?</p>
        {!openHowItWork &&
          <button> Open </button>
        }
        {openHowItWork &&
          <>
            <div>
              <p>As an User  </p>
              <p className="text-white">User send a JOB_REQUEST with different params on the Nostr event:</p>
              <p className="text-white">You need theses params on the Nostr event:</p>
              <p className="text-white">It can change with all STWO Prover enabled on the Marketplace</p>
              <p className="text-white">Request: {JSON.stringify({
                "claim": "413300",
                "log_size":"5"
              })} // The input of the Program</p>
              <p className="text-white ">Tags: ["param", "input_name", "value"] </p>
            </div>
            <button> Close </button>
          </>
        }
      </div>

      <div className="text-left max-w-md">

        <p className="text-center mb-5"> Existing Config of DVM</p>
        <p className="text-center text-white mb-1">Last config</p>
        {lastConfig &&
          <>
            <EventCard event={lastConfig}></EventCard>
          </>
        }

        <p className="block mb-1 font-bold py-2 px-4 "> By Event KIND enabled on ASKELADD (WIP)</p>
        <p className="block mb-5 text-md text-white px-4">  (5600, 6600 and more soon)</p>

        <p className="block mb-5 font-bold text-white px-4"> Admin config (WIP)</p>
        <p className="block mb-5 text-md text-white px-4">TODO</p>

        <p className="block mb-5 font-bold py-2 px-4"> Others config</p>

        <div className="grid sm:grid-cols-1 grid-cols-2 md:grid-cols-3 gap-4">{events?.map((event, i) => {
          return (
            <EventCard key={i} event={event}></EventCard>
          )

        })}
        </div>
      </div>


      <div className="marquee">
      </div>
    </main>
  );
}
