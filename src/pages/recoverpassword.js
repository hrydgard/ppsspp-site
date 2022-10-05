import React from 'react';
import clsx from 'clsx';
import { useState, useEffect } from 'react';
import { useHistory, useLocation } from 'react-router-dom'
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';

import { useUserData, defaultUserContext } from '@site/src/theme/Root';

// data has two fields only, the rest comes from the server cookie.
// oldPassword and newPassword
async function recoverPassword(data) {
  return fetch('/api/recoverpassword', {
    method: 'POST',
    header: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(data)
  }).then(data => {
    if (data.status == 200) {
      console.log("success");
      return true
    } else {
      return false;
    }
  })
}

function RecoverPasswordForm() {
  const [email, setEmail] = useState();
  const [recoverSucceeded, setRecoverSucceeded] = useState();
  const [recoverFailed, setRecoverFailed] = useState();

  const handleSubmit = async e => {
    e.preventDefault();

    if (!email || email === "") {
      // don't do anything
      return;
    }
    console.log("email: " + email)
    var result = await recoverPassword({ email: email });
    if (result) {
      setRecoverSucceeded(true);
    } else {
      setRecoverFailed(true);
    }
  }

  return (<>
    {recoverFailed ? <div className="alert alert--warning" role="alert">Failed to send email, try again.</div> : <></>}
    {recoverSucceeded ? <div className="alert alert--success" role="alert">E-mail sent, if registered!</div> : <></>}
    <form onSubmit={handleSubmit}>
      { recoverSucceeded ||
      <>
        <label>
          <div>E-mail address</div>
          <input type="text" onChange={e => setEmail(e.target.value)} />
        </label>
        <div>
        <button className="button button--primary margin-top--md" type="submit">Request recovery link</button>
      </div>
      </>
      }
    </form>
  </>);
}

export default function Home() {
  const {siteConfig} = useDocusaurusContext();
  const {userData, setUserData} = useUserData();

  if (userData.loggedIn) {
    return (
      <Layout title={`Recover password`} description="Recover password">
        <p>Already logged in - no need to recover your password.</p>
      </Layout>
    )
  }

  return (
    <Layout title={`Recover password`} description="Recover password">
    <br/>
      <div className="container">
        <div className={clsx("row simple-center")}>
          <div className={clsx("col col--6")}>
            <div className="card">
              <div className="card__header">
                <h1 style={{paddingTop: "10px"}}>Recover your password</h1>
              </div>
              <div className="card__body">
                <RecoverPasswordForm />
              </div>
            </div>
          </div>
        </div>
      </div>
    </Layout>
  );
}
