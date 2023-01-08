export default function SignOutButton({accountId, handleClick}) {
  return (
    <button className="signout-button" onClick={handleClick}>
      Sign out {accountId} 
    </button>
  )
}