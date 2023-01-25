import React from 'react';
import clsx from 'clsx';
import { useState, useEffect } from 'react';
import { useHistory, useLocation } from 'react-router-dom'
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import Link from '@docusaurus/Link';

import { useUserData, defaultUserContext } from '@site/src/theme/Root';

// API call wrappers, separated from UI concerns
// Generally return a false-y value on failure and true-y values (or useful json) on success.

async function loginUser(credentials) {
  return fetch('/api/login', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(credentials)
  }).then(data => {
    if (data.status == 200) {
      return data.json();
    } else {
      return null;
    }
  });
}

async function logoutUser(userData, setUserData) {
  // Reset user data immediately
  setUserData(defaultUserContext);
  // Reset the local cookie so we don't auto-login on the next load.
  localStorage.removeItem('ppsspp-auth');
  // Reset the server cookie
  return fetch('/api/logout', {
    method: 'POST',
    header: {
      'Content-Type': 'application/json'
    },
  }).then(data => {
    console.log("Logged out.");
  });
}

async function giveFreeGold(goldUserData) {
  return fetch('/api/freegold', {
    method: 'POST',
    header: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(goldUserData)
  }).then(data => {
    if (data.status == 200) {
      return data.json();
    } else {
      return null;
    }
  });
}

// email field is enough for this one.
async function getMagicLink(goldUserData) {
  return fetch('/api/getmagiclink', {
    method: 'POST',
    header: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(goldUserData)
  }).then(data => {
    if (data.status == 200) {
      return data.json();
    } else {
      return null;
    }
  })
}

function LoginForm({ setLoginData, forward }) {
    const [email, setEmail] = useState();
    const [password, setPassword] = useState();
    const [loginFailed, setLoginFailed] = useState();
    const [errorMessage, setErrorMessage] = useState("");

    const handleSubmit = async e => {
      e.preventDefault();  // Prevents an automatic refresh from the submit.

      if (!email || !password) {
        setLoginFailed(true);
        setErrorMessage("You must provide both email and password.");
        return;
      }
      if (!email.includes("@") || !email.includes(".")) {
        setLoginFailed(true);
        setErrorMessage("An email address must have a @ and a dot.");
        return;
      }

      var credentials = {
        'email': email,
        'password': password,
        'key': "",
      };
      const loginData = await loginUser(credentials);
      if (loginData) {
        console.log("Loginform submitted");
        console.log(loginData);
        setLoginData(loginData);
        setLoginFailed(false);
        if (forward) {
          console.log("Forwarding to " + forward);
          window.location.href = "/" + forward;
        } else {
          console.log("No forward given, not navigating");
        }
      } else {
        console.log("No login data");
        setLoginFailed(true);
        setErrorMessage("Email/Password didn't match.");
      }
    }

    return (
      <>
        <form onSubmit={handleSubmit}>
          {loginFailed ? <div className="alert alert--warning" role="alert">{errorMessage}</div> : <></>}
          <label>
            <div>E-mail address</div>
            <span><input type="text" size="38" onChange={e => setEmail(e.target.value)} /></span>
          </label>
          <label>
            <div>Password</div>
            <span><input type="password" size="38" onChange={e => setPassword(e.target.value)} /></span>
          </label>
          <div><Link to="/recoverpassword">Forgot your password?</Link></div>
          <div>
            <button className="button button--primary margin-top--md" type="submit">Login</button>
          </div>
        </form>
      </>
    );
}

function GiveFreeGoldForm() {
  const [email, setEmail] = useState();
  const [name, setName] = useState();
  const [successAlert, setSuccessAlert] = useState();
  const [failedAlert, setFailedAlert] = useState();
  const [magicLink, setMagicLink] = useState("");

  const handleSubmit = async e => {
    e.preventDefault();

    if (!name || !email) {
      setFailedAlert(true);
      setSuccessAlert(false);
      return;
    }

    var freeGoldUser = {
      'name': name,
      'email': email,
    };
    const response = await giveFreeGold(freeGoldUser);
    if (response) {
      console.log("success");
      setSuccessAlert(true);
      setFailedAlert(false);
      setMagicLink(response.magicLink)
    } else {
      console.log("failure");
      setSuccessAlert(false);
      setFailedAlert(true);
    }
  }

  return (<>
    <h3>Give Free Gold</h3>
    <form onSubmit={handleSubmit}>
      {failedAlert ? <div className="alert alert--warning" role="alert">Failed giving free gold.</div> : <></>}
      {successAlert ? <div className="alert alert--success" role="alert">Free gold given to {name}!</div> : <></>}
      <label>
        <div>Name</div>
        <input type="text" onChange={e => setName(e.target.value)} />
      </label>
      <label>
        <div>Email address</div>
        <input type="text" onChange={e => setEmail(e.target.value)} />
      </label>
      <div>
        <button className="button button--primary margin-top--md" type="submit">Give free Gold!</button>
      </div>
    </form>
    <p>{magicLink}</p>
  </>);
}

function GetMagicLinkForm() {
  const [email, setEmail] = useState();
  const [magicLink, setMagicLink] = useState();
  const [userNotFound, setUserNotFound] = useState();

  const handleSubmit = async e => {
    e.preventDefault();

    if (!email) {
      setUserNotFound(true);
      return;
    }

    var existingGoldUser = {
      'email': email,
    };
    var magicLink = await getMagicLink(existingGoldUser);
    if (magicLink) {
      console.log("success: " + magicLink);
      setMagicLink(magicLink.link);
      setUserNotFound(false);
    } else {
      console.log("failure");
      setMagicLink(null);
      setUserNotFound(true);
    }
  }

  return (<>
    <h3>Get Magic Link</h3>
    <form onSubmit={handleSubmit}>
      {userNotFound ? <div className="alert alert--warning" role="alert">User not found.</div> : <></>}
      <label>
        <div>Email address</div>
        <input type="text" onChange={e => setEmail(e.target.value)} />
      </label>
      <div>
        <button className="button button--primary margin-top--md" type="submit">Get magic link</button>
      </div>
      {magicLink ? <div>{magicLink}</div> : <></>}
    </form>
  </>);
}

function AdminTools(userData) {
  if (!userData.admin) {
    return (<></>);
  }
  return (
    <>
      <div className="card">
        <div className="card__header">
          <h3>Admin Tools</h3>
          <p>Login type: {userData.loginType}</p>
          <GiveFreeGoldForm/>
          <br/>
          <GetMagicLinkForm/>
        </div>
      </div>
    </>
  )
}

function translateForward(forward, reason) {
  // Fix up some old forward values
  switch (forward) {
  case "login":
    // Prevent malicious forwarding loops.
    forward = null;
    break;
  case "downloadgold":
    forward = "download";
    break;
  }
  if (forward) {
    console.log("query forward after translation: " + forward + " reason = " + reason);
  }
  return forward;
}

export default function Home() {
  const {siteConfig} = useDocusaurusContext();
  const {userData, setUserData} = useUserData();

  function setLoginData(loginData) {
    var userData = {
      'loggedIn': true,
      'goldUser': loginData.goldUser,
      'admin': loginData.admin,
      'name': loginData.name,
      'email': loginData.email,
      'loginType': loginData.loginType,
    };

    console.log("Setting userdata: ");
    console.log(userData);

    setUserData(userData);
    // Also set a non-server localstorage "cookie" that we can restore in Root, for visual rendering
    // purposes (does not have anything to do with authentication, but we clear it when logging out).
    localStorage.setItem('ppsspp-auth', JSON.stringify(userData));
  }

  console.log("Userdata at load of login.js:");
  console.log(userData);

  // Get query parameters, in the format of the old gold URLs.
  const location = useLocation();
  const history = useHistory();
  useEffect(() => {
    // Handle login-by-key-in-query-parameters (magic links).
    async function l() {
      const queryParams  = new URLSearchParams(location.search);
      const queryEmail = queryParams.get('Email');
      const queryKey = queryParams.get('Key');
      var forward = null;

      // Note: We allow logins by key if already logged in by password.
      // This is so that password recovery emails can still be opened if you happened to forget
      // the password while you were already logged in...
      if (queryEmail && queryKey && (!userData.loggedIn || userData.loginType == "password")) {
        // Automated login by key
        var credentials = {
          'email': queryEmail,
          'key': queryKey,
        };
        console.log("Trying to log in by key: " + credentials.email + " " + credentials.key);
        const loginData = await loginUser(credentials);
        if (loginData) {
          setLoginData(loginData);
          // setLoginFailed(false);
          forward = translateForward(queryParams.get('Forward'), "effect");
        } else {
          console.log("No login data in response, not logging in. forward was " + forward);
          // setLoginFailed(true);
          // Remove query parameters. This causes some kind of wacky redirect loop though!
          // queryParams.delete('error')
        }
      } else if (userData.loggedIn) {
        forward = translateForward(queryParams.get('Forward'), "effect");
      }
      if (forward) {
        var url = "/" + forward;
        console.log("Login success. Attempting to navigate to forwarding URL " + url);
        window.location.href = url;
      } else {
        console.log("not forwarding, forward == null");
      }
    }
    l();
  });

  if (userData.loggedIn) {
    // Show logged-in dashboard, with a log out button and so on.
    return (
      <Layout title={`Logged in`} description="Logged in">
        <br/>
        <div className="centering-container">
          <div className="card">
            <div className="card__header">
              <h3>{userData.name}</h3>
            </div>
            <div className="card__body">
              <p>Logged in: {userData.email}. {userData.goldUser ? "Has PPSSPP Gold." : ""}</p>
              <p><Link to="/changepassword">Change password</Link></p>
              <p><Link to="/requestgold">Request Gold for Android</Link></p>
              <p><Link to="/download">Downloads</Link></p>
              <p>
                <button className="button button--primary margin-top--md" type="submit" onClick={() => logoutUser(userData, setUserData)}>Log out</button>
              </p>
            </div>
          </div>
          <br/>
          {AdminTools(userData)}
        </div>
      </Layout>
    )
  }

  const queryParams = new URLSearchParams(location.search);
  const forward = translateForward(queryParams.get('Forward'), "set form forward");

  return (
    <Layout title={`${siteConfig.title} Login`} description="Login">
      <br/>
      <div className="container">
        <div className={clsx("row simple-center")}>
          <div className={clsx("col col--6")}>
            <div className="card">
              <div className="card__header">
                <h1 style={{paddingTop: "10px"}}>Login</h1>
              </div>
              <div className="card__body">
                <LoginForm setLoginData={setLoginData} forward={forward} />
                <br/>
                <h3>Important!</h3>
                <p>Login above to access PPSSPP Gold for Windows, if you have already bought it. If not, you can <Link to="/buygold">buy it here</Link>.</p>
                <p>If you already have PPSSPP Gold for Android, and want an account to download it for Windows, <Link to="/requestgold">click here</Link> for information.</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </Layout>
  );
}
