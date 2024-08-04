import { NDKEvent, NDKKind } from '@nostr-dev-kit/ndk';
import { Event as NostrEvent, Relay, SimplePool } from 'nostr-tools';
import React, { useEffect, useMemo, useState } from 'react';
import { ContractUploadType, IGenerateZKPRequestDVM, JobResultProver, KIND_JOB_REQUEST, KIND_JOB_RESULT, ProgramInternalContractName } from '@/types';
import { useFetchEvents } from '@/hooks/useFetchEvents';
import { ASKELADD_RELAY } from '@/constants/relay';
import init, { verify_stark_proof, verify_stark_proof_wide_fibo, prove_and_verify, stark_proof_wide_fibo, prove_stark_proof_poseidon, verify_stark_proof_poseidon } from "../../pkg"
import { useNostrContext } from '@/context/NostrContext';
// Define the props for the component
interface TagsCardProps {
    event?: NDKEvent | NostrEvent;  // Array of array of strings
    program?: IGenerateZKPRequestDVM
}
const ProgramCard: React.FC<TagsCardProps> = ({ event, program }) => {
    const { fetchEvents, fetchEventsTools, setupSubscriptionNostr } = useFetchEvents()
    const { ndk, pool } = useNostrContext()

    const [isOpenForm, setIsOpenForm] = useState(false)
    const [logSize, setLogSize] = useState<number>(5);
    const [claim, setClaim] = useState<number>(443693538);
    const [publicKey, setPublicKey] = useState<string | undefined>();
    const [jobId, setJobId] = useState<string | undefined>();
    const [error, setError] = useState<string | undefined>()
    const [starkProof, setStarkProof] = useState<any | undefined>()
    const [jobEventResult, setJobEventResult] = useState<NostrEvent | undefined | NDKEvent>()
    const [seeTag, setSeeTag] = useState<boolean>(false)
    const [proof, setProof] = useState<string | null>(null);
    const [isLoading, setIsLoading] = useState<boolean>(false);
    const [isInitialized, setIsInitialized] = useState(false);
    const [isFetchJob, setIsFetchJob] = useState(false);
    const [isLoadingJobResult, setIsLoadingJobResult] = useState(false);
    const [isWaitingJob, setIsWaitingJob] = useState(false);
    const [timestampJob, setTimestampJob] = useState<number | undefined>();
    const [proofStatus, setProofStatus] = useState<
        "idle" | "pending" | "received" | "verified"
    >("idle");
    const [selectedEvent, setSelectedEvent] = useState<NostrEvent | undefined | NDKEvent>()

    let eventIdRequest = useMemo(() => {
        return jobId
    }, [jobId])

    // Init wasm module to run_fibonacci_verify
    useEffect(() => {
        init()
            .then(() => setIsInitialized(true))
            .catch((error) => {
                console.error("Failed to initialize WASM module:", error);

            });
    }, []);

    useEffect(() => {
        // const pool = new SimplePool();
        if (pool) {
            runSubscriptionEvent(pool)
        }
        if (!jobId && !jobEventResult) {
            timeoutWaitingForJobResult()
        }
    }, [jobId, jobEventResult, pool])


    const runSubscriptionEvent = (pool: SimplePool, pubkey?: string) => {

        // WebSocket connection setup
        // const ws = new WebSocket([ASKELADD_RELAY[0]]);  // Replace with your Nostr relay URL

        // ws.onopen = () => {
        //     // Subscribe to specific events, adjust filters as needed
        //     ws.send(JSON.stringify({
        //         "req": "EVENTS",
        //         // "filter": {
        //         //     "#e": ["3a5f5b4..."]  // Your event criteria here
        //         // }
        //     }));
        // };

        // ws.onmessage = (event) => {
        //     const data = JSON.parse(event.data);
        //     if (data) {
        //         if (!jobId) return;
        //         if (pubkey && data?.pubkey == pubkey) {
        //             setJobId(data?.id)
        //         }
        //         // setEvents(currentEvents => [...currentEvents, data]);
        //     }
        // };

        // ws.onerror = (error) => {
        //     console.error("WebSocket error:", error);
        // };

        let poolSubscription = pool.subscribeMany(
            ASKELADD_RELAY,
            [
                // {
                //   kinds: [KIND_JOB_REQUEST as NDKKind],
                //   // since:timestampJob
                //   // authors: pubkey ? [pubkey] : []
                // },
                {
                    kinds: [KIND_JOB_RESULT as NDKKind],
                    // since:timestampJob
                },
            ],
            {
                onevent(event) {
                    //   if (event?.kind == KIND_JOB_REQUEST) {
                    //     if (!jobId) return;
                    //     if (pubkey && event?.pubkey == pubkey) {
                    //       setJobId(event?.id)
                    //     }
                    //     poolSubscription.close();
                    //   }
                    if (event?.kind == KIND_JOB_RESULT) {
                        if (!jobId) return;
                        let id = jobId ?? eventIdRequest;
                        if (id && !jobEventResult) {
                            console.log("Event job result received: ", event?.id);
                            console.log("event job content result include job: ", id);
                            let isIncludedJobId = event?.content?.includes(jobId)
                            let jobEventResultFind = event?.content?.includes(jobId)
                            console.log("isIncludedJobId", isIncludedJobId);
                            if (isIncludedJobId) {
                                console.log("Event JOB_RESULT find", jobEventResultFind);
                                getDataOfEvent(event);
                                setJobEventResult(event)
                            }
                        }
                        poolSubscription.close();
                    }
                },
                onclose: () => {
                    poolSubscription.close()
                },
                oneose() {
                    poolSubscription.close()
                }
            }
        )
    }


    const timeoutWaitingForJobResult = async () => {
        console.log("waiting timeout job result")
        setTimeout(() => {
            waitingForJobResult()
        }, 5000);
    }

    /** Effect to fetch the job result when a job request is sent */
    const waitingForJobResult = async () => {
        if (jobEventResult && jobId) return;
        fetchEventsProof()
        setIsLoading(false);
        setIsWaitingJob(false)
    }

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
            // console.log("filterJob", filterJob);
            if (jobEventResultFind?.id) {
                console.log("Event JOB_RESULT find", jobEventResultFind);
                getDataOfEvent(jobEventResultFind);
                setJobEventResult(jobEventResultFind)
            }
        }
    }

    const getDataOfEvent = (lastEvent?: NDKEvent | NostrEvent) => {
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

    const fetchJobRequest = async (pubkey?: string) => {
        const { events } = await fetchEventsTools({
            kind: KIND_JOB_REQUEST,
            since: timestampJob,
            // authors: pubkey ? [pubkey] : []
        });
        console.log("events job request", events);
        if (!events) return;
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


    /** Submit job with JOB_REQUEST 5600
 * - Use extension NIP-7
 * - Default public key demo
 * - NDK generate key or import later
*/
    const submitJob = async () => {
        try {

            /** Todo better check */
            if (!isLoading && !isOpenForm && Object.entries(form).length == 0) return;
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

            const inputs: Map<string, string> = new Map<string, string>();
            {
                Object.entries(form).map(([key, value]) => {
                    inputs.set(key, value as string)
                }
                )
            }
            console.log("inputs", Object.fromEntries(inputs))
            const content = JSON.stringify({
                request: form,
                program: {
                    contract_name: program?.program_params?.contract_name,
                    internal_contract_name: program?.program_params?.internal_contract_name,
                    contract_reached: program?.program_params?.contract_reached,
                    inputs: Object.fromEntries(inputs),
                    inputs_types: undefined,
                    inputs_encrypted: undefined
                }
            })
            // Define the timestamp before which you want to fetch events
            setTimestampJob(new Date().getTime())
            console.log("inputs", inputs)
            console.log("content", content)
            /** Use Nostr extension to send event */
            const pool = new SimplePool();
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
                const eventID = await Promise.any(pool.publish(ASKELADD_RELAY, event as NostrEvent));
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

    const verifyProofHandler = async () => {
        try {
            if (proof) {
                setIsLoading(true);
                const inputs: Map<string, string> = new Map<string, string>();
                {
                    Object.entries(form).map(([key, value]) => {
                        inputs.set(key, value as string)
                    }
                    )
                }

                if (program?.program_params?.internal_contract_name == ProgramInternalContractName.FibonnacciProvingRequest) {
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
                } else if (program?.program_params?.internal_contract_name == ProgramInternalContractName?.PoseidonProvingRequest) {

                    let log_n_instances = inputs.get("log_n_instances");
                    if (!log_n_instances) return;
                    const prove_result = prove_stark_proof_poseidon(Number(log_n_instances));
                    console.log("prove_result", prove_result);
                    const serialised_proof_from_nostr_event = JSON.stringify(starkProof);
                    console.log("serialised_proof_from_nostr_event", serialised_proof_from_nostr_event);
                    const verify_result = verify_stark_proof_poseidon(Number(log_n_instances), serialised_proof_from_nostr_event);
                    console.log("verify result", verify_result);
                    console.log("verify message", verify_result.message);
                    console.log("verify success", verify_result.success);
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

    const date: string | undefined = event?.created_at ? new Date(event?.created_at).toDateString() : undefined

    const params = Object.fromEntries(program?.program_params?.inputs?.entries() ?? [])

    const [form, setForm] = useState({})
    // Handle changes in form inputs
    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value } = e.target;
        setForm(prev => ({
            ...prev,
            [name]: value
        }));
        console.log("form", form)
    };

    const program_params = program?.program_params;
    return (
        <div className="max-w-sm cursor-pointer my-5 p-1  m-1 break-normal p-5 m-5 text-white mx-auto max-w-lg p-6 border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700">
            {program_params?.event_id &&
                <p>Event id: {program?.program_params?.event_id}</p>
            }
            <p className='break-words whitespace-pre-line'>{program?.program_params?.contract_name?.toString()}</p>
            {/* <p>{p.program_params?.internal_contract_name}</p> */}
            <p className='break-words whitespace-pre-line'>Deployed: {program?.program_params?.contract_reached == ContractUploadType.InternalAskeladd && "Internal Program"}</p>
            {isLoading && <div className="pixel-spinner mt-4 mx-auto"></div>}
            <button
                className={`mt-4 opacity-50 bg-blue-500 my-5 p-1`}
                onClick={() => setIsOpenForm(!isOpenForm)}>Open</button>
            {isOpenForm &&

                <div className='my-5'>
                    {Object.entries(form).map(([key, value]) => (
                        <p key={key}>{`${key}: ${value}`}</p>
                    ))}

                    {Object.entries(params).map((e, i) => {
                        return (
                            <div
                                key={i}
                            >
                                <p> {e?.[1]}</p>
                                <input

                                    className='text-black'
                                    name={e?.[1]}
                                    onChange={handleChange}
                                ></input>
                            </div>
                        )
                    })}
                </div>
            }

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

            <button
                onClick={submitJob}
                disabled={isLoading || !isOpenForm && !form}
                className={`submit-job-button ${isLoading ? "opacity-50 cursor-not-allowed" : ""
                    }`}
            >
                {isLoading ? "PROCESSING..." : "SUBMIT JOB"}
            </button>

        </div>
    )
};

export default ProgramCard;
