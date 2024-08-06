import { ASKELADD_RELAY } from '@/constants/relay';
import { useNostrContext } from '@/context/NostrContext';
import { IGenerateZKPRequestDVM, KIND_JOB_REQUEST, KIND_JOB_RESULT, ProgramInternalContractName } from '@/types';
import { NDKKind } from '@nostr-dev-kit/ndk';
import { SimplePool, NostrEvent, Relay } from 'nostr-tools';
import { useMemo, useState } from 'react';
import { useFetchEvents } from './useFetchEvents';
import { useDVMState } from './useDVMState';

export const useSubmitJob = () => {
    const { ndk } = useNostrContext();
    const [pool, setPool] = useState(new SimplePool())

    const { fetchEvents, fetchEventsTools, setupSubscriptionNostr } = useFetchEvents()
    const { setIsWaitingJob, setJobId, fetchJobRequest, fetchEventsProof, jobId, eventIdRequest,
        setPublicKey
    } = useDVMState()


    return {
        setupSubscriptionNostr,
        setJobId,
        eventIdRequest,
        // submitJob
    }
};
