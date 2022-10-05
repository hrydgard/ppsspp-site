import React from 'react';
import clsx from 'clsx';
import { useState, useEffect, useRef } from 'react';
import Layout from '@theme/Layout';

import { useUserData, defaultUserContext } from '@site/src/theme/Root';

var ORDER_REFRESH_INTERVAL = 1000;

// data has two fields only, the rest comes from the server cookie.
// oldPassword and newPassword
async function getOrderStatus(data) {
  return fetch('/api/getorderstatus', {
    method: 'POST',
    header: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(data)
  }).then(data => {
    if (data.status == 200) {
      return data.json();
    } else {
      return null;
    }
  })
}


// Custom hook from https://overreacted.io/making-setinterval-declarative-with-react-hooks/

function useInterval(callback, delay) {
  const savedCallback = useRef();

  // Remember the latest callback.
  useEffect(() => {
    savedCallback.current = callback;
  }, [callback]);

  // Set up the interval.
  useEffect(() => {
    function tick() {
      savedCallback.current();
    }
    if (delay !== null) {
      let id = setInterval(tick, delay);
      return () => clearInterval(id);
    }
  }, [delay]);
}

function copyMagicLink() {
  var copyText = document.getElementById("magicLink");
  copyText.select();
  copyText.setSelectionRange(0, 99999); // For mobile devices
  navigator.clipboard.writeText(copyText.value);
}

export default function Home() {
  const {userData, setUserData} = useUserData();
  const [orderStatus, setOrderStatus] = useState();
  const [attempts, setAttempts] = useState();

  const queryParams  = new URLSearchParams(location.search);
  const reference = queryParams.get('orderId');

  useInterval(async () => {
    setAttempts(attempts + 1);
    if (orderStatus || attempts > 50) {
      return;
    }
    var status = await getOrderStatus({reference: reference});
    console.log(status);
    setOrderStatus(status);
  }, ORDER_REFRESH_INTERVAL);

  var progress = <p>Checking for confirmation...</p>;
  var explanation = (<>
  <p>
    NOTE: If no e-mail arrives within a few minutes, please check your spam box.
    If it's not there or you want to use a different e-mail address for log-in,
    then <a href="mailto:hrydgard+ppssppgold@gmail.com">e-mail me</a> and I'll sort it out.
  </p>
  <p>
    Also, if you want access to the Android version of PPSSPP Gold, likewise, use the contact address above.
  </p>
  <p>Your order reference: <strong>{reference}</strong>.</p>
  </>);

  var goldAlready = <>
    <p>You already have PPSSPP Gold and are logged in, so you're all set!</p>
    <a href="/download">Go to downloads</a>
  </>

  var report = <></>;

  if (orderStatus) {
    progress = <></>;
    if (orderStatus.paymentStatus == "completed") {
      report = <>
      <div className="alert alert--success" role="alert">E-mail with login link sent to <strong>{orderStatus.email}</strong>!</div>
      <br/>
      <p>Your login link:<br/>
      <input type="text" readOnly value={orderStatus.magicLink} id="magicLink"/>&nbsp;<button onClick={() => copyMagicLink()}>Copy</button></p>
      <a href={orderStatus.magicLink}>Login and download now!</a><br/>
      </>;
    } else {
      // This shouldn't really happen, we don't log failed purchases, and we shouldn't have gotten
      // forwarded to this page. So let's just do something simple here.
      report = <p>Your order status is: {orderStatus.paymentStatus}</p>
    }
  }

  // TODO: Automatically poll for the order number every 5 seconds, and once done, show the details directly on the site,
  // along with a login link! This will require an API on the backend to poll as well.
  // See https://blog.bitsrc.io/polling-in-react-using-the-useinterval-custom-hook-e2bcefda4197 .

  return (
    <Layout title={`Thank you!`} description="Thank you!">
    <br/>
      <div className="container">
        <div className={clsx("row simple-center")}>
          <div className={clsx("col col--6")}>
            <div className="card">
              <div className="card__header">
                <h1>Thanks for buying PPSSPP Gold for Windows!</h1>
              </div>
              <div className="card__body">
              <img src="/img/platform/ppsspp-icon-gold.png" width="48px" />

              <p>Thank you for your purchase!</p>

              {progress}

              {report}
              <br/>
              {userData.loggedIn ? goldAlready : explanation}
              </div>
            </div>
          </div>
        </div>
      </div>
    </Layout>
  );
}
