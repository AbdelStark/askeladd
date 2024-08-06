import { ContractUploadType, IGenerateZKPRequestDVM, ProgramInternalContractName } from "@/types";

/** TODO can be used as an event after a JOB REQUEST LAUNCH PROGRAM */

const program_map_fibo= new Map<string,string>()

program_map_fibo.set("0","log_size")
program_map_fibo.set("1","claim")


const program_map_wide_fibo= new Map<string,string>()

program_map_wide_fibo.set("0","log_fibonnacci_size")
program_map_wide_fibo.set("1","log_n_instances")



const program_map_poseidon= new Map<string,string>()
program_map_poseidon.set("0","log_n_instances")




const program_map_multi_fibo= new Map<string,string>()
program_map_multi_fibo.set("0","log_sizes");
program_map_multi_fibo.set("1","claims");

export const PROGRAM_INTERAL_REQUEST:IGenerateZKPRequestDVM[] = [

 
    {
        // Wide Fibonnaci
        request: {
            log_fibonacci_size:0,
            log_n_instances:0
        },
        program: {
            contract_name:ProgramInternalContractName.WideFibonnaciProvingRequest.toString(),
            internal_contract_name:ProgramInternalContractName.WideFibonnaciProvingRequest,
            contract_reached:ContractUploadType.InternalAskeladd,
            inputs:program_map_wide_fibo
        }

    },


    {
        // Poseidon
        request: {
            log_n_instances:0
        },
        program: {
            contract_name:ProgramInternalContractName.PoseidonProvingRequest.toString(),
            internal_contract_name:ProgramInternalContractName.PoseidonProvingRequest,
            contract_reached:ContractUploadType.InternalAskeladd,
            inputs:program_map_poseidon
        }

    },

    // {
    //     // Fibonnaci
    //     request: {
    //         log_size:0,
    //         claim:0
    //     },
    //     program: {
    //         contract_name:ProgramInternalContractName.FibonnacciProvingRequest.toString(),
    //         internal_contract_name:ProgramInternalContractName.FibonnacciProvingRequest,
    //         contract_reached:ContractUploadType.InternalAskeladd,
    //         inputs:program_map_fibo
    //     }

    // },


    // {
    //     // Multi Fibonnaci
    //     request: {
    //         log_sizes:0,
    //         claims:0
    //     },
    //     program: {
    //         contract_name:ProgramInternalContractName.MultiFibonnacciProvingRequest.toString(),
    //         internal_contract_name:ProgramInternalContractName.MultiFibonnacciProvingRequest,
    //         contract_reached:ContractUploadType.InternalAskeladd,
    //         inputs:program_map_multi_fibo
    //     }

    // },

]