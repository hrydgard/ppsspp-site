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
      Since you already have PPSSPP Gold for Windows, you are eligible to request one Google Play promo code, which
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

function RequestGoldForm() {
  const [succeeded, setSucceeded] = useState();
  const [failed, setFailed] = useState();
  const [email, setEmail] = useState();
  const [playEmail, setPlayEmail] = useState();
  const [name, setName] = useState();

  const handleSubmit = async e => {
    e.preventDefault();
    if ((!email && !playEmail) || !name) {
      setFailed(true);
      setSucceeded(false);
      // TODO: Explain error.
      return;
    }
    if (!playEmail) {
      setPlayEmail(email);
    }
    if (!email) {
      setEmail(playEmail);
    }
    var credentials = { email: email, name: name, playEmail: playEmail };
    var result = await jsonPost("makegoldrequest", credentials);
    if (result) {
      setSucceeded(true);
      setFailed(false);
    } else {
      setSucceeded(false);
      setFailed(true);
    }
  };

  return (<>
    {failed ? <div className="alert alert--warning" role="alert">Failed to request link, try again.</div> : <></>}

    <div className="alert alert--info" role="alert">IMPORTANT! Only make a request if you are going to use it!</div>
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
        Request PPSSPP Gold for Windows
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
            <h3>Do you have PPSSPP Gold for Windows, but want it for Android?</h3>
            <p>If so, just <Link to="/login?Forward=requestgold">login here!</Link></p>
            <h3>Do you have PPSSPP Gold for Android, but want it for Windows?</h3>
            <RequestGoldForm/>
          </div>
        </div>
      </div>
      </Layout>
    )
  }

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
