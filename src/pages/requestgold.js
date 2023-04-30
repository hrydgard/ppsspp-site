import React from 'react';
import clsx from 'clsx';
import { useState, useEffect } from 'react';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import Link from '@docusaurus/Link';

import { useUserData } from '@site/src/theme/Root';
import { jsonFetch, jsonPost } from '../util/json_fetch';

function RequestGooglePlayForm({userData}) {
  const [promoCode, setPromoCode] = useState();
  const [succeeded, setSucceeded] = useState();
  const [failed, setFailed] = useState();

  const handleSubmit = async e => {
    e.preventDefault();
    var result = await jsonFetch("getgoogleplaycode", null);
    if (result) {
      setPromoCode(result.code)
      setSucceeded(true);
      setFailed(false);
    } else {
      setFailed(true);
    }
  }
  return (<>
    {failed ? <div className="alert alert--warning" role="alert">Failed to request code, try again.</div> : <></>}

    <h1>Request Google Play promo code</h1>
    <p>
      Since you already have PPSSPP Gold for Windows and macOS, you are eligible to request one Google Play promo code, which
      can be redeemed for PPSSPP Gold for Android free on any Google Play account.
    </p>
    <div className="alert alert--info" role="alert">IMPORTANT! Only request one if you are going to use it!</div>
    <br/>
    <p>Note: If you have requested a code before, you'll get the same one again, which might thus already be spent.</p>
    { succeeded &&
    <>
      <p>Your Google Play promo code:</p>
      {promoCode !== "" ? <pre className="alert alert--success">{promoCode}</pre> : "(ran out of codes, contact hrydgard+ppssppgold@gmail.com)"}
    </> }
    { succeeded ||
    <form onSubmit={handleSubmit}>
      <button className="button button--primary margin-top--md" type="submit">Request Google Play promo code</button>
    </form>
    }
    <br/>
    <p><Link to="https://support.google.com/googleplay/answer/3422659?hl=en">How to redeem Google Play promo codes</Link></p>
  </>);
}

function ValidateEmail(mail) {
  return (/^\w+([\.-]?\w+)*@\w+([\.-]?\w+)*(\.\w{2,3})+$/.test(mail));
}

function RequestGoldForm() {
  const [errorMessage, setErrorMessage] = useState("");
  const [succeeded, setSucceeded] = useState();
  const [failed, setFailed] = useState();
  const [email, setEmail] = useState();
  const [playEmail, setPlayEmail] = useState();
  const [name, setName] = useState();

  const handleSubmit = async e => {
    e.preventDefault();

    var submitEmail = email;
    var submitPlayEmail = playEmail;

    if ((!submitEmail && !submitPlayEmail) || !name) {
      setErrorMessage("Missing information");
      setFailed(true);
      setSucceeded(false);
      // TODO: Explain error.
      return;
    }
    if (!submitPlayEmail) {
      submitPlayEmail = submitEmail;
    }
    if (!submitEmail) {
      submitEmail = submitPlayEmail;
    }
    if (!ValidateEmail(submitPlayEmail)) {
      setErrorMessage("Invalid email address " + submitPlayEmail)
      setFailed(true);
      setSucceeded(false);
      return;
    }
    if (!ValidateEmail(submitEmail)) {
      setErrorMessage("Invalid email address " + submitEmail)
      setFailed(true);
      setSucceeded(false);
      return;
    }

    var credentials = { email: submitEmail, name: name, playEmail: submitPlayEmail };
    console.log(credentials);
    var result = await jsonPost("makegoldrequest", credentials);
    if (result) {
      setSucceeded(true);
      setFailed(false);
    } else {
      setErrorMessage("Failed to make request - duplicate?");
      setSucceeded(false);
      setFailed(true);
    }
  };

  return (<>
    <div className="alert alert--info" role="alert">IMPORTANT! <strong>You MUST already own PPSSPP Gold on your Play account</strong> to use this form.<br/>Also, only request if you actually plan to use the PC version!<br/>Your request may take up to 72 hours to process.</div>
    {failed ? <div className="alert alert--warning" role="alert">Failed to request link: {errorMessage}</div> : <></>}
    <br/>
    <form onSubmit={handleSubmit}>
      <label>
        <div>Name</div>
        <span><input type="text" size="38" onChange={e => setName(e.target.value)} /></span>
      </label>
      <label>
        <div>Google Play account</div>
        <span><input type="text" size="38" onChange={e => setPlayEmail(e.target.value)} /></span>
      </label>
      <label>
        <div>E-mail address (if different from Google Play account)</div>
        <span><input type="text" size="38" onChange={e => setEmail(e.target.value)} /></span>
      </label>
      <button className="button button--primary margin-bottom--md margin-top--md" type="submit">
        Request PPSSPP Gold for Windows and macOS
      </button>
    </form>

    <br/>
    {succeeded ? <div className="alert alert--success" role="alert">Request sent!</div> : <></>}
  </>);
}

export default function Home() {
  const {siteConfig} = useDocusaurusContext();
  const {userData, setUserData} = useUserData();

  if (!userData.goldUser) {
    return (
      <Layout title={`Cross licensing`} description="Cross licensing">
      <div className="container">
        <div className={clsx("row simple-center")}>
          <div className={clsx("col col--8")}>
            <h1>PPSSPP Gold - Cross License</h1>
            <h3>Do you have PPSSPP Gold for Windows/macOS, but want it for Android?</h3>
            <p>If so, just <Link to="/login?Forward=requestgold">login here</Link> and follow the instructions!</p>
            <h3>Do you have PPSSPP Gold for Android, but want it for Windows?</h3>
            <p>Then please send an e-mail to hrydgard+ppssppgold@gmail.com , attaching your Google Play receipt. Your request will be handled within 72 hours.</p>
            <p>(There used to be a convenient form here, but it got completely overloaded with 95% bogus requests).</p>
          </div>
        </div>
      </div>
      </Layout>
    )
  }

  // <RequestGoldForm/>

  return (
    <Layout title={`Request Google Play code`} description="Request Google Play code">
    <br/>
      <div className="container">
        <div className={clsx("row simple-center")}>
          <div className={clsx("col col--6")}>
            <RequestGooglePlayForm userData={userData}/>
          </div>
        </div>
      </div>
    </Layout>
  );
}
