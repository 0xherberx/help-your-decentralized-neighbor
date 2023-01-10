Help Your Decentralized Neighbor!
==================

This simple dapp enables you to help people in a decentralized way. Make a simple donation and let randomness decide which person in need to help, it could be your next door neighbor!

Concept
-------------------------------------

Do you want to help someone but don't have time to do it? Do you want to help but you don't know who to help?
This dapp allows anyone, using the decentralized world, to help people in need with transparency, anonymity and globally.

Through a simple anonymous donation you will increase the total prize pool and on the first of each month the smart contract will choose one lucky decentralized neighbor to help.

Transparency and security are guaranteed by the smart contract and thanks to the NEAR blockchain ecosystem there is no need for intermediaries or third parties anymore.

Don't be greedy! A small action for a great purpose!

### Example Story

Below is an example of three users' experience: Alice, Bob and Carol.
1. Alice is always very busy at work. Being generous she would like to help others but she doesn't have much time to do it. So she accesses the dapp and in a few clicks she decides to donate 10 NEAR tokens.
2. Bob has a desire to help someone but can't decide who. Also, he doesn't want to waste time with intermediaries or third parties. So he enters the new dapp and donates 40 NEAR tokens, letting randomness choose the lucky neighbor.
3. Carol has two children and she just lost her job due to these tough times. She needs help paying the rent, so she learns about the "Help your decentralized neighbor!" dapp and she decides to join the lottery.
4. It's the first of the month and like every month the smart contract chooses a neighbor to help. Carol got lucky this month and she automatically receives 99% of the prize pool. She is happy and she can finally pay the rent. She is also grateful and in better times she will donate what she can to help someone in need.


Quick Start
===========

1. Clone this repo
2. Run `npm install` (install dependencies)
3. Run `npm run deploy` (build the contract and deploy it in a testnet account)
4. Run `npm start` (start the web application to interact with your smart contract)

Scripts
-------------------------------------

See below for more convenience scripts...

### Build the smart contract and the web application

```
npm run build              // build both contract and frontend
npm run build:contract     // build contract only
npm run build:web          // build frontend only
```

### Run unit tests and integration tests

```
npm test                    // run both unit tests and integration tests
npm run test:unit           // run unit tests only
npm run test:integration    // run integration tests only
```


Exploring The Code
==================

This repository includes:
- One smart contract (backend)
- A react application (frontend)
- Unit tests and integration tests for the contract
- UI wireframes and/or mockups for the frontend
- Scripts for building, testing, and deploying contracts

Contract
-------------------------------------

There is only one smart contract that manages all the backend system.

The two main features for users are:
- add_donation: this feature adds the donated NEAR tokens to the final prize pool amount
- join_the_lottery: the user decides to register his account to participate in the lottery

The following feature will be activated every first of the month:
- pick_lottery_winner_and_withdraw: the smart contract will choose an account among those registered and transfer the prize pool to the lucky winner.

Randomness in the lottery is ensured by the NEAR random seed feature, it comes from the validator that produced the block signing the previous block-hash with their private key.

The recurring task of picking the winner and transferring the prize pool is scheduled using cron.cat, a decentralized scheduling for blockchain transactions.

Web Application
-------------------------------------

The frontend part is built on ReactJS and uses NEAR API JS to connect and communicate with the NEAR blockchain.
/frontend/assets/SignInPage.png
/frontend/assets/MainPage.png

Future Development
==================
Some ideas for future enhancements development:
- Lottery history
- Monthly hall of fame for donors
- Top 3 donations
- Location statistics
- Winners can claim with a message