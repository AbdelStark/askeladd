import React, { useState, useEffect } from "react";
import styled from "@emotion/styled";
import { getOrCreateKeyPair } from "../background/index";

const Container = styled.div`
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
  background-color: #2c2c2c;
  color: #00ff00;
  font-family: "Press Start 2P", cursive;
`;

const Title = styled.h1`
  font-size: 24px;
  text-align: center;
  margin-bottom: 20px;
`;

const KeyContainer = styled.div`
  background-color: #1a1a1a;
  padding: 10px;
  border-radius: 5px;
  word-break: break-all;
  font-size: 12px;
  margin-bottom: 20px;
`;

const Button = styled.button`
  background-color: #00ff00;
  color: #000;
  border: none;
  padding: 10px 20px;
  font-family: "Press Start 2P", cursive;
  cursor: pointer;
  width: 100%;
  margin-bottom: 10px;

  &:hover {
    background-color: #00cc00;
  }
`;

export const Options: React.FC = () => {
  const [publicKey, setPublicKey] = useState<string | null>(null);
  const [privateKey, setPrivateKey] = useState<string | null>(null);

  useEffect(() => {
    loadKeys();
  }, []);

  const loadKeys = async () => {
    console.log("Loading keys if they exist, otherwise creating new ones");
    const keys = await getOrCreateKeyPair();
    console.log("Keys loaded, public key: ", keys.publicKey);
    setPublicKey(keys.publicKey || null);
    setPrivateKey(keys.privateKey || null);
  };

  const handleGenerateKeys = async () => {
    console.log("Generating keys");
    await loadKeys();
  };

  return (
    <Container>
      <Title>Thorfinn Options</Title>
      {publicKey && privateKey ? (
        <>
          <h2>Public Key:</h2>
          <KeyContainer>{publicKey}</KeyContainer>
          <h2>Private Key:</h2>
          <KeyContainer>{privateKey}</KeyContainer>
          <Button onClick={handleGenerateKeys}>Generate New Keys</Button>
        </>
      ) : (
        <Button onClick={handleGenerateKeys}>Generate Keys</Button>
      )}
    </Container>
  );
};
