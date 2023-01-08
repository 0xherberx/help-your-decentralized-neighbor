// React
import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';

// NEAR
/*
  We implement a ./near-wallet.js module, where we abstracted the wallet selector
  into a Wallet object to simplify using it
 */
import { Wallet } from './near-wallet';
/* 
  The process.env property returns an object containing the user environment.
  Example:
  {
    SHELL: '/usr/local/bin/bash',
    USER: 'maciej',
    PWD: '/Users/maciej',
    HOME: '/Users/maciej',
    _: '/usr/local/bin/node'
  }
 */
const CONTRACT_ADDRESS = process.env.CONTRACT_NAME;

// When creating the wallet you can optionally ask to create an access key
// Having the key enables to call non-payable methods without interrupting the user to sign
const wallet = new Wallet({ createAccessKeyFor: CONTRACT_ADDRESS })

// Setup on page load
/* 
override the window.onload method with a function that calls the wallet.startUp() method.
Such method returns if the user is already signed-in
 */
window.onload = async () => {
  const isSignedIn = await wallet.startUp();
  console.log(isSignedIn);
  console.log(CONTRACT_ADDRESS);
  const root = ReactDOM.createRoot(document.getElementById('root'));
  root.render(<App isSignedIn={isSignedIn} contractId={CONTRACT_ADDRESS} wallet={wallet} />);
}
