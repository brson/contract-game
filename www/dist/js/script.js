"use strict";

let polkadot = null;
let gameController = null;

function maybeLoad() {
    window.addEventListener("load", (event) => {
        onLoad();
    });
}

function onLoad() {
    loadApis();

    gameController = {
    };

    initPage();
}

function loadApis() {
    console.assert(document.polkadotApiBundle);
    polkadot = document.polkadotApiBundle;
}

function initPage() {
    initAccountPage();
}

function initAccountPage() {
    let nodeStatusSpan = document.getElementById("node-status");
    let nodeEndpointInput = document.getElementById("node-endpoint");
    let nodeConnectButton = document.getElementById("node-connect");

    let keyringStatusSpan = document.getElementById("wallet-status");
    let walletConnectButton = document.getElementById("wallet-connect");
    let accountIdInput = document.getElementById("account-id");
    let accountStatusSpan = document.getElementById("account-status");

    let createGameAccountButton = document.getElementById("create-game-account");
    let gameAccountLevelSpan = document.getElementById("account-level");

    nodeConnectButton.disabled = false;
    nodeEndpointInput.disabled = false;

    nodeConnectButton.addEventListener("click", async (event) => {
        let nodeEndpoint = nodeEndpointInput.value;

        setInnerMessageNeutral(nodeStatusSpan, "waiting");
        nodeConnectButton.disabled = true;
        nodeEndpointInput.disabled = true;
        try {
            let api = await nodeConnect(nodeEndpoint);

            console.log("api:");
            console.log(api);

            let { chain, nodeName, nodeVersion }
                = await getChainMetadata(api);

            let msg = `Connected to ${chain} using ${nodeName} v${nodeVersion}`;
            console.log(msg);

            setInnerMessageSuccess(nodeStatusSpan, msg);

        } catch (error) {
            setInnerMessageFail(nodeStatusSpan, error);
            nodeConnectButton.disabled = false;
            nodeEndpointInput.disabled = false;
            return;
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
    }
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
