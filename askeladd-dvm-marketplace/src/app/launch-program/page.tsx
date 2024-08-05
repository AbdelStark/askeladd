"use client";

import { useState, useEffect } from "react";
import { NDKEvent, NDKKind } from '@nostr-dev-kit/ndk';
import { useSendNote } from "@/hooks/useSendNote";
import { useFetchEvents } from "@/hooks/useFetchEvents";
import { APPLICATION_PUBKEY_DVM, ASKELADD_RELAY } from "@/constants/relay";
import { Event as EventNostr, SimplePool } from "nostr-tools";
import { ASKELADD_KINDS, ConfigHandle, IGenerateZKPRequestDVM, IProgramParams } from "@/types";
import EventCard from "../components/EventCard";
import { generateContentAndTags } from "../utils/generateAppHandler";
import { HowItWork } from "../components/description";
import { PROGRAM_INTERAL_REQUEST } from "@/constants/program";

export default function LaunchProgram() {
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
  const [isNeedLoadEvents, setIsNeedLoadEvents] = useState(true);
  const [isAdmin, setIsAdmin] = useState(false);
  const [timestampJob, setTimestampJob] = useState<number | undefined>();
  const { fetchEvents, fetchEventsTools, setupSubscriptionNostr } = useFetchEvents()
  const { sendNote, publishNote } = useSendNote()
  const [logSize, setLogSize] = useState<number>(5);
  const [claim, setClaim] = useState<number>(443693538);
  const [inputIndex, setInputsIndex] = useState(0)
  const [isOpenForm, setIsOpenForm] = useState(false)
  const [form, setForm] = useState({})
  const [formType, setFormType] = useState({})
  const [formEncrypted, setFormEncrypted] = useState({})
  const [programParam, setProgramParam] = useState<IProgramParams>({
    pubkey_app: undefined,
    event_id: undefined,
    unique_id: undefined,
    inputs: new Map<string, string>(),
    contract_name: undefined,
    contract_reached: undefined,
    internal_contract_name: undefined
  })
  // Handle changes in form inputs
  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setForm(prev => ({
      ...prev,
      [name]: value
    }));

    // setFormEncrypted(prev => ({
    //   ...prev,
    //   [value]: false
    // }));

    setFormType(prev => ({
      ...prev,
      [name]: "String"
    }));
    console.log("form", form)
    console.log("form encrypted", formEncrypted)
    console.log("form type", formType)
  };

  // Handle changes in form inputs
  const handleInputType = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setFormType(prev => ({
      ...prev,
      [name]: value
    }));
    console.log("form type", form)
  };

  // Handle changes in form inputs
  const handleInputEncrypted = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setFormEncrypted(prev => ({
      ...prev,
      [name]: value
    }));
    console.log("formEncrypted", formEncrypted)
  };

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
   * JOB_REQUEST 5700: Laucn hrpgraom
   * JOB_RESULT 6600: Result
  */
  const submitJob = async () => {
    try {
      setIsLoading(true);
      setJobId(undefined)
      setProofStatus("pending");
      setError(undefined);

      submitProgram()
      setIsNeedLoadEvents(true)

    } catch (e) {
    } finally {
      setIsLoading(false);

    }

  };

  const uploadWasm = async () => {
    try {
      setIsLoading(true);
      setJobId(undefined)
      setProofStatus("pending");
      setError(undefined);
    } catch (e) {
    } finally {
      setIsLoading(false);

    }

  };

  const mockProgram = async () => {
    /** Todo better check */
    if (!isLoading && !isOpenForm && Object.entries(form).length == 0) return;
    setIsLoading(true);
    setJobId(undefined)
    setProofStatus("pending");
    setError(undefined);
    const tags = [
      ['param', 'log_size', logSize.toString()],
      ['param', 'claim', claim.toString()],
      ['output', 'text/json']
    ];

  }
  const submitProgram = async () => {
    try {
      setIsLoading(true);
      setProofStatus("pending");
      setLastConfig(undefined);
      setError(undefined);
      console.log("formEncrypted", formEncrypted)

      let tags: string[][] = []
      const inputs: Map<string, string> = new Map<string, string>();
      {
        Object.entries(form).map(([key, value]) => {
          inputs.set(key, value as string)
        }
        )
      }
      for (let [key, value] of inputs) {
        tags.push(["param", key, value])
      }

      const inputs_encrypted: Map<string, string> = new Map<string, string>();
      Object.entries(formEncrypted).map(([key, value]) => {
        inputs_encrypted.set(key, value as string)
      }
      )
      for (let [key, value] of inputs_encrypted) {
        tags.push(["param_encrypted", key, value])
      }
      console.log("inputs_encrypted", Object.fromEntries(inputs_encrypted))

      // const inputs_types: Map<string, string> = new Map<string, string>();
      // {
      //   Object.entries(formType).map(([key, value]) => {
      //     inputs_types.set(key, value as string)
      //   }
      //   )
      // }
      for (let [key, value] of inputs_encrypted) {
        tags.push(["param_encrypted", key, value])
      }
      const content = JSON.stringify({
        request: form,
        program: {
          contract_name: programParam?.contract_name,
          internal_contract_name: programParam?.internal_contract_name,
          contract_reached: programParam?.contract_reached,
          inputs: Object.fromEntries(inputs),
          inputs_types: undefined,
          inputs_encrypted: Object.fromEntries(inputs_encrypted)
        }
      })

      console.log("tags", tags)
      console.log("content", content)
      setTimestampJob(new Date().getTime())
      /** Use Nostr extension to send event */
      const pool = new SimplePool();
      let pubkey;
      if (typeof window !== "undefined" && window.nostr) {
        console.log("pubkey", pubkey)
        // await connectExtension()
        if (!publicKey) return;
        if (!content) return;

        // let created_at = new Date().getTime();
        // const event = await window.nostr.signEvent({
        //   pubkey: publicKey ?? pubkey,
        //   created_at: created_at,
        //   kind: NDKKind.AppHandler,
        //   tags: tags,
        //   content: content
        // }) // takes an event object, adds `id`, `pubkey` and `sig` and returns it
        // // Setup job request to fetch job id

        // // let eventID = await relay.publish(event as EventNostr);
        // const eventID = await Promise.any(pool.publish(ASKELADD_RELAY, event as EventNostr));
        // console.log("eventID", eventID[0])
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

  const handleLoadFormEncrypted = () => {
    console.log("form load key")
    Object.entries(form).map(([key, value]) => {
      setFormEncrypted({ ...formEncrypted, [value as string]: true })
    }
    )

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


      <div className="arcade-cabinet">
        <h1 className="text-4xl mb-4 text-center glitch neon-text" data-text="Askeladd DVM Arcade NIP-89">Askeladd DVM</h1>
        <p className="text-2xl mb-4 text-center glitch neon-text" data-text="Askeladd DVM Arcade NIP-89">Launch program</p>

        <div className="max-w-md mx-auto bg-dark-purple p-6 rounded-lg shadow-neon mt-8 relative game-screen">
          <div>
            <p>Program param</p>
            <input
              placeholder="Pubkey"
              className='text-black'
              name={"pubkey_app"}
              onChange={(e) => {
                programParam.pubkey_app = e.target.value
              }}
            ></input>

            <div className="max-w-md mx-auto bg-dark-purple p-6 rounded-lg shadow-neon mt-8 relative game-screen">
              <p>Inputs</p>
              {form && Object.entries(form).map(([key, value], i) => {
                return (
                  <div key={i}>
                    <p >{`${key}`}</p>
                    <p>{`Name: ${value}`}</p>
                    <input
                      className='text-black'
                      placeholder="Name of your input"
                      name={key}
                      onChange={handleChange}
                    ></input>
                  </div>
                )
              })}
              <button
                className="bg-blue border border-r-3 secondary-button w-full"
                onClick={() => {
                  setInputsIndex(inputIndex + 1);
                  setForm({...form, [inputIndex+1]:inputIndex+1})
                  // form[String(inputIndex + 1).toString()] = (inputIndex + 1).toString()
                }}
              >
                Add input
              </button>
            </div>

            <div className="max-w-md mx-auto bg-dark-purple p-6 rounded-lg shadow-neon mt-8 relative game-screen">
              <p>Inputs encrypted</p>

              <button onClick={handleLoadFormEncrypted}> Load inputs to continue settings</button>
              {formEncrypted && Object.entries(formEncrypted).map(([key, value], i) => {
                return (
                  <div key={i}>
                    <p className="text-white" key={key}>{`${key}: ${value}`}</p>

                    <div className="flex items-center mb-4">

                      {formEncrypted && formEncrypted[key ] == false ?
                        <>
                          <label className="ms-2 text-sm font-medium text-gray-900 dark:text-gray-300">Change to True</label>
                          <input
                            type="checkbox"
                            // value={key}
                            className="w-10 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                            onClick={() => setFormEncrypted({ ...formEncrypted, [key]: true })}
                          />
                        </>
                        :
                        <>
                          <label className="ms-2 text-sm font-medium text-gray-900 dark:text-gray-300">Change to False</label>
                          <input type="checkbox"
                            value=""
                            className="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                            onClick={() => setFormEncrypted({ ...formEncrypted, [key]: false })}
                          />
                        </>
                      }
                    </div>
                  </div>
                )
              })}
            </div>

          </div>
          <label>Upload your STWO WASM file</label>
          <>
            <button>
              <input type="file" />
            </button></>
          <button
            onClick={uploadWasm}
            disabled={isLoading}
            className={`submit-job-button ${isLoading ? "opacity-50 cursor-not-allowed mb-5" : ""
              }`}
          >
            {isLoading ? "PROCESSING..." : "Upload WASM"}
          </button>
          <button
            onClick={submitJob}
            disabled={isLoading}
            className={`submit-job-button ${isLoading ? "opacity-50 cursor-not-allowed mb-5" : ""
              }`}
          >
            {isLoading ? "PROCESSING..." : "Submit program"}
          </button>
        </div>
        {isLoading && <div className="pixel-spinner mt-4 mx-auto"></div>}
      </div>
      {/* </div> */}
      {/* <HowItWork /> */}
      {/* <button
        className={`block mb-5 font-bold py-2 px-4 rounded bg-blue-500 hover:bg-blue-700 ${isLoading ? "opacity-50 cursor-not-allowed" : ""}`}
        disabled={isLoading}
        onClick={fetchEventsApp}>{events && events?.length ? "Refresh" : "Load"}</button> */}

      <div className="marquee">
      </div>
    </main>
  );
}
