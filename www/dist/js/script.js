"use strict";

let polkadot = null;

function maybeLoad() {
    window.addEventListener("load", (event) => {
        onLoad();
    });
}

function onLoad() {
    loadApis();

    initPage();
}

function loadApis() {
    console.assert(document.polkadotApiBundle);
    polkadot = document.polkadotApiBundle;
}

function initPage() {
    let nodeStatusSpan = document.getElementById("node-status");
    let nodeEndpointInput = document.getElementById("node-endpoint");
    let nodeConnectButton = document.getElementById("node-connect");

    let keyringStatusSpan = document.getElementById("keyring-status");
    let accountKeyInput = document.getElementById("account-key");
    let accountIdSpan = document.getElementById("account-id");
    let keyringConnectButton = document.getElementById("keyring-connect");

    let playerAccountStatusSpan = document.getElementById("player-account-status");
    let playerAccountLevelSpan = document.getElementById("player-account-level");
    let createPlayerAccountButton = document.getElementById("create-player-account");

    nodeEndpointInput.disabled = false;
    nodeConnectButton.disabled = false;

    let api = null;
    let keyring = null;
    let keypair = null;

    nodeConnectButton.addEventListener("click", async (event) => {
        let nodeEndpoint = nodeEndpointInput.value;

        setInnerMessageNeutral(nodeStatusSpan, "waiting");

        nodeEndpointInput.disabled = true;
        nodeConnectButton.disabled = true;

        try {
            api = await nodeConnect(nodeEndpoint);

            console.log("api:");
            console.log(api);

            let { chain, nodeName, nodeVersion }
                = await getChainMetadata(api);

            let msg = `Connected to ${chain} using ${nodeName} v${nodeVersion}`;
            console.log(msg);

            setInnerMessageSuccess(nodeStatusSpan, msg);

            accountKeyInput.disabled = false;
            keyringConnectButton.disabled = false;

        } catch (error) {
            setInnerMessageFail(nodeStatusSpan, error);
            nodeEndpointInput.disabled = false;
            nodeConnectButton.disabled = false;
            return;
        }
    });

    keyringConnectButton.addEventListener("click", async (event) => {
        console.assert(api);

        const accountKey = accountKeyInput.value;

        keyring = new polkadot.Keyring();
        keypair = keyring.addFromUri(accountKey);

        console.log(`Key ${keypair.meta.name}: has address ${keypair.address} with publicKey [${keypair.publicKey}]`);

        let msg = `Connected as ${keypair.address}`;
        setInnerMessageSuccess(keyringStatusSpan, msg);

        accountKeyInput.disabled = true;
        keyringConnectButton.disabled = true;

        try {
            let accountInfo = await loadPlayerAccountInfo(api, keypair);
        } catch (error) {
        }
    });
}

async function nodeConnect(addr) {
    console.log(`Trying to connect to ${addr}`);
    
    const provider = new polkadot.WsProvider(addr);
    const api = await polkadot.ApiPromise.create({ provider });

    return api;
}

async function getChainMetadata(api) {

    const [chain, nodeName, nodeVersion] = await Promise.all([
        api.rpc.system.chain(),
        api.rpc.system.name(),
        api.rpc.system.version()
    ]);

    return {
        chain,
        nodeName,
        nodeVersion
    };
}

async function loadPlayerAccountInfo(api, keypair) {
    
}

function setInnerMessageSuccess(elt, msg) {
    elt.innerText = msg;
    elt.classList.remove("msg-fail");
    elt.classList.add("msg-success");
}

function setInnerMessageFail(elt, msg) {
    elt.innerText = msg;
    elt.classList.remove("msg-success");
    elt.classList.add("msg-fail");
}

function setInnerMessageNeutral(elt, msg) {
    elt.innerText = msg;
    elt.classList.remove("msg-success");
    elt.classList.remove("msg-fail");
}







maybeLoad();
