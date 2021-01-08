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
    if (document.getElementById("account-page") != null) {
        initAccountPage();
    }
}

function initAccountPage() {
    let nodeStatusSpan = document.getElementById("node-status");
    let nodeConnectButton = document.getElementById("node-connect");

    let keyringStatusSpan = document.getElementById("wallet-status");
    let walletConnectButton = document.getElementById("wallet-connect");
    let accountIdSpan = document.getElementById("account-id");
    let accountStatusSpan = document.getElementById("account-status");

    let createGameAccountButton = document.getElementById("create-game-account");
    let gameAccountLevelSpan = document.getElementById("account-level");

    nodeConnectButton.disabled = false;

    nodeConnectButton.addEventListener("click", (event) => {
        nodeConnect()
            .then((msg) => {
                nodeStatusSpan.innerText = msg;
                nodeConnectButton.disabled = true;
            })
            .catch((error) => {
                nodeStatusSpan.innerText = error;
                nodeConnectButton.disabled = true;
            });
    });
}

async function nodeConnect() {
    let addr = "ws://127.0.0.1:9944"
    console.log(`Trying to connect to ${addr}`);
    
    const provider = new polkadot.WsProvider(addr);
    const api = await polkadot.ApiPromise.create({ provider });

    const [chain, nodeName, nodeVersion] = await Promise.all([
        api.rpc.system.chain(),
        api.rpc.system.name(),
        api.rpc.system.version()
    ]);

    let msg = `Connected to ${chain} using ${nodeName} v${nodeVersion}`;
    console.log(msg);
    return msg;
}



maybeLoad();
