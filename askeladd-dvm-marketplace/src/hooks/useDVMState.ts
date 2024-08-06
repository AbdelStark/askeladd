import { ASKELADD_RELAY } from '@/constants/relay';
import { useNostrContext } from '@/context/NostrContext';
import { IGenerateZKPRequestDVM, KIND_JOB_REQUEST, KIND_JOB_RESULT } from '@/types';
import { NDKEvent, NDKKind } from '@nostr-dev-kit/ndk';
import { SimplePool, NostrEvent, Relay } from 'nostr-tools';
import { useMemo, useState } from 'react';
import { useFetchEvents } from './useFetchEvents';

export const useDVMState = () => {
  const [proofStatus, setProofStatus] = useState<
    "idle" | "pending" | "received" | "verified"
  >("idle");
  const [publicKey, setPublicKey] = useState<string | undefined>();

  const { ndk } = useNostrContext();
  // const pool = new SimplePool()
  const [pool, setPool] = useState(new SimplePool())
  const [jobId, setJobId] = useState<string | undefined>();
  const [jobIdResult, setJobIdResult] = useState<string | undefined>();
  const [isWaitingJob, setIsWaitingJob] = useState(false);
  const [jobEventResult, setJobEventResult] = useState<NostrEvent | undefined | NDKEvent>()
  const [starkProof, setStarkProof] = useState<any | undefined>()
  const [isFetchJob, setIsFetchJob] = useState(false);
  const [isLoadingJobResult, setIsLoadingJobResult] = useState(false);
  const [selectedEvent, setSelectedEvent] = useState<NostrEvent | undefined | NDKEvent>()

  let eventIdRequest = useMemo(() => {
    return jobId
  }, [jobId])
  const { fetchEvents, fetchEventsTools, setupSubscriptionNostr } = useFetchEvents()

  const [proof, setProof] = useState<string | null>(null);

  /** TODO fetch subscribed event
* fix search jobId => check if relayer support NIP-50 
* Fetch Job result from the Prover
* - Tags: By reply of the event_id of the job request?
* - By author
* - Timestamp since/until (doesn't work as expected for me)
*/
  const fetchJobRequest = async (timestampJob?: number, pubkey?: string) => {

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



  /** TODO fetch subscribed event
    * fix search jobId => check if relayer support NIP-50 
    * Fetch Job result from the Prover
     * - Tags: By reply of the event_id of the job request?
     * - By author
     * - Timestamp since/until (doesn't work as expected for me)
     */
  const fetchEventsProof = async () => {
    console.log("fetch events job result proof")
    console.log("last job request id",jobId)
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
  
  const submitJob = async (kind: number, form: any, zkp_request?: IGenerateZKPRequestDVM, tags_parents?:string[][], request?: any) => {
    try {
        // setIsLoading(true);
        // setIsFetchJob(false);
        // setJobId(undefined)
        // setProofStatus("pending");
        // setProof(null);
        // setJobEventResult(undefined);
        // setError(undefined);
        let tags: string[][] = tags_parents ?? []
        console.log("tags parents", tags)
        console.log("zkp_request parent", zkp_request)

        const inputs: Map<string, string> = zkp_request?.program?.inputs ?? new Map<string, string>();
        if (zkp_request?.program?.inputs) {
            Object.entries(zkp_request.program.inputs).map(([key, value]) => {
                inputs.set(key, value as string)
            }
            )
            for (let [key, value] of inputs) {
                tags.push(["param", key, value])
            }
        } else if(form) {
            Object.entries(form).map(([key, value]) => {
                inputs.set(key, value as string)
            }
            )
            for (let [key, value] of inputs) {
                tags.push(["param", key, value])
            }
        }
        tags.push(['output', 'text/json'])
        console.log("tags", tags)

        console.log("inputs", Object.fromEntries(inputs))
        const content = JSON.stringify({
            // request: form,
            request: form ?? inputs,
            // request: Object.fromEntries(inputs),
            program: {
                contract_name: zkp_request?.program?.contract_name,
                internal_contract_name: zkp_request?.program?.internal_contract_name,
                contract_reached: zkp_request?.program?.contract_reached,
                inputs: Object.fromEntries(inputs),
                // inputs:inputs,
                inputs_types: undefined,
                inputs_encrypted: undefined
            }
        })


        // const content = JSON.stringify({
        //     request: request,
        //     program: {
        //         // contract_name: "PoseidonProvingRequest",
        //         // internal_contract_name: "PoseidonProvingRequest",
        //         contract_name: ProgramInternalContractName.WideFibonnaciProvingRequest.toString(),
        //         internal_contract_name: ProgramInternalContractName.WideFibonnaciProvingRequest.toString(),
        //         // internal_contract_name: "PoseidonProvingRequest",

        //         // contract_name:"FibonnacciProvingRequest",
        //         // internal_contract_name:"FibonnacciProvingRequest",
        //         contract_reached: "InternalAskeladd",
        //         // inputs:JSON.stringify(Object.fromEntries(inputs)),
        //         inputs: Object.fromEntries(inputs),
        //         // inputs:tags 
        //     }
        // })
        // Define the timestamp before which you want to fetch events
        // setTimestampJob(new Date().getTime() / 1000)
        //   setTimestampJob(new Date().getTime())
        console.log("inputs", inputs)
        console.log("content", content)
        // return ;
        const timestamp = new Date().getTime()
        /** Use Nostr extension to send event */
        const pool = new SimplePool();
        const poolJob = new SimplePool();
        const relay = await Relay.connect(ASKELADD_RELAY[0])
        if (typeof window !== "undefined" && window.nostr) {

            const pubkey = await window.nostr.getPublicKey();
            console.log("pubkey", pubkey)
            setPublicKey(pubkey)

            let created_at = new Date().getTime();
            // setPublicKey(pubkey)
            const event = await window.nostr.signEvent({
                pubkey: pubkey,
                created_at: created_at,
                kind: kind,
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
            await fetchJobRequest(timestamp, pubkey)
            setIsWaitingJob(true);
            // await timeoutWaitingForJobResult()

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
        // setIsLoading(false);
    }

};

  return {
    starkProof, proof, proofStatus,
    runSubscriptionEvent,
    fetchJobRequest,
    submitJob,
    fetchEventsProof,
    setJobId,
    jobId, eventIdRequest,
    isWaitingJob, setIsWaitingJob,
    publicKey, setPublicKey,
    setIsLoadingJobResult
  }
};
