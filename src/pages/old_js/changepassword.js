import React from 'react';
import clsx from 'clsx';

import { useState } from 'react';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';

import { useUserData, defaultUserContext } from '@site/src/theme/Root';
import { jsonFetch, jsonPost } from '../util/json_fetch';

function ChangePasswordForm() {
  const {userData, setUserData} = useUserData();

  const [oldPassword, setOldPassword] = useState();
  const [newPassword, setNewPassword] = useState();

  const [changeFailed, setChangeFailed] = useState();

  const handleSubmit = async e => {
    e.preventDefault();
    var result = await jsonPost("changepassword", { oldPassword: oldPassword, newPassword: newPassword }, setUserData);
    if (result) {
      window.location.href = "/login";
    } else {
      setChangeFailed(true);
    }
  }

  var showOldPasswordField = userData.loginType != "key";
  return (<>
    {changeFailed ? <div className="alert alert--warning" role="alert">Failed to change password, try again.</div> : <></>}
    <form onSubmit={handleSubmit}>
      {showOldPasswordField ?
        <label>
          <div>Previous password</div>
          <input type="password" onChange={e => setOldPassword(e.target.value)} />
        </label> : <></>
      }
      <label>
        <div>New password</div>
        <input type="password" onChange={e => setNewPassword(e.target.value)} />
      </label>
      <div>
        <button className="button button--primary margin-top--md" type="submit">Change password</button>
      </div>
    </form>
  </>);
}

export default function Home() {
  const {siteConfig} = useDocusaurusContext();
  const {userData, setUserData} = useUserData();

  if (!userData.loggedIn) {
    return (
      <Layout title="Change password" description="Change password">
        <p>Not logged in - can't change password.</p>
        <p><Link to="/login">Login</Link></p>
      </Layout>
    )
  }

  return (
    <Layout title="Change password" description="Change password">
    <br/>
      <div className="container">
        <div className={clsx("row simple-center")}>
          <div className={clsx("col col--6")}>
            <div className="card">
              <div className="card__header">
                <h1 style={{paddingTop: "10px"}}>Change your password</h1>
              </div>
              <div className="card__body">
                <ChangePasswordForm/>
              </div>
            </div>
          </div>
        </div>
      </div>
    </Layout>
  );
}
