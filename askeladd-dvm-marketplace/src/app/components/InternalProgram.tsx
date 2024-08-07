import { NDKEvent, NDKKind } from '@nostr-dev-kit/ndk';
import { Event as NostrEvent, Relay, SimplePool } from 'nostr-tools';
import React, { useEffect, useMemo, useState } from 'react';
import { ContractUploadType, IGenerateZKPRequestDVM, JobResultProver, KIND_JOB_REQUEST, KIND_JOB_RESULT, ProgramInternalContractName } from '@/types';
import { useFetchEvents } from '@/hooks/useFetchEvents';
import { ASKELADD_RELAY } from '@/constants/relay';
import init, { verify_stark_proof, verify_stark_proof_wide_fibo, prove_and_verify, stark_proof_wide_fibo, prove_stark_proof_poseidon, verify_stark_proof_poseidon, prove_and_verify_fib, verify_stark_proof_fib } from "../../pkg"
import { useNostrContext } from '@/context/NostrContext';
import { useDVMState } from '@/hooks/useDVMState';
// Define the props for the component
interface TagsCardProps {
    event?: NDKEvent | NostrEvent;  // Array of array of strings
    zkp_request?: IGenerateZKPRequestDVM
}
const InternalProgram: React.FC<TagsCardProps> = ({ event, zkp_request }) => {
    const { ndk, pool } = useNostrContext()
    const [form, setForm] = useState({})
    const program = zkp_request?.program;
    const [isOpenForm, setIsOpenForm] = useState(false)
    const [logSize, setLogSize] = useState<number>(5);
    const [claim, setClaim] = useState<number>(443693538);
    const [publicKey, setPublicKey] = useState<string | undefined>();
    const [error, setError] = useState<string | undefined>()
    const [jobEventResult, setJobEventResult] = useState<NostrEvent | undefined | NDKEvent>()
    const [isLoading, setIsLoading] = useState<boolean>(false);
    const [isInitialized, setIsInitialized] = useState(false);
    const [isFetchJob, setIsFetchJob] = useState(false);
    const [isWaitingJob, setIsWaitingJob] = useState(false);
    const {jobId,  fetchJobRequest, fetchEventsProof, starkProof, submitJob: submitJobModular, proof, proofStatus, setProof, setProofStatus } = useDVMState()
    // Init wasm module to run_fibonacci_verify
    useEffect(() => {
        init()
            .then(() => setIsInitialized(true))
            .catch((error) => {
                console.error("Failed to initialize WASM module:", error);

            });
    }, []);

    useEffect(() => {
      
        if (!jobId && !jobEventResult) {
            timeoutWaitingForJobResult()
        }
    }, [jobId, jobEventResult, pool])


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
            setProofStatus("pending");
            setProof(null);
            setJobEventResult(undefined);
            setError(undefined);

            let tags: string[][] = [
            ];

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
            console.log("inputs", Object.fromEntries(inputs))
            console.log("inputs", inputs)
            let job_request: IGenerateZKPRequestDVM = {
                // request: form,
                request: Object.fromEntries(inputs),
                program: {
                    contract_name: zkp_request?.program?.contract_name,
                    internal_contract_name: zkp_request?.program?.internal_contract_name,
                    contract_reached: zkp_request?.program?.contract_reached,
                    inputs: inputs,
                    // inputs_types: undefined,
                    // inputs_encrypted: undefined
                }
            }

            const content = JSON.stringify(job_request)
            console.log("content", content)

            let res = await submitJobModular(5600,
                Object.fromEntries(inputs),
                job_request,
                tags

            )
            if(res && res?.success) {
                fetchJobRequest()
                fetchEventsProof()
            }
            return res;
        } catch (e) {
        } finally {
            setIsLoading(false);
        }

    };

    const verifyProofHandler = async () => {
        try {
            if (proof && starkProof) {
                setIsLoading(true);
                const inputs: Map<string, string> = new Map<string, string>();
                {
                    Object.entries(form).map(([key, value]) => {
                        inputs.set(key, value as string)
                    }
                    )
                }
                console.log("wide fibo prove_result", starkProof);

                if (zkp_request?.program?.internal_contract_name == ProgramInternalContractName.WideFibonacciProvingRequest) {
                    let log_n_instances = inputs.get("log_n_instances");
                    let log_fibonacci_size = inputs.get("log_fibonacci_size");
                    const serialised_proof_from_nostr_event = JSON.stringify(starkProof);
                    if (!log_n_instances && !log_fibonacci_size && !serialised_proof_from_nostr_event) return;
                    const prove_result = stark_proof_wide_fibo(Number(log_fibonacci_size), Number(log_n_instances));
                    console.log("wide fibo prove_result", prove_result);
                    console.log("serialised_proof_from_nostr_event", serialised_proof_from_nostr_event);
                    const verify_result = verify_stark_proof_wide_fibo(Number(log_fibonacci_size), Number(log_n_instances), serialised_proof_from_nostr_event);
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
                else if (zkp_request?.program?.internal_contract_name == ProgramInternalContractName?.PoseidonProvingRequest) {

                    let log_n_instances = inputs.get("log_n_instances");
                    if (!log_n_instances) return;
                    const prove_result = prove_stark_proof_poseidon(Number(log_n_instances));
                    console.log("poseidon prove_result", prove_result);
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
                else if (zkp_request?.program?.internal_contract_name == ProgramInternalContractName.FibonacciProvingRequest) {
                    const prove_result = prove_and_verify_fib(logSize, claim);
                    console.log("prove_result", prove_result);
                    const serialised_proof_from_nostr_event = JSON.stringify(starkProof);
                    console.log("serialised_proof_from_nostr_event", serialised_proof_from_nostr_event);
                    const verify_result = verify_stark_proof_fib(logSize, claim, serialised_proof_from_nostr_event);
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

    const params = Object.fromEntries(zkp_request?.program?.inputs?.entries() ?? [])
    // Handle changes in form inputs
    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value } = e.target;
        setForm(prev => ({
            ...prev,
            [name]: value
        }));
        console.log("form", form)
    };

    return (
        <div className="max-w-sm cursor-pointer my-5 p-1  m-1 break-normal p-5 m-5 text-white mx-auto max-w-lg p-6 border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700">
            {program?.event_id &&
                <p>Event id: {zkp_request?.program?.event_id}</p>
            }
            <p className='break-words whitespace-pre-line'>{zkp_request?.program?.contract_name?.toString()}</p>
            <p className='break-words whitespace-pre-line'>Deployed: {zkp_request?.program?.contract_reached == ContractUploadType.InternalAskeladd && "Internal Program"}</p>
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

export default InternalProgram;
