"use strict";

import { ApiPromise, WsProvider } from "@polkadot/api";
import { Keyring } from "@polkadot/keyring";

document.apiBundle = {
    ApiPromise,
    WsProvider,
    Keyring
};
