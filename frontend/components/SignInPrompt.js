export default function SignInPrompt({handleClick}) {
  return (
    <section className="signin-page">
      <span className="signin-page--title">Help your decentralized neighbor!</span>
      <h3 className="signin-page--subtitle">A simple dapp for helping an anonymous neighbor with an anonymous donation</h3>
      <button
        className="signin-page--button"
        onClick={handleClick}
      >
        Sign in with NEAR Wallet
      </button>
    </section>        
    )    
}