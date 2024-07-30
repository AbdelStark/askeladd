import React, { useEffect, useState } from "react";
import browser from "webextension-polyfill";

const Popup: React.FC = () => {
  const [publicKey, setPublicKey] = useState<string | null>(null);

  useEffect(() => {
    const fetchPublicKey = async () => {
      const { publicKey } = await browser.storage.local.get("publicKey");
      setPublicKey(publicKey || null);
    };

    fetchPublicKey();
  }, []);

  return (
    <div>
      <h1>Thorfinn Nostr Signer</h1>
      <p>Your public key:</p>
      {publicKey ? (
        <pre>{publicKey}</pre>
      ) : (
        <p>No public key found. Please generate one.</p>
      )}
    </div>
  );
};

export default Popup;
