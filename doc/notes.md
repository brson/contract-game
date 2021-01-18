# 2021/01/01

# Further adventures with parity's `ink!`

During my [last series of bloggings][subblog] about [Substrate],
I intended to explore [`ink!`],
Parity's DSL library for writing smart contracts in Rust.
Those posts though mostly ended up being about setting up
the appropriate development environment,
and my failures along the way.

I remain though interested in learning Substrate,
and in particular smart contract programming in Rust on Substrate.
So in the time since that last exploration,
[Aimee] and I have envisioned a small project
to hack on that will guide us to learn more about
Substrate/Ink development.

This report will be about our experiences in the
first few weeks of implementing that project.
It is broadly divided into three sections:
contract implementation with Ink,
client implementation with [polkadot.js],
and some concluding thoughts.


[Substrate]: https://substrate.dev
[subblog]: https://brson.github.io/2020/12/03/substrate-and-ink-part-1
[`ink!`]: https://github.com/paritytech/ink
[plokadot.js]: https://github.com/polkadot-js

- [Our project][#our-project]
- [Terminology][#terminology]
- [Implementing the contract][#implementing-the-contract]
  - [Debugging cross-contract calls][#debugging-cross-contract-calls]
  - [But first, updating our tools][#but-first-updating-our-tools]
  - [And then debugging cross-contract calls][#and-then-debugging-cross-contract-calls]
  - [Another try at cross-contract lass with `CallBuilder`][#another-try-at-cross-contract-calls-with-callbuilder]
- [Connecting to our contract with polkadot-js][#connecting-to-our-contract-with-polkadot-js]
- [Some thoughts][#some-thoughts]
  - [First, some hopefulness][#first-some-hopefulness]
  - [Next, some venting][#next-some-venting]
  - [It's ok, I'm learning][#its-ok-im-learning]



## Our project


## Implementing the contract

ink_storage::HashMap doesn't implement Clone.

We try to put an `ink_storage::HashMap` into a custom `GameAccount` struct
inside another `ink_storage::HashMap`, inside our `Game` storage type.

Something like

```rust
    #[ink(storage)]
    pub struct Game {
        game_accounts: ink_storage::HashMap<AccountId, GameAccount>,
    }

    #[derive(Debug, Clone, scale::Encode, scale::Decode, ink_storage_derive::PackedLayout, ink_storage_derive::SpreadLayout)]
    pub struct GameAccount {
        level: u32,
        level_programs: ink_storage::HashMap<u32, AccountId>,
    }
```

It doesn't work, with a bunch of errors about traits like `WrapperTypeEncode`
not being implemented for types like `ink_storage::HashMap`.

We look at the ink examples and don't see any using nested collection
types in their storage type.
Instead they all use a "flat" data model.
I don't really want to do that because it will be harder to maintain
the invariants I want.
Reading the API docs for `scale::Encode` we see that the standard
`BTreeMap` type implements it,
so instead of nesting `ink_storage::HashMap`s inside each other,
we use a `BTreeMap` instead, like

```rust
    #[ink(storage)]
    pub struct Game {
        game_accounts: ink_storage::HashMap<AccountId, GameAccount>,
    }

    #[derive(Debug, Clone, scale::Encode, scale::Decode, ink_storage_derive::PackedLayout, ink_storage_derive::SpreadLayout)]
    pub struct GameAccount {
        level: u32,
        level_programs: BTreeMap<u32, AccountId>,
    }
```

I imagine this will be inefficient,
but for now we want our code to be readable,
not efficient.


For each level of our game,
our game contract needs to call a player's "level contract".
So each of our levels defines a contract interface,
and each player implements that interface in their own
contract for the level.

When we start trying to figure out how to call another
arbitrary contract,
using only some interface,
we run into a lack of examples and documentation.




While incrementally adding features,
experimenting with ink APIs,
and attempting to debug,
we find that we don't know how to do "println debugging":
ink defines [`ink_env::debug_println`],
but when we use it we don't see any output anywhere.

[`ink_env::debug_println`]: https://paritytech.github.io/ink/ink_env/fn.debug_println.html

I aske in the `#ink:matrix.parity.io` channel where to see the output,
and Alexander Theißen replies:

> "They are printed to the console. You need to raise the log level for `runtime`
  module in order to see them. I also recommend to silence all the block
  production noise: `-lerror,runtime=debug`"

So those are presumably flags to `canvas-node`.

Now I am running my canvas node, from source, with the command

```
cargo run -- --dev --tmp -lerror,runtime=debug
```





We have Alice construct our game contract,
and want to test having that contract
call another contract (a "level contract").
For testing purposes that level contract is the
"flipper" example contract.
We upload that contract and have Bob construct it.

We are confused about:

- Are contracts identified by users' account IDs,
  or do they have their own account IDs?
- How can we find the account ID of a contract we've constructed?

We're confused every step of the way.

While trying to figure out Bob's account ID we make two discoveries:

1) The `subkey inspect` command:

    ```
    subkey inspect //Bob
    Secret Key URI `//Bob` is account:
      Secret seed:      0x398f0c28f98885e046333d4a41c19cee4c37368a9832c6502f6cfd182e2aef89
      Public key (hex): 0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48
      Account ID:       0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48
      SS58 Address:     5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
    ```

    I don't fully understand how to specify account names, but I know
    that `//Alice` and `//Bob` are known test accounts. This `subkey inspect`
    command gives the account id and secret key.

    The `subkey` command is part of the substrate repo and [the command
    for installing it is a bit awkward][subkeycommand], but easy enough
    to copy-paste.

2) The [polkadot explorer][pex] can be configured to connect to my
   local devnet!

   This seems pretty huge. I can see account ID's here, and hopefully
   even more useful info about what is going on inside my devnet.

[subkeycommand]: https://substrate.dev/docs/en/knowledgebase/integrate/subkey#build-from-source
[pex]: https://polkadot.js.org/apps/#/explorer



Both canvas-ui and the polkadot.js explorer cache information about
code hashes (and probably other things) they've seen, but
which don't actually exist on the chain they are currently connected to.
This is maddening every time I restart my devnet and see a bunch of
hashes of old contracts that I can't distinguish from the new contracts.

These apps should either verify these code hashes actually exist on
chain, or at least give me a big red "reset" button to purge all
this incorrect info.

At some point I had canvas-ui showing me two "flipper" contracts,
and I didn't know which one was real,
so I told it to "forget" both of them.
Then I tried to redeploy flipper,
but of course flipper was already deployed so I got an error,
and now I don't know the correct address of flipper,
and can't add it back to the UI and I'm stuck,
have to shut down my devnet and restart it.

I now have a habit of going through all the canvas-ui tabs
and "forgetting" everything every time I restart canvas-node.

At least the polkadot explorer says "not on-chain" for code
hashes that don't actually exist.

Another note: there are many opportunities in both UIs to
"add a code hash", but it seems like this option is useless
unless you also provide the contract metadata.
TODO why this sucks

After some experimenting I learn that
constructing a contract creates a new account;
that is, a contract is not associated with the account
of the user that creates it; a contract has its own account.



We execute transactions in the canvas-ui, but no
events seem to register in the explorer ui.

We're going to have to add logging _everywhere_
to have any clue what the hell is going on.



### Debugging cross-contract calls

We ran into our first big blocker:
we can't figure out how to call another contract via interface.

The ink repo contains examples of defining traits,
via `#[trait_definition]`
and calling contracts through them,
but those examples all still have direct access to the _implementations_
of those traits.
That is too limiting for our needs.

By browsing the API docs we came across [CallBuilder],
which appears to call an arbitrary method in another contract,
given that contract's account id, a known _method selector_,
which is a unique ID used to identify a method,
and the configuration for the args, return value,
gas, and other things.

[CallBuilder]: https://paritytech.github.io/ink/ink_env/call/struct.CallBuilder.html

We tried to use this and failed.
And failed.

And failed.

For days.

We were discouraged,
and it was hard to come back to this problem,
but we can't make any progress without solving it.

So that's today's mission.


### But first, updating our tools

Since the canvas tools are immature,
and it has been over a week since I last used them,
I first pull and rebuild `canvas-node`,
`cargo-contract`, and `canvas-ui`.

I recall that last time I built `canvas-ui` it
would not successfully connect to my `canvas-node` instance,
and I hope things have changed.

After running `git pull` on `canvas-node`,
I see no new commits, which is surprising &mdash;
no changes since December 2!

Development must not happen on the master branch.
Wait, there's no obvious development branch either.
It seems that canvas-node really hasn't changed in over a month.

With consideration,
this is pretty reasonable,
since canvas-node is just an instantiation of the gigantic
substrate toolkit &mdash;
it is only 1200 lines of code &mdash;
so most development goes directly into substrate.

This though makes me wonder if I can update canvas's substrate revision
(also if I should).
It is something I am curious to attempt,
but doesn't seem prudent right now,
since presumably most everybody else using canvas-node is using the substrate revisions in Cargo.lock.

`cargo-canvas` has updates and I install them from the source directory with

```
git pull
cargo install --path .
```

I notice that while I usually run `canvas-node` directly from the source directory via `cargo`,
I have been installing `cargo-canvas`.
I think this is because the two have different usage models:
`canvas-node` is a server and I mostly just leave it running in a tmux window;
`cargo-canvas` though is a tool I might need to run in any of multiple windows
for different purposes.

`canvas-ui` also has updates and I build them with ... (I have to look this up in `package.json`):

```
git pull
yarn install
```

and then I try the `yarn build` command listed in `package.json`,
thinking that it is probably one of the prerequisites to `yarn start`,
but it doesn't seem so,
as it doesn't work, printing

```
command not found: node_modules/@polkadot/dev/scripts/polkadot-dev-build-ts.js
```

and exiting with an error code.
That's fine: I know how to run `canvas-ui` with `yarn start`.
It takes a long time to start up though,
and I was hoping I could do some of that webpacking or whatever ahead of time.

While I'm tinkering with building my tooling,
I wonder if I can install an updated [`subkey`] tool from source.

I `cd` into my substrate source directory and run

```
git pull
cargo install --path bin/utils/subkey/
```

It works. I have updated tools.
Enough procrastinating; time to solve problems.


### And then debugging cross-contract calls

What happens when we try to call a method via `CallBuilder` is
that `canvas-node` logs (when `-lruntime=debug` is passed on the command line):

```
2021-01-14 16:34:36.014  DEBUG tokio-runtime-worker runtime:DispatchError
2021-01-14 16:34:36.014  DEBUG tokio-runtime-worker runtime:8
2021-01-14 16:34:36.014  DEBUG tokio-runtime-worker runtime:17
2021-01-14 16:34:36.014  DEBUG tokio-runtime-worker runtime:ContractTrapped
2021-01-14 16:34:36.014  DEBUG tokio-runtime-worker runtime:PostInfo:
2021-01-14 16:34:36.014  DEBUG tokio-runtime-worker runtime:actual_weight=
2021-01-14 16:34:36.014  DEBUG tokio-runtime-worker runtime:7172485790
2021-01-14 16:34:36.014  DEBUG tokio-runtime-worker runtime:pays_fee=
2021-01-14 16:34:36.014  DEBUG tokio-runtime-worker runtime:Yes
```

As the only clue to what's happening, it's some pretty horrible feedback.
By ripgrepping the substrate code [for `DispatchError`][fde] we gather that

- "8" is a "module index", here the contracts module, though it isn't explicitly stated
- "17" is a module-specific error number, here it corresponds to [`ContractTrapped`].
  This _is_ indicated in the output, though the connection between "17" and `ContractTrapped`
  is not explicit.
- "actual_weight" is part of [`PostDispatchInfo`] and indicates the
  "actual weight consumed by a call or `None` which stands for the worst case static weight".

[fde]: https://github.com/paritytech/substrate/blob/7a79f54a5d92cecba1d9c1e4da71df1e8a6ed91b/primitives/runtime/src/lib.rs#L404
[`ContractTrapped`]: https://github.com/paritytech/substrate/blob/7a79f54a5d92cecba1d9c1e4da71df1e8a6ed91b/frame/contracts/src/lib.rs#L399
[`PostDispatchInfo`]: https://github.com/paritytech/substrate/blob/7a79f54a5d92cecba1d9c1e4da71df1e8a6ed91b/frame/support/src/weights.rs#L329

With our "actual weight" looking like a pretty large number,
our best guess right now is that we just didn't spend enough gas
to make the call.

For our experiment today

We create a repo just for testing `CallBuilder`:

> https://github.com/Aimeedeer/game-test

Our `CallBuilder` right now looks like:

```rust
            let return_value: bool = build_call::<DefaultEnvironment>()
                .callee(program_id) 
                .gas_limit(50)
                .transferred_value(10)
                .exec_input(
                    ExecutionInput::new(Selector::new([0xDE, 0xAD, 0xBE, 0xFF]))
                )
                .returns::<ReturnType<bool>>()
                .fire()
                .unwrap();
```

The `program_id` is the `AccountId` of the flipper contract,
provided as an argument to the caller.
The selector is one that we've given to the flipper's `get`
method with the `selector` attribute.

When we run our contract we see 

```
2021-01-16 15:03:42.007 DEBUG tokio-runtime-worker runtime:DispatchError
2021-01-16 15:03:42.007 DEBUG tokio-runtime-worker runtime:8
2021-01-16 15:03:42.007 DEBUG tokio-runtime-worker runtime:6
2021-01-16 15:03:42.007 DEBUG tokio-runtime-worker runtime:OutOfGas
2021-01-16 15:03:42.007 DEBUG tokio-runtime-worker runtime:PostInfo:
2021-01-16 15:03:42.007 DEBUG tokio-runtime-worker runtime:actual_weight=
2021-01-16 15:03:42.007 DEBUG tokio-runtime-worker runtime:50000000
2021-01-16 15:03:42.007 DEBUG tokio-runtime-worker runtime:pays_fee=
2021-01-16 15:03:42.007 DEBUG tokio-runtime-worker runtime:Yes
```

Hey!

This is a different error: now we're `OutOfGas`, not `ContractTrapped`.

And `actual_weight` looks suspicious:
it is 50 million;
and our call builder set the gas limit to "50".
Is `gas_limit` specified in millions?

After some experimentation the answer seems to be "no".
and these two numbers are not directly connected,
and we were putting some number beginning with "5" as
the gas limit in canvas-ui.

We have become completely baffled by how units are specified
in various places in the code and UI.
Parts of the code say the units are "Unit",
like when specifying the amount paid to a contract;
parts say "M",
like when specifying the gas paid to run a contract,
which we assume is a million units;
but units don't seem to be the actual underlying integer number of units,
since units are divisible to some much smaller number.

We think we are not providing enough gas to the call builder
and increase that number;
we trying increasing the gas we provide to the callee
when invoking it in the canvas-ui.
No matter what we do it's `OutOfGas` or `ContractTrapped`.

We think maybe our calling account doesn't have enough
"units" to execute the contract,
so we transfer all of the money in the ALICE, ALICE_STASH,
and BOB_STASH accounts to BOB, and BOB still can't seem to
get the contract to execute successfully.

I ask some questions in the ink matrix channel,
and I feel extremely frustrated.

> Every combination of attempts I make results in either an "OutOfGas" or "ContractTrapped" error

> We're very confused about units and gas and weight. How many underlying integer units are in a "unit" (how divisible is a unit)? Does "Max Gas Allowed (M)" mean the amount of millions of "unit"s payed from the caller to run the contract?

> When our default devnet accounts say alice has "1,152" "units" does that man she doesn't have the millions of gas necessary to execute a contract?

Robin responds:

> To clarify: cross-contract calls are possible using the CallBuilder API that should ideally not be used since it is a rather low-level abraction API. Instead users should try using the interfaces allowed for by the ink_lang macros.

> Users generally won't need to use the CallBuilder API for their everyday smart contracts; it is only really required for niche use cases that you can inspect in our multi_sig example contract.

> The cross-chain calling is something completely different and requires support in lower-levels of abstraction than ink! itself.

> I am sorry to hear about your confusing user experiences with the CallBuilder API. Answering the question "how much gas is enough?" isn't easy since it depends on the chain, its configuration and also on which version of Substrate it is based (defaults). E.g. for the Canvas network we resolved that approx 5 CANs should be sufficient to operate smart contracts for a week on our chain. The Alice test account normally has plenty of funds and will likely not require any more if the chain configuration for gas prices is sane.

This doesn't quite answer most of my questions,
besides asserting that the default accounts should have enough "units" to pay for gas.
I still don't have a clue how divisible units are or whether gas is paid in millions.
And if I'm not supposed to use `CallBuilder` to make cross-contract calls,
then I don't know what I am supposed to use instead.

I ask:

> @Robin I don't see in ink_lang how to call a cross-contract method for contracts that I don't a-priori have the implementation for. I know the signature of the method I want to call, and the account id of the contract i want to call, but don't know the concrete interface. Is there a way to make that call without callbuilder?

During this time we do figure out one thing we were doing wrong:
we were calling `unwrap` on the results of executing our cross-contract call,
and _that_ was what was triggering the `ContractTrapped` error.
When we stopped unwrapping and printed the result we could see
that the call was returning a `Err(Decode(Error))`.

In the meantime I decide to debug-log _everything_ in the environment I can to try to understand
what units are what,
and how the numbers I input in canvas-ui translate to the numbers my code sees.

I try to do this in the caller's method:

```rust
ink_env::debug_println(&format!("env: {:?}", self.env()));
ink_env::debug_println(&format!("weight_to_fee(gas_left): {}", self.env().weight_to_fee(self.env().gas_left() as _)));
```

Just dump the environment.
When I try to upload this to the chain the UI reports an error, and the log says

```
2021-01-16 22:29:48.038  DEBUG tokio-runtime-worker runtime:DispatchError                                                                                                             2021-01-16 22:29:48.038  DEBUG tokio-runtime-worker runtime:Invalid module for imported function
2021-01-16 22:29:48.038  DEBUG tokio-runtime-worker runtime:PostInfo:                                                                                                                 2021-01-16 22:29:48.038  DEBUG tokio-runtime-worker runtime:actual_weight=
2021-01-16 22:29:48.038  DEBUG tokio-runtime-worker runtime:max-weight
2021-01-16 22:29:48.039  DEBUG tokio-runtime-worker runtime:pays_fee=
2021-01-16 22:29:48.039  DEBUG tokio-runtime-worker runtime:Yes
```

Further experiments with logging the environment don't exhibit this error,
so I do some work reducing this error.
I create a contract that consists only of this:

```rust
    #[ink(storage)]
    pub struct Game { }

    impl Game {
        #[ink(constructor)]
        pub fn default() -> Self {
            Game { }
        }

        #[ink(message)]
        pub fn run_game_test(&mut self) {
            ink_env::debug_println(&format!("env: {:?}", self.env()));
            ink_env::debug_println(&format!("weight_to_fee(gas_left): {}", self.env().weight_to_fee(self.env().gas_left() as _)));
        }
    }
```

It can't be uploaded. Doing so results in the above `DispatchError`.

If I remove the _second_ `debug_println` I can upload and execute this contract.
So this method now is doing nothing but 

```rust
            ink_env::debug_println(&format!("env: {:?}", self.env()));
```

It logs

```
DEBUG tokio-runtime-worker runtime:env: EnvAccess
```

So not useful.

Changing the method to do only

```rust
            ink_env::debug_println(&format!("weight_to_fee(gas_left): {}", self.env().weight_to_fee(self.env().gas_left() as _)));
```

Results in a contract that again can't be uploaded.

Anyway, I'm going to move on and continue trying to log whatever bits of the environment I can,
in our original contract method.

I try to log just about every accessor on `self.env()`,
and the only one that results in a contract that successfully uploads is `self.env().caller()`.
Once again I am completely baffled.
Clearly I am doing something wrong because this is just broken.

During all this I note repeatedly that today the deployed canvas-ui's icons are all missing,
replaced by those replacement unicode number filler characters.
It's getting a bit annoying.

So much brokenness.
It's super frustrating.

I make a note that some of these things are probably things _I_ can help fix,
at some point,
once I feel like I have an understanding of how things are supposed to work.
Right now though I don't know what I don't know,
and am just flailing around,
feeling like nothing works.

Anyway today I went backwards,
and am fed up.

So that's it.
Time for a break.


### Another try at cross-contract calls with `CallBuilder`

This will be our 4th attempt to successfully call another conract with `CallBuilder`.

In the meantime Robin responded to more of my questions in chat.

From that I learned that I should definitely be able to do what I'm attempting,
but more importantly that I can set the gas limit to 0 in a cross-contract call,
and that will just make the gas limit unlimited.
That will remove one of the hurdles we've had during development &mdash;
figuring out how much gas to provide in the canvas-ui.

TODO












## Connecting to our contract with polkadot-js

It's strange that the JS compononts are "polkadot"-branded,
where so many other things in this ecosystem are "substrate"-branded.

I try not to use npm unless I have to.
Trying to create a simple frontend using plain HTML and JavaScript.
All the documentation for the polkadot JS API assumes the use of node/yarn.
I am trying to figure out how to use webpack to package up @polkadot/api so I can use it outside npm,
but don't know how.

I have previously succeeded in this with web3.js and ipfs.js,
but don't really remember how,
and don't see any obvious evidence that the polkadot APIs are ready to webpack.

I ask in #ink:matrix.parity.io

> I'm not very familiar with npm programming, and I want to use the
  @polkadot/api from a non-npm script in the browser. Is it possible to use
  webpack to package @polkadot/api into a standalone javascript file that I can
  use outside of npm. Any hints how to do that?

In the meantime I give up trying to package polkadot-js for use
outside of npm,
and try to set up a yarn app that will let me import the the library
in the expected way.

As someone mostly unfamiliar with npm I immediately encounter more problems.
I know how to add `@polkadot/api` to a yarn project,
but I don't know how to set yarn up to run a server on `yarn start`.
From Googling, as with most things in the npm ecosystem,
there seem to be many options.

Similar to the Ethereum docs,
I'm finding that the polkadot-js docs completely gloss over topics
about setting up npm/parn projects,
and I am completely lost.

I know that I can't expect Polkadot to teach me npm,
just like I can't expect them to teach me Rust,
but this has been a huge problem for me every time I try to use modern JavaScript.

On https://polkadot.js.org/docs/api/examples/promise/ it says

> "From each folder, run yarn to install the required dependencies and then run
  yarn start to execute the example against the running node."

But there are no "folders" in this documentation.
Is there a link to actual, complete, ready-to-run source code I'm missing?

I'm quite frustrated.

I additionally ask in the "#ink" channel if there's a basic
yarn template for using the polkadot JS API's.

Dan Shields comes through with a link to

> https://github.com/substrate-developer-hub/substrate-front-end-template

I recall seeing this before and must have forgotten about it.
I'll try to reboot my web efforts from this template.

Well, maybe I'll just use it for hints.
It's a react app, and I really don't want to learn react right now.
So complex.

I am going to try instead using `webpack-dev-server` for my `yarn start` command.

I eventually follow the [webpack "Getting Started" guide][wpgs].
I'm real rak shaving now.

[wpgs]: https://webpack.js.org/guides/getting-started/

While running weback with my script that imports "@polkadot/api" I run into this error:

```
ERROR in ./node_modules/scryptsy/lib/utils.js 1:15-32
Module not found: Error: Can't resolve 'crypto' in '/home/ubuntu/contract-game/www2/node_modules/scryptsy/lib'

BREAKING CHANGE: webpack < 5 used to include polyfills for node.js core modules by default.
This is no longer the case. Verify if you need this module and configure a polyfill for it.

If you want to include a polyfill, you need to:
        - add a fallback 'resolve.fallback: { "crypto": require.resolve("crypto-browserify") }'
        - install 'crypto-browserify'
If you don't want to include a polyfill, you can use an empty module like this:
        resolve.fallback: { "crypto": false }
 ```

To fix this it seems I have to add a `webpack.config.js` and configure it per the
[webpack "resolve" instructions][wri].
After creating `webpack.config.js` I can more-or-less copy-paste the suggestion
straight from the command line.
My new `webpack.config.js` looks like

```js
const path = require("path");

module.exports = {
    entry: './src/index.js',
    output: {
        filename: 'main.js',
        path: path.resolve(__dirname, 'dist'),
    },
    resolve: {
        fallback: {
            "crypto": require.resolve("crypto-browserify")
        }
    }
};
```

I also have to `yarn add crypto-browserify`.
Once I do I see more similar errors about polyfills,
and when I finally have webpack working I have
three polyfills in my `webpack.config.js`:

```js
        fallback: {
            "buffer": require.resolve("buffer"),
            "crypto": require.resolve("crypto-browserify"),
            "stream": require.resolve("stream-browserify")
        }
```

[wri]: https://webpack.js.org/configuration/resolve/

Hours and hours go by...

I'm using webpack 5,
which doesn't do a bunch of node polyfills when it compiles
for the browser.
I think that's a big part of my pain.

After tons of Googling and hacking I finally manage
to load the polkadot JS API in my mostly vanilla JS
web page.

I have to do a lot of hacks.

At the end my `webpack.config.js` looks like

```js
const path = require("path");
const webpack = require("webpack");

module.exports = {
    mode: "development",
    entry: './src/index.js',
    output: {
        filename: 'js/bundle.js',
        path: path.resolve(__dirname, 'dist'),
    },
    resolve: {
        fallback: {
            "buffer": require.resolve("buffer"),
            "crypto": require.resolve("crypto-browserify"),
            "stream": require.resolve("stream-browserify")
        }
    },
    plugins: [
        new webpack.ProvidePlugin({
            Buffer: ['buffer', 'Buffer'],
        }),
    ]
};
```

That `plugins` section is new and mysterious,
just copy pasted from [some commit to some other
software project][webhack].

[webhack]: https://github.com/duplotech/create-react-app/commit/d0be703d40cd4bc32cd91128ba407a138c608243#diff-8e25c4f6f592c1fcfc38f0d43d62cbd68399f44f494c1b60f0cf9ccd7344d697

I also have this lovely garbage in my HTML header
before loading my webpack bundle:

```html
  <script>
    let global = window;
    let process = {
      "versions": null
    };
  </script>
```

Yup.

Somebody tell me what I'm doing wrong. Please.


## Some thoughts

### First, some hopefulness

As I learn about Substrate and the corners of its ecosystem,
I do feel a bit giddy.
The high level docs are pretty enlightening to read,
and I find myself keeping them open in a browser tab,
and just perusing them sometimes;
I think they provide some good insight into the structure
of modern programmable blockchains generally,
not just Substrate.
And when I discovered that Polkadot explorers,
specifically the one at [polkadot.js.org][pex],
can more-or-less seemlessly work with any substrate chain,
even my own dev chain,
I was kind of blown away.

I can see the possibility of many people building
new and different chains, that can all interoperate,
on one powerful toolkit.

It seems like it could be fun to be a part of.


### Next, some venting

Right now though, it is not fun.
Writing applications on blockchains is not fun,
both generally,
and specifically in the case of Substrate,
for many reasons.

Smart contract programming continues to be challenging.
Learning new things always is I know,
but I'm finding myself surprised at how long it is
taking me to turn the corner and understand what I'm doing.

Programming a smart-contract-based application for
Substrate requires at least three kinds of expertise:

- Rust programming, and its idiosyncracies and ecosystem
- Node programming, and its complexly layered ecosystem
- Blockchain technology, and its alien and rapidly changing
  form of software construction and interaction

If one doesn't literally need to be an "expert" in
these things, then at least one needs intermediate experience
in both Rust and Node to make any kind of satisfying progress
on the path to learning Substrate.

Each of these three technologies is intimidating,
but combining them is brutal.

And before much longer smart contract programmers are going
to need to understend zero-knowledge cryptography too.
TODO

TODO mention tokenomics?

How can this complexity ever be reduced to something an average programmer can
manage? The blockchain ecosystem just seems to be exploding in complexity.

I can hardly imagine the path forward,
but I hope one day there is a contraction and consolidation
of all the complexity,
of the extremely many slightly-different models of permissionless
distributed programming evolving all over the industry.


This is all without even addressing the horrifically
immature state of developer tooling in blockchain ecosystems.
It will be a decade before the smart contract programming experience,
on any chain,
is as convenient as the traditional programming experience.
There just aren't enough resources dedicated to it,
nor reason to bother dedicating those resources.
With blockchain programming having such an incredibly
small developer audience, given the limited application domains,
and the extreme expertise necessary to participate,
the momentum isn't there to sustain the TODO

The blockchain world appears to be filled with developer tooling
projects that fulfill their grant's contractual requirements,
and then stop evolving.

It's an entire universe of 90% solutions.


### It's ok - I'm learning

My partner is learning to program right now,
and I am patiently watching her go through all the difficulties
every programmer goes through as they discover basic computer
science topics. I try to be empathetic by remembering
my learning experience so long ago, of hacking out simple
programs without understanding.

I am feeling a lot of that pain again now as a smart contract learner.
I have been following the progress of blockchain tech for
quite a long time,
and doing some hands on programming on and off for
nearly a year.
That combined with my long background in writing software
gives me a lot of intuition about how software running
on the blockchain _should_ work.
But I still feel lost.
I am still not seeing the big picture.

I am going to continue trying though,
and hoping for a moment of enlightenment,
where it all feels right,
and where the fun begins.


## TODO

- tips
  - installing tools
    - canvas-node
    - cargo-contract
    - canvas-ui
    - subkey
- docs.rs!
- uploading and running contracts via web ui sucks

Since last hacking on canvas,
I've installed the polkadot.js extension to Brave.
Now when I navigate to canvas-ui,
I get asked by the extension to approve the UI,
and whether I do or don't,
I can't connect to the local devnet.
