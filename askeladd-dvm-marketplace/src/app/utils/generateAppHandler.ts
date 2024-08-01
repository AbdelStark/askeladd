import { APPLICATION_PUBKEY_DVM, ASKELADD_RELAY } from "@/constants/relay";
import { ASKELADD_KINDS, ConfigHandle, KIND_JOB_REQUEST, KIND_JOB_RESULT } from "@/types";

const PARAMS_JOB_REQUEST_ZK = {
    request: {

    },
    params: {

    },
    program: {

    }
}

const PARAMS_JOB_RESULT_ZK = {
    job_id: {

    },
    proof: {

    },
    result: {

    }
}

const PARAMS_ALL_KIND = {
    [KIND_JOB_REQUEST]:PARAMS_JOB_REQUEST_ZK,
    [KIND_JOB_RESULT]:PARAMS_JOB_RESULT_ZK,
}
export const generateTagsByAppKind = (tags: string[][], appKind: ASKELADD_KINDS, config:ConfigHandle) => {

    const randomId = Math.random().toString(36).substring(7);
    
    if(config == ConfigHandle.SPECIFIC_KIND) {
        tags = [
            ["d", randomId],
            ["k", appKind?.toString()],
            ["a", `31990:app1-pubkey:${APPLICATION_PUBKEY_DVM}`, ASKELADD_RELAY[0], "ios"],
            ["a", `31990:app2-pubkey:${APPLICATION_PUBKEY_DVM}`, ASKELADD_RELAY[0], "web"],
            ["a", `31990:app3-pubkey:${APPLICATION_PUBKEY_DVM}`, ASKELADD_RELAY[0], "android"],
            ['output', 'text/json']
        ];
    } else {
        tags = [
            ["d", randomId],
            ["k", ASKELADD_KINDS.KIND_JOB_REQUEST.toString()],
            ["k", ASKELADD_KINDS.KIND_JOB_RESULT.toString()],
            ["a", `31990:app1-pubkey:${APPLICATION_PUBKEY_DVM}`, ASKELADD_RELAY[0], "ios"],
            ["a", `31990:app2-pubkey:${APPLICATION_PUBKEY_DVM}`, ASKELADD_RELAY[0], "web"],
            ["a", `31990:app3-pubkey:${APPLICATION_PUBKEY_DVM}`, ASKELADD_RELAY[0], "android"],
            // ["web", "https://..../a/<bech32>", "nevent"],
            // ["web", "https://..../p/<bech32>", "nprofile"],
            // ["web", "https://..../e/<bech32>"],
            ['output', 'text/json']
        ];
    }
    return tags
 
}
export const generateContentAndTags = (configKind: ConfigHandle, appKind?: ASKELADD_KINDS, pubkey?:string): { tags?: string[][], content?: string } => {
    let tags: string[][] = []
    let content = "";
    if (configKind == ConfigHandle?.ALL_KIND) {
        content = JSON.stringify(PARAMS_ALL_KIND);
        tags = [
            ["d",],
            ["k", ASKELADD_KINDS.KIND_JOB_REQUEST.toString()],
            ["k", ASKELADD_KINDS.KIND_JOB_RESULT.toString()],
            ["a", `31990:app1-pubkey:${APPLICATION_PUBKEY_DVM}`, ASKELADD_RELAY[0], "ios"],
            ["a", `31990:app2-pubkey:${APPLICATION_PUBKEY_DVM}`, ASKELADD_RELAY[0], "web"],
            ["a", `31990:app3-pubkey:${APPLICATION_PUBKEY_DVM}`, ASKELADD_RELAY[0], "android"],
            // ["web", "https://..../a/<bech32>", "nevent"],
            // ["web", "https://..../p/<bech32>", "nprofile"],
            // ["web", "https://..../e/<bech32>"],
            ['output', 'text/json']
        ];

    } else if (configKind == ConfigHandle?.SPECIFIC_KIND) {
        if (!appKind) return { tags, content }
        tags = generateTagsByAppKind(tags, appKind, configKind)
        if (appKind && appKind?.toString() == ASKELADD_KINDS.KIND_JOB_REQUEST.toString()) {
            content = JSON.stringify(PARAMS_JOB_REQUEST_ZK)
        }
        else if (appKind && appKind == ASKELADD_KINDS.KIND_JOB_RESULT) {
            content = JSON.stringify(PARAMS_JOB_RESULT_ZK)
        }

    }

    return { tags, content }
}
