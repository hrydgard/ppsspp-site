import React from 'react';
import clsx from 'clsx';
import { useState, useEffect } from 'react';
import { useHistory, useLocation } from 'react-router-dom'
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';

import { useUserData, defaultUserContext } from '@site/src/theme/Root';

// data has two fields only, the rest comes from the server cookie.
// oldPassword and newPassword
async function jsonRequest(apiname, jsonData) {
  return fetch('/api/' + apiname, {
    method: 'POST',
    header: {
      'Content-Type': 'application/json'
    },
    body: jsonData ? JSON.stringify(jsonData) : null
  }).then(returnedData => {
    if (returnedData.status == 200) {
      return returnedData.json();
    } else {
      return null;
    }
  })
}

function RequestGooglePlayForm({userData}) {
  const [promoCode, setPromoCode] = useState();
  const [succeeded, setSucceeded] = useState();
  const [failed, setFailed] = useState();

  const handleSubmit = async e => {
    e.preventDefault();
    var result = await jsonRequest("getgoogleplaycode", null);
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
      can be redeemed PPSSPP Gold for Android free on any Google Play account.
    </p>
    <div className="alert alert--warning" role="alert">IMPORTANT! Only request one if you are going to use it!</div>
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
    <p><a href="https://support.google.com/googleplay/answer/3422659?hl=en">How to redeem Google Play promo codes</a></p>
  </>);
}

export default function Home() {
  const {siteConfig} = useDocusaurusContext();
  const {userData, setUserData} = useUserData();

  if (!userData.goldUser) {
    return (
      <Layout title={`Recover password`} description="Recover password">
      <div className="container">
        <div className={clsx("row simple-center")}>
          <div className={clsx("col col--8")}>
            <h1>PPSSPP Gold - Cross License</h1>
            <h3>You have PPSSPP Gold for Android, but want it for Windows?</h3>
            <p>In this case, e-mail me on <a href="hrydgard+ppssppgold@gmail.com">hrydgard+ppssppgold@gmail.com</a>!</p>
            <h3>You have PPSSPP Gold for Windows, but want it for Android?</h3>
            <p>If so, just <a href="/login?Forward=requestgold">login here!</a></p>
          </div>
        </div>
      </div>
      </Layout>
    )
  }

  return (
    <Layout title={`Recover password`} description="Recover password">
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
