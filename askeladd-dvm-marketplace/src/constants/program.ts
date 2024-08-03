import { ContractUploadType, IGenerateZKPRequestDVM, ProgramInternalContractName } from "@/types";


const program_map_fibo= new Map<string,string>()

program_map_fibo.set("0","log_size")
program_map_fibo.set("1","claim")


const program_map_wide_fibo= new Map<string,string>()

program_map_wide_fibo.set("0","log_fibonacci_size")
program_map_wide_fibo.set("1","log_n_instances")
export const PROGRAM_INTERAL_REQUEST:IGenerateZKPRequestDVM[] = [

    {
        // Fibonnaci
        request: {
            log_size:0,
            claim:0
        },
        program_params: {
            contract_name:ProgramInternalContractName.FibonnacciProvingRequest.toString(),
            internal_contract_name:ProgramInternalContractName.FibonnacciProvingRequest,
            contract_reached:ContractUploadType.InternalAskeladd,
            params_map:program_map_fibo
        }

    },

    {
        // Wide Fibonnaci
        request: {
            log_fibonacci_size:0,
            log_n_instances:0
        },
        program_params: {
            contract_name:ProgramInternalContractName.WideFibonnaciProvingRequest.toString(),
            internal_contract_name:ProgramInternalContractName.WideFibonnaciProvingRequest,
            contract_reached:ContractUploadType.InternalAskeladd,
            params_map:program_map_wide_fibo
        }

    },

]