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
    console.assert(document.apiBundle);

    let ApiPromise = document.apiBundle.ApiPromise;
    let WsProvider = document.apiBundle.WsProvider;

    console.assert(ApiPromise);
    console.assert(WsProvider);

    console.log(ApiPromise);
    console.log(WsProvider);

    polkadot = {
        ApiPromise,
        WsProvider
    };
}

function initPage() {
    if (document.getElementById("account-page") != null) {
        initAccountPage();
    }
}

function initAccountPage() {
    let nodeStatusSpan = document.getElementById("node-status");
    let nodeConnectButton = document.getElementById("node-connect");
    let walletStatusSpan = document.getElementById("wallet-status");
    let walletConnectButton = document.getElementById("wallet-connect");
    let accountIdSpan = document.getElementById("account-id");
    let accountStatusSpan = document.getElementById("account-status");
    let createAccountButton = document.getElementById("create-account");
    let accountLevelSpan = document.getElementById("account-level");

    console.assert(nodeStatusSpan);
    console.assert(nodeConnectButton);
    console.assert(walletStatusSpan);
    console.assert(walletConnectButton);
    console.assert(accountIdSpan);
    console.assert(accountStatusSpan);
    console.assert(createAccountButton);
    console.assert(accountLevelSpan);

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
