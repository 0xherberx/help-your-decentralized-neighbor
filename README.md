Help your decentralized neighbor!
==================

This simple dapp enables you to help people in a decentralized way. Make a simple donation and let randomness decide which person in need to help, it could be your next door neighbor!

Concept
-------------------------------------

Do you want to help someone but don't have time to do it? Do you want to help but you don't know who to help?
The "Help your decentralized neighbor!" dapp allows anyone, using the decentralized world, to help people in need with transparency, anonymity and globally.
Through a simple anonymous donation you will increase the total prize pool and on the first of each month the smart contract will choose one lucky decentralized neighbor to help.
Transparency and security are guaranteed by the smart contract and thanks to the NEAR blockchain ecosystem there is no need for intermediaries or third parties anymore.
Don't be greedy! A small action for a great purpose!

Example Story
Below is an example of three users' experience: Alice, Bob and Carol.
	1. Alice is always very busy at work. Being generous she would like to help others but she doesn't have much time to do it. So she accesses the dapp and in a few clicks she decides to donate 10 NEAR tokens.
	2. Bob has a desire to help someone but can't decide who. Also, he doesn't want to waste time with intermediaries or third parties. So he enters the new dapp and donates 40 NEAR tokens, letting randomness choose the lucky neighbor.
	3. Carol has two children and she just lost her job due to these tough times. She needs help paying the rent, so she learns about the "Help your decentralized neighbor!" dapp and she decides to join the lottery.
	4. It's the first of the month and like every month the smart contract chooses a neighbor to help. Carol got lucky this month and she automatically receives 99% of the prize pool. She is happy and she can finally pay the rent. She is also grateful and in better times she will donate what she can to help someone in need.

Contract
-------------------------------------

There is only one smart contract that manages all the backend system.
The two main features for users are:
	• add_donation: this feature adds the donated NEAR tokens to the final prize pool amount
	• join_the_lottery: the user decides to register his account to participate in the lottery
The following feature will be activated every first of the month:
	• pick_lottery_winner_and_withdraw: the smart contract will choose an account among those registered and transfer the prize pool to the lucky winner.
Randomness in the lottery is ensured by the NEAR random seed feature, it comes from the validator that produced the block signing the previous block-hash with their private key.
The recurring task of picking the winner and transferring the prize pool is scheduled using cron.cat, a decentralized scheduling for blockchain transactions.

Installation
	1. Close this repo
	2. Run npm install
Run npm test


Quick Start
===========

If you haven't installed dependencies during setup:

    npm install


Build and deploy your contract to TestNet with a temporary dev account:

    npm run deploy

Test your contract:

    npm test

If you have a frontend, run `npm start`. This will run a dev server.


Exploring The Code
==================

1. The smart-contract code lives in the `/contract` folder. See the README there for
   more info. In blockchain apps the smart contract is the "backend" of your app.
2. The frontend code lives in the `/frontend` folder. `/frontend/index.html` is a great
   place to start exploring. Note that it loads in `/frontend/index.js`,
   this is your entrypoint to learn how the frontend connects to the NEAR blockchain.
3. Test your contract: `npm test`, this will run the tests in `integration-tests` directory.


Deploy
======

Every smart contract in NEAR has its [own associated account][NEAR accounts]. 
When you run `npm run deploy`, your smart contract gets deployed to the live NEAR TestNet with a temporary dev account.
When you're ready to make it permanent, here's how:


Step 0: Install near-cli (optional)
-------------------------------------

[near-cli] is a command line interface (CLI) for interacting with the NEAR blockchain. It was installed to the local `node_modules` folder when you ran `npm install`, but for best ergonomics you may want to install it globally:

    npm install --global near-cli

Or, if you'd rather use the locally-installed version, you can prefix all `near` commands with `npx`

Ensure that it's installed with `near --version` (or `npx near --version`)


Step 1: Create an account for the contract
------------------------------------------

Each account on NEAR can have at most one contract deployed to it. If you've already created an account such as `your-name.testnet`, you can deploy your contract to `near-blank-project.your-name.testnet`. Assuming you've already created an account on [NEAR Wallet], here's how to create `near-blank-project.your-name.testnet`:

1. Authorize NEAR CLI, following the commands it gives you:

      near login

2. Create a subaccount (replace `YOUR-NAME` below with your actual account name):

      near create-account near-blank-project.YOUR-NAME.testnet --masterAccount YOUR-NAME.testnet

Step 2: deploy the contract
---------------------------

Use the CLI to deploy the contract to TestNet with your account ID.
Replace `PATH_TO_WASM_FILE` with the `wasm` that was generated in `contract` build directory.

    near deploy --accountId near-blank-project.YOUR-NAME.testnet --wasmFile PATH_TO_WASM_FILE


Step 3: set contract name in your frontend code
-----------------------------------------------

Modify the line in `src/config.js` that sets the account name of the contract. Set it to the account id you used above.

    const CONTRACT_NAME = process.env.CONTRACT_NAME || 'near-blank-project.YOUR-NAME.testnet'



Troubleshooting
===============

On Windows, if you're seeing an error containing `EPERM` it may be related to spaces in your path. Please see [this issue](https://github.com/zkat/npx/issues/209) for more details.


  [create-near-app]: https://github.com/near/create-near-app
  [Node.js]: https://nodejs.org/en/download/package-manager/
  [jest]: https://jestjs.io/
  [NEAR accounts]: https://docs.near.org/concepts/basics/account
  [NEAR Wallet]: https://wallet.testnet.near.org/
  [near-cli]: https://github.com/near/near-cli
  [gh-pages]: https://github.com/tschaub/gh-pages
