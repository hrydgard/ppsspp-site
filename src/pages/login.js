import React from 'react';
import clsx from 'clsx';
import { useState, useEffect } from 'react';
import { useHistory, useLocation } from 'react-router-dom'
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import Link from '@docusaurus/Link';

import { useUserData, defaultUserContext } from '@site/src/theme/Root';

import { jsonFetch, jsonPost } from '../util/json_fetch';

// API call wrappers, separated from UI concerns
// Generally return a false-y value on failure and true-y values (or useful json) on success.

async function logoutUser(setUserData) {
  // Reset user data immediately
  setUserData(defaultUserContext);
  // Reset the local cookie so we don't auto-login on the next load.
  localStorage.removeItem('ppsspp-auth');
  // Reset the server cookie
  jsonPost("logout");
}

function LoginForm({ setLoginData, forward }) {
    const [email, setEmail] = useState();
    const [password, setPassword] = useState();
    const [loginFailed, setLoginFailed] = useState();
    const [errorMessage, setErrorMessage] = useState("");

    const handleSubmit = async e => {
      e.preventDefault();  // Prevents an automatic refresh from the submit.

      var userEmail = email.trim();

      if (!userEmail || !password) {
        setLoginFailed(true);
        setErrorMessage("You must provide both email and password.");
        return;
      }
      if (!userEmail.includes("@") || !userEmail.includes(".") || userEmail.includes(" ")) {
        setLoginFailed(true);
        setErrorMessage("An email address must have a @ and a dot, and can't have spaces.");
        return;
      }

      var credentials = {
        'email': userEmail,
        'password': password,
        'key': "",
      };
      console.log(credentials);
      const loginData = await jsonFetch("login", credentials);
      console.log(loginData);
      if (loginData) {
        console.log(loginData);
        setLoginData(loginData);
        setLoginFailed(false);
        if (forward) {
          console.log("Forwarding to " + forward);
          window.location.href = "/" + forward;
        }
      } else {
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

    var userName = name;
    var userEmail = email;

    if (!userName) {
      setFailedAlert(true);
      setSuccessAlert(false);
      return;
    }

    if (userEmail) {
      userEmail = userEmail.trim();
    }
    if (userName) {
      userName = userName.trim();
    }

    if (userName && !userEmail) {
      // See if we can auto-reformat from this format: Mr User <mruser@gmail.com>
      if (userName.includes(" <")) {
        console.log("splitting " + userName);
        var offset = userName.indexOf(" <");
        if (offset != -1) {
          userEmail = userName.substr(offset + 2, userName.length - offset - 2 - 1);
          userName = userName.substr(0, offset);
        } else {
          console.log("not splitting, separator not found");
        }
      }

      // For now, return.
      console.log(userName + " '" + userEmail + "'");
    }

    if (!userEmail) {
      return;
    }

    var freeGoldUser = {
      'name': userName,
      'email': userEmail,
    };
    const response = await jsonFetch("freegold", freeGoldUser);
    if (response) {
      setSuccessAlert(true);
      setFailedAlert(false);
      setMagicLink(response.magicLink)
    } else {
      setSuccessAlert(false);
      setFailedAlert(true);
    }
  }

  return (<>
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
    <p>{magicLink &&
      <div contentEditable="true" name="freeGoldText">
        Certainly!
        <br/>
        <br/>
        Here's your link to log in automatically, which will let you access PPSSPP Gold downloads for Windows:
        <br/>
        <br/>
        {magicLink}
        <br/>
        <br/>
        Enjoy,
        <br/>
        Henrik
      </div> }
    </p>
  </>);
}

function GetMagicLinkForm() {
  const [email, setEmail] = useState();
  const [magicLink, setMagicLink] = useState();
  const [userNotFound, setUserNotFound] = useState();

  const handleSubmit = async e => {
    e.preventDefault();
    var userEmail = email.trim();

    if (!userEmail) {
      setUserNotFound(true);
      return;
    }

    var existingGoldUser = {
      'email': userEmail,
    };
    var magicLink = await jsonFetch("getmagiclink", existingGoldUser);
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

function GooglePlayCodeForm() {
  const [codesUpToDate, setCodesUpToDate] = useState(false);
  const [codesUsed, setCodesUsed] = useState();
  const [codesLeft, setCodesLeft] = useState();
  const [pendingCodes, setPendingCodes] = useState();

  useEffect(async () => {
    if (codesUpToDate) {
      return;
    }
    var data = await jsonFetch("googleplaycodeadmin", {});
    if (data) {
      setCodesUsed(data.codesUsed);
      setCodesLeft(data.codesLeft);
    } else {
      console.log("got no data");
    }
    setCodesUpToDate(true);
  });

  const handleSubmit = async e => {
    e.preventDefault();
    var codeString = pendingCodes.trim();

    if (!codeString) {
      return;
    }

    // TODO: Validate contents better.
    var codes = codeString.split(/\r?\n/).filter(x => x.length > 0).map(x => x.trim());
    console.log(codes);
    if (codes.length == 0) {
      console.log("nothing to do");
      return;
    }

    var request = {
      'codes': codes,
    };
    var result = await jsonPost("googleplaycodeadmin", request);
    if (result) {
      console.log("success: " + magicLink);
    } else {
      console.log("failure");
    }
  }

  return <>
    {codesLeft > 0 && codesLeft < 10 && <div className="alert alert--warning" role="alert">Nearly out of codes!</div>}
    {codesLeft == 0 && <div className="alert alert--error" role="alert">Out of codes!</div>}
    <p>Codes used: {codesUsed}</p>
    <p>Codes left: {codesLeft}</p>

    <form onSubmit={handleSubmit}>
      <label>
        <div>Codes to add:</div>
        <textarea rows="20" cols="24" onChange={e => setPendingCodes(e.target.value)} />
      </label>
      <div>
        <button className="button button--primary margin-top--md" type="submit">Add promo codes</button>
      </div>
    </form>
  </>;
}

function AdminCard({title, contents}) {
  return (
    <div className="card margin-bottom--md">
      <div className="card__header">
        <h3>Admin: {title}</h3>
      </div>
      <div className="card__body">
        {contents()}
      </div>
    </div>);
}

function AdminTools(userData) {
  if (!userData.admin) {
    return (<></>);
  }
  return (
    <>
      <div className={clsx("col col--3")}>
        <AdminCard title="Free Gold" contents={GiveFreeGoldForm}/>
      </div>
      <div className={clsx("col col--3")}>
        <AdminCard title="Get Magic Link" contents={GetMagicLinkForm}/>
      </div>
      <div className={clsx("col col--3")}>
        <AdminCard title="Google Play Codes" contents={GooglePlayCodeForm}/>
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
        const loginData = await jsonFetch("login", credentials);
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
        <div className="container">
          <div className="row centering-row">
            <div className="col col--3">
              <div className="card">
                <div className="card__header">
                  <h3>{userData.name}</h3>
                </div>
                <div className="card__body">
                  <p><strong>{userData.email}</strong></p>
                  {userData.goldUser && <p>Has PPSSPP Gold.</p>} {userData.admin && <p>Has admin rights.</p>}
                  <p><Link to="/changepassword">Change password</Link></p>
                  <p><Link to="/requestgold">Request Gold for Android</Link></p>
                  <p><Link to="/download">Downloads</Link></p>
                  <p>
                    <button className="button button--primary margin-top--md" type="submit" onClick={() => logoutUser(setUserData)}>Log out</button>
                  </p>
                </div>
              </div>
            </div>
            {AdminTools(userData)}
          </div>
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
        <div className={clsx("row centering-row")}>
          <div className="col col--4">
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
