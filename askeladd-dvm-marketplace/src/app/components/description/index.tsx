import { useState } from "react";

export const HowItWork = () => {

    const [openHowItWork, setOpenHowItWork] = useState(false);

    return(
        <div onClick={() => setOpenHowItWork(!openHowItWork)}
        className="max-w-sm cursor-pointer my-5 p-1  m-1 whitespace-pre-line break-words"
      >
        <p className="text-white">How the ASKELADD DVM ZK works?</p>
        {!openHowItWork &&
          <button> Open </button>
        }
        {openHowItWork &&
          <>
            <div>
              <p>As an User  </p>
              <p className="text-white">User send a JOB_REQUEST with different params on the Nostr event.</p>
              <p className="text-white">It can change with all STWO Prover enabled on the Marketplace</p>
              <p className="text-white mb-5">You need theses params on the Nostr event:</p>
              <p>Inputs  </p>

              <p className="text-white">Request: {JSON.stringify({
                "claim": "413300",
                "log_size": "5"
              })} &quot; The input of the Program</p>
              <p className="text-white ">Tags: {`[
                ["param", "input_name", "value"] // The input of the Program
              ]`} </p>
            </div>
            <button> Close </button>
          </>
        }
      </div>
    )
}