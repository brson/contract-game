"use strict";

import { ApiPromise, WsProvider } from "@polkadot/api";
import { Keyring } from "@polkadot/keyring";
import { CodePromise, BlueprintPromise, ContractPromise, Abi } from "@polkadot/api-contract";

document.polkadotApiBundle = {
    ApiPromise,
    WsProvider,
    Keyring,
    CodePromise,
    BlueprintPromise,
    ContractPromise,
    Abi
};
