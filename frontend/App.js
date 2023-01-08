import 'regenerator-runtime/runtime';
import React, { isValidElement } from 'react';
//import { EducationalText, SignInPrompt, SignOutButton } from './ui-components';
import './sass/App.scss';
import './assets/global.css';
import Navbar from 'react-bootstrap/Navbar';
import Container from 'react-bootstrap/Container';
import Nav from 'react-bootstrap/Nav';
import Button from 'react-bootstrap/Button';
import Modal from "react-bootstrap/Modal";
import SignInPrompt from './components/SignInPrompt';
import SignOutButton from './components/SignOutButton';
import { utils } from 'near-api-js';

export default function App({ isSignedIn, contractId, wallet }) {
  const [totalDonations, setTotalDonations] = React.useState(() => 0);
  const [isRegisteredForLottery, setIsRegisteredForLottery] = React.useState(() => false);
  const [showJoinedLotteryAlert, setShowJoinedLotteryAlert] = React.useState(() => false);
  const [formData, setFormData] = React.useState(() => ({
    donation1: 5,
    donation2: 10,
    donation3: 20,
    donation4: 0,
    selected: ""
  }));

  function updateTotalDonation() {
    async function get_total_global_amount_donated() {      
      const total_global_amount_donated = await wallet.viewMethod({method: 'get_total_global_amount_donated', contractId});
      // converts yoctoNEAR (10^-24) amount into NEAR
      const amountInNEAR = utils.format.formatNearAmount(total_global_amount_donated);
      console.log(amountInNEAR);
      setTotalDonations(amountInNEAR);
    }
    
    get_total_global_amount_donated();
  }

  function isAccountRegisteredForLottery() {
    async function is_account_registered_for_lottery() {      
      const is_registered = await wallet.viewMethod({
        method: 'is_account_registered_for_lottery',
        args: { account_to_check: wallet.accountId },
        contractId
      });      
      setIsRegisteredForLottery(is_registered);
    }
    
    is_account_registered_for_lottery();
  }

  React.useEffect(() => {
    /*async function getGreeting() {
      const currentGreeting = await wallet.viewMethod({method: 'get_greeting', contractId});
      console.log(currentGreeting);
    }    
    getGreeting();*/
    if(isSignedIn) {
      updateTotalDonation();
      isAccountRegisteredForLottery();
    }    
  }, [])
  
  console.log('render App');
  console.log("isSignedIn: " + isSignedIn);

  function handleSubmit(event) {    
    event.preventDefault();
    const donationAmount = formData.selected != "" ? formData[formData.selected] : 0;
    console.log('Donated amount in NEAR: ' + donationAmount);
    // Convert to yocto - 1Ⓝ = 10^24yocto
    const amountInYocto = utils.format.parseNearAmount(donationAmount.toString());
    console.log('Donated amount in yocto NEAR: ' + amountInYocto);

    // Calling a payable method
    async function add_donation() {
      const response = await wallet.callMethod({
        method: "add_donation",
        args: { text_message: "test msg" },
        contractId,
        deposit: amountInYocto
      })      
      const responseData = await response.json();
      console.log("donated!" + responseData);  
      updateTotalDonation();
    }
    
    add_donation();
  }

  function selectDonationAmount(event) {
    const {name, value, type} = event.target;
    setFormData(oldFormData => ({
      ...oldFormData,
      [name]: value != "" ? parseFloat(value) : 0,
      selected: value != "" && value != "0" ? name : ""
    }))
  }
  
  function joinTheLottery(event) {
    async function join_the_lottery() {
      const response = await wallet.callMethod({
        method: "join_the_lottery",
        args: { account_to_register: wallet.accountId },
        contractId
      })      
      //const responseData = await response.json();
      console.log("joined lottery!");
    }
    
    join_the_lottery();
    setIsRegisteredForLottery(true);
    setShowJoinedLotteryAlert(true);
  }

  console.log(formData);

  return (
    <main>
      <div className="main-body">
        {
          isSignedIn
          ?
          <section className="donation-page">
            <div className="donation-page--header">
              <span className="donation-page--total">
                Total Donations: {Number(totalDonations).toFixed(2)} NEAR
              </span>
              <SignOutButton accountId={wallet.accountId} handleClick={() => wallet.signOut()} />
            </div>
            <h3 className="donation-page--desc">
                Every month, one lucky person will be picked out and will receive 99% of the total donations.<br/>
                Please this amount is to help people in need.<br/>
                Don't be greedy!<br/>
                Help your decentralized neighbor!
            </h3>
            <form className="donation-page--form" onSubmit={handleSubmit}>
              <div className="donation-page--inputs">
                <input
                  type="number"
                  placeholder="5"
                  className={`donation-page--amount no-spinner ${formData.selected === "donation1" ? "donation-selected" : ""}`}
                  name="donation1"
                  onClick={selectDonationAmount}
                  value={formData.donation1}
                  readOnly
                />
                <input
                  type="number"
                  placeholder="10"
                  className={`donation-page--amount no-spinner ${formData.selected === "donation2" ? "donation-selected" : ""}`}
                  name="donation2"
                  onClick={selectDonationAmount}
                  value={formData.donation2}
                  readOnly
                />
                <input
                  type="number"
                  placeholder="20"
                  className={`donation-page--amount no-spinner ${formData.selected === "donation3" ? "donation-selected" : ""}`}
                  name="donation3"
                  onClick={selectDonationAmount}
                  value={formData.donation3}
                  readOnly
                />
                <input
                  type="number"
                  placeholder="_"                  
                  className={`donation-page--amount ${formData.selected === "donation4" ? "donation-selected" : ""}`}
                  name="donation4"
                  onClick={selectDonationAmount}
                  onChange={selectDonationAmount}
                  value={formData.donation4==0 ? "" : formData.donation4}
                  step="0.1"
                  min="0"
                />
              </div>              
              <button
                disabled={!formData.selected} 
                className="donate-button"
              >
                Donate
              </button>
            </form>
            <h3 className="donation-page--or">Or</h3>
            <button
              disabled={isRegisteredForLottery}
              className="lottery-button"
              onClick={joinTheLottery}
            >
              {isRegisteredForLottery ? "Already joined the lottery" : "Join the lottery"}
            </button>
            <Modal show={showJoinedLotteryAlert} onHide={() => setShowJoinedLotteryAlert(false)}>
              <Modal.Header closeButton>
                <Modal.Title>Registered for the lottery</Modal.Title>
              </Modal.Header>
              <Modal.Body>Your account has been successfully registered for the lottery!</Modal.Body>
              <Modal.Footer>
              </Modal.Footer>
            </Modal>
          </section>
          :
          <SignInPrompt handleClick={() => wallet.signIn()}/>
        }    
      </div>        
    </main>    
  )
}