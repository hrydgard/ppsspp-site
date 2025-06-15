// NOTE: This struct should match the LoginResponse used on the server side.
const defaultUserContext = {
    // Permissions. These are also checked on the server each request,
    // so these state bools are only here to easily hide irrelevant UI etc.
    loggedIn: false,
    goldUser: false,
    admin: false,
    loginType: "password",

    name: "NotLoggedIn",
    email: "notloggedin@fake.com",
};

var g_userData = defaultUserContext;

// Alert types for visual display.
const ERROR = "alert-error";
const WARNING = "alert-warning";
const SUCCESS = "alert-success";
const INFO = "alert-info";
const HIDDEN = "alert-hidden";
const ALL_ALERTS = [ERROR, WARNING, SUCCESS, INFO, HIDDEN];

function buyProduct(productId, item) {
    console.log("Buy button clicked for " + productId);
    console.log(item);
    window.fastspring.builder.push({
        products: [
            {
                path: item.path,
                quantity: 1
            }
        ],
        checkout: true
    });
}

function statusCodeAction(status) {
    if (status == 401) {
        // If any API call returns Unauthorized, we'll simply
        // log out the user, since this should only ever happen if
        // (1) The user doesn't have the required status, in which case there might be something suspicious afoot
        // (2) The user login expired
        // Reset user data immediately
        console.log("Some request returned 401, logging out");
        logoutUser();
    }
}

function validateEmailAddress(email) {
    var validRegex = /^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*$/;
    return email.match(validRegex);
}

async function jsonFetch(apiName, requestBody) {
    return fetch('/api/' + apiName, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: requestBody ? JSON.stringify(requestBody) : null,
    }).then(data => {
        if (apiName != "logout") {
            console.log("Calling statusCodeAction for " + apiName + " with status " + data.status);
            statusCodeAction(data.status);
        } else {
            console.log("Logout request failed!");
        }
        if (data.status == 200) {
            return data.json();
        } else {
            console.log("data.status was " + data.status + ", not 200, returning null");
            return null;
        }
    });
}

async function jsonFetchGetCode(apiName, requestBody) {
    return fetch('/api/' + apiName, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: requestBody ? JSON.stringify(requestBody) : null,
    }).then(data => {
        return data.status;
    });
}

// This one doesn't expect a JSON response, just a status.
async function jsonPost(apiName, requestBody) {
    return fetch('/api/' + apiName, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: requestBody ? JSON.stringify(requestBody) : null,
    }).then(data => {
        statusCodeAction(data.status);
        return data.status == 200;
    });
}

function setDisplayMode(className, mode) {
    // console.log("Setting all with " + className + " to " + mode);
    for (let element of document.getElementsByClassName(className)) {
        // console.log(element);
        element.style.display = mode;
    }
}

// It's a little awkward that unlike handlebars, sqrl has these "it." everywhere.
// Also, ifs and each look a bit different.

// TODO: See if we can find a better client-side templating solution. While it's nice to
// avoid react, this does get a little awkward, vscode fails to syntax color this.
const tmplUserInfo = `
<div>
<p>{{it.name}}
{{ @if (it.admin) }}
<strong>&nbsp;(ADMIN)</strong>
{{ /if }}
</p>
<p>Email: {{it.email}}.</p>
{{ @if (it.goldUser) }}
<p>Gold status! <img src="/static/img/platform/ppsspp-icon-gold.png" aria-hidden="true" class="icon-24"></p>
{{ /if }}
<p><a href="/changepassword">Change password</a></p>
</div>
`;

// Not really secret panel, even if you know what this can do you can't do anything :P
const tmplAdminPanel = `
<div class="col-6">

<div class="card">
<div class="card-title">
    <h2 class="no-icon">Give free gold</h2>
</div>
<form action="#" onSubmit="return handleGiveFreeGold(event)">
<div class="alert alert-hidden" id="error_message" role="alert"></div>
<label>
    <div>Name</div>
    <span><input type="text" size="38" id="freegold_name" /></span>
    <div>E-mail address</div>
    <span><input type="text" size="38" id="freegold_email" /></span>
</label>
<div>
    <button class="download-button" type="submit">Give free gold</button>
</div>
<div id="freeGoldStatus"></div>
</form>
</div>

</div>

<div class="col-6">

<div class="card">
<div class="card-title">
    <h2 class="no-icon">Get magic link</h2>
</div>
<form action="#" onSubmit="return handleGetMagicLink(event)">
<div class="alert alert-hidden" id="error_message" role="alert"></div>
<label>
    <div>E-mail address</div>
    <span><input type="text" size="38" id="magiclink_email" /></span>
</label>
<div>
    <button class="download-button" type="submit">Get magic link!</button>
</div>
<div id="magic_link"></div>
<div id="magicLinkStatus" class="alert alert-hidden"></div>
</form>
</div>

</div>

<div class="col-6">
<div class="card">
<div class="card-title">
    <h2 class="no-icon">Google Play Codes</h2>
</div>

<div id="playCodesStats">...</div>
<div id="googlePlayStatus" class="alert alert-hidden"></div>
<form action="#" onSubmit="return handleAddGooglePlayCodes(event)">
<label>
  <div>Codes to add:</div>
  <textarea rows="10" cols="24" id="pending_codes"}></textarea>
</label>
<div>
  <button class="download-button" type="submit">Add promo codes</button>
</div>
</form>
</div>

</div>
</div>
`;

const tmplPlayCodes = `
<p>Codes used: {{it.codesUsed}}</p>
<p>Codes left: {{it.codesLeft}}</p>
`;

const tmplLoginCorner = `
<a href='/login' class="center-vertical">
{{@if(it.loggedIn)}}
<i aria-label="Profile" class="icon-ui icon-ui-user"></i>
{{#else}}
Login
{{/if}}
</a>
`;

const tmplLoginItem = `
<a href='/login'>
{{@if(it.loggedIn)}}
<i aria-hidden="true" class="icon-ui icon-ui-user"></i>{{it.name}}
{{#else}}
Login
{{/if}}
</a>
`;

async function applyDOMVisibility() {
    // Hides and shows stuff depending on your current login state.
    if (g_userData.loggedIn) {
        setDisplayMode("logged-in-only", g_userData.loggedIn ? "block" : "none");
        setDisplayMode("not-logged-in-only", g_userData.loggedIn ? "none" : "block");
    } else {
        setDisplayMode("logged-in-only", "none");
        setDisplayMode("not-logged-in-only", "block");
    }

    setDisplayMode("not-login-key-only", g_userData.loginType != "key" ? "block" : "none");

    setDisplayMode("gold-only", g_userData.goldUser ? "block" : "none");
    setDisplayMode("gold-only-inline", g_userData.goldUser ? "inline" : "none");
    setDisplayMode("no-gold-only", g_userData.goldUser ? "none" : "block");

    // Update various textual fields if present on the page.
    // Top navigation bar
    const navbarIcon = document.getElementById("navbarIcon");
    if (navbarIcon) {
        navbarIcon.src = g_userData.goldUser ? "/static/img/platform/ppsspp-icon-gold.png" : "/static/img/platform/ppsspp-icon.png";
    }
    const loginCorner = document.getElementById("loginCorner");
    if (loginCorner) {
        loginCorner.innerHTML = Sqrl.render(tmplLoginCorner, g_userData);
    }

    // Hamburger menu
    const loginItem = document.getElementById("loginItem");
    if (loginItem) {
        loginItem.innerHTML = Sqrl.render(tmplLoginItem, g_userData);
    }

    // Login page
    const loginInfo = document.getElementById("loginInfo");
    if (loginInfo) {
        loginInfo.innerHTML = Sqrl.render(tmplUserInfo, g_userData);
        const adminPanels = document.getElementById("adminPanels");
        if (g_userData.admin && adminPanels) {
            adminPanels.innerHTML = Sqrl.render(tmplAdminPanel, g_userData);
            let freegold_name_input = document.getElementById("freegold_name");
            let freegold_email_input = document.getElementById("freegold_email");
            let listener = (event) => {
                setStatusDisplay(HIDDEN, "freeGoldStatus");
            };
            freegold_name_input.addEventListener("input", listener);
            freegold_email_input.addEventListener("input", listener);
        } else {
            adminPanels.innerHTML = "";
        }
    }
    const playCodesStats = document.getElementById("playCodesStats");
    if (playCodesStats) {
        await updatePlayCodesStats();
    }
}

async function updatePlayCodesStats() {
    const data = await jsonFetch("googleplaycodeadmin", {});
    if (data) {
        console.log(data);
        playCodesStats.innerHTML = Sqrl.render(tmplPlayCodes, data);
    } else {
        console.log("got no play codes stats");
    }
}

// severities: WARN, ERROR, SUCCESS, INFO
function setStatusDisplay(severity, errorElement, errorString) {
    let errorBox = document.getElementById(errorElement);
    if (errorBox) {
        console.log(severity + ": " + errorString + " (updating alert state next) " + severity);
        errorBox.innerHTML = errorString;
        errorBox.classList.add("alert");
        ALL_ALERTS.forEach(function (a) {
            console.log("removing " + a)
            errorBox.classList.remove(a);
        });
        errorBox.classList.add(severity);
        console.log("done with alert states. Added " + severity);
    } else {
        console.log("Error-display '" + errorElement + "' didn't exist: " + errorString)
    }
}

async function handleGiveFreeGold(event) {
    event.preventDefault();
    let userName = document.getElementById("freegold_name").value.trim();
    let userEmail = document.getElementById("freegold_email").value.trim();

    console.log("Gonna give free gold to " + userEmail + " " + userName);

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
            const offset = userName.indexOf(" <");
            if (offset != -1) {
                userEmail = userName.substr(offset + 2, userName.length - offset - 2 - 1);
                userName = userName.substr(0, offset);
            } else {
                console.log("not splitting, separator not found");
            }
        }

        console.log(userName + " '" + userEmail + "'");
    }

    if (!userEmail || !validateEmailAddress(userEmail)) {
        setStatusDisplay(ERROR, "freeGoldStatus", "Failed to send free gold - no valid email.");
        return;
    }

    // OK, let's actually give gold!

    const freeGoldUser = {
        'name': userName,
        'email': userEmail,
    };
    const response = await jsonFetch("freegold", freeGoldUser);
    if (response) {
        console.log("gave free gold to " + userName + " '" + userEmail + "'\n" + response.magicLink);
        setStatusDisplay(SUCCESS, "freeGoldStatus", "Gave free gold to " + userName + " '" + userEmail + "'\n");
        console.log(response);
    } else {
        setStatusDisplay(ERROR, "freeGoldStatus", "Failed to give free gold");
    }
    return false;
}

async function handleGetMagicLink(event) {
    event.preventDefault();
    const email = document.getElementById("magiclink_email").value.trim();
    const magicLink = await jsonFetch("getmagiclink", {
        'email': email,
    });
    magicLinkDest = document.getElementById("magic_link");
    if (magicLink) {
        if (magicLinkDest) {
            magicLinkDest.innerHTML = magicLink.link;
            // Also copy to clipboard
            await navigator.clipboard.writeText(magicLink.link);
        }
        console.log("success: " + magicLink.link);
        setStatusDisplay(SUCCESS, "magicLinkStatus", "Link fetched.");
    } else {
        if (magicLinkDest) {
            magicLinkDest.innerHTML = "";
        }
        setStatusDisplay(ERROR, "magicLinkStatus", "Error getting magic link.");
    }
    return false;
}

async function handleAddGooglePlayCodes(event) {
    event.preventDefault();

    const codeString = document.getElementById("pending_codes").value.trim();
    // TODO: Validate contents better.
    let codes = codeString.split(/\r?\n/).filter(x => x.length > 0).map(x => x.trim());
    if (codes.length == 0) {
        console.log("nothing to do");
        return;
    }

    console.log(codes);
    var result = await jsonPost("googleplaycodeadmin", {
        'addCodes': codes,
    });
    if (result) {
        console.log("success");
        setStatusDisplay(SUCCESS, "googlePlayStatus", "Codes added.");
    } else {
        console.log("failure");
        setStatusDisplay(ERROR, "googlePlayStatus", "Failed to add codes.");
    }

    await updatePlayCodesStats();

    console.log("google play codes");
    return false;
}

function processLoginData(loginData) {
    console.log(loginData);
    const userData = {
        'loggedIn': true,
        'goldUser': loginData.goldUser,
        'admin': loginData.admin,
        'name': loginData.name,
        'email': loginData.email,
        'loginType': loginData.loginType,
    };

    console.log("Setting user data: ");
    console.log(userData);

    g_userData = userData;

    // Store the data for the next page load.
    localStorage.setItem('ppsspp-auth', JSON.stringify(userData));
}

async function handleLoginForm(event) {
    event.preventDefault();

    const email = document.getElementById("email").value.trim();
    const password = document.getElementById("password").value.trim();

    const queryParams = new URLSearchParams(location.search);
    const forward = translateForward(queryParams.get('Forward'), "set form forward");

    if (!validateEmailAddress(email)) {
        setStatusDisplay(ERROR, "loginStatus", BAD_EMAIL_ADDRESS);
        return false;
    }
    if (!password) {
        setStatusDisplay(ERROR, "loginStatus", MISSING_PASSWORD);
        return false;
    }

    const credentials = {
        'email': email,
        'password': password,
        'key': "",
    };
    console.log(credentials);
    const loginData = await jsonFetch("login", credentials);
    if (loginData) {
        processLoginData(loginData);

        if (forward) {
            console.log("Forwarding to " + forward);
            window.location.href = "/" + forward;
        }
        setStatusDisplay(HIDDEN, "loginStatus");
    } else {
        setStatusDisplay(ERROR, "loginStatus", "Email/Password didn't match, or account doesn't exist.");
    }

    applyDOMVisibility();

    return false;  // irrelevant since we're in async context here
}

async function handleLoginByKey() {
    const queryParams = new URLSearchParams(location.search);
    const queryEmail = queryParams.get('Email');
    const queryKey = queryParams.get('Key');
    var forward = null;
    // Note: We allow logins by key if already logged in by password.
    // This is so that password recovery emails can still be opened if you happened to forget
    // the password while you were already logged in...
    if (queryEmail && queryKey && (!g_userData.loggedIn || g_userData.loginType == "password")) {
        // Automated login by key
        var credentials = {
            'email': queryEmail,
            'key': queryKey,
        };
        console.log("Trying to log in by key: " + credentials.email + " " + credentials.key);
        const loginData = await jsonFetch("login", credentials, null);
        if (loginData) {
            processLoginData(loginData);
            // setLoginFailed(false);
            forward = translateForward(queryParams.get('Forward'), "effect");
            console.log("Login-by-key success");
            setStatusDisplay(HIDDEN, "loginStatus");
        } else {
            console.log("No login data in response, not logging in. forward was " + forward);
            // setLoginFailed(true);
            // Remove query parameters. This causes some kind of wacky redirect loop though!
            // queryParams.delete('error')
            setStatusDisplay(ERROR, "loginStatus", "Failed to login by key.");
        }
    } else if (g_userData.loggedIn) {
        // Just proceed with the forward, we're already logged in.
        forward = translateForward(queryParams.get('Forward'), "effect");
    }
    if (forward) {
        var url = "/" + forward;
        console.log("Attempting to navigate to forwarding URL " + url);
        window.location.href = url;
    } else {
        console.log("not forwarding, forward == null");
    }
}

async function handleChangePasswordForm(event) {
    event.preventDefault();
    const status = "changePasswordStatus";
    const oldPassword = document.getElementById("old_password").value.trim();
    const newPassword = document.getElementById("new_password").value.trim();

    if (newPassword.length < 8) {
        setStatusDisplay(ERROR)
    }

    var result = await jsonPost("changepassword", { oldPassword: oldPassword, newPassword: newPassword });
    if (result) {
        console.log(result);
        setStatusDisplay(SUCCESS, status, "Password changed");
        // window.location.href = "/login";
    } else {
        setStatusDisplay(ERROR, status, "Failed to change password. Was your old password correct?");
    }

    return false;
}

async function handleRecoverPasswordForm(event) {
    event.preventDefault();
    const status = "recoverPasswordStatus";
    const email = document.getElementById("email").value.trim();
    if (!validateEmailAddress(email)) {
        setStatusDisplay(ERROR, status, BAD_EMAIL_ADDRESS);
        return false;
    }
    await jsonPost('recoverpassword', { email: email }, null, null); // not fetch!
    // Don't reveal success or failure.
    setStatusDisplay(SUCCESS, status, "E-mail with recover link sent to your account, if valid.");
    return false;
}

async function handleRequestPromoCodeForm(event) {
    const status = "requestPromoCodeStatus";
    event.preventDefault();
    var result = await jsonFetch("getgoogleplaycode", null, null);
    if (result) {
        setStatusDisplay(SUCCESS, status, result.code);
    } else {
        setStatusDisplay(ERROR, status, "Failed to make request - duplicate?");
    }
}

function loadCredentials() {
    // This is done on every page load. We get credentials from localStorage
    // and update the UI accordingly.
    const cookie = localStorage.getItem('ppsspp-auth');
    if (cookie) {
        console.log("Loading credentials...");
        const userDataFromCookie = JSON.parse(cookie);
        if (userDataFromCookie) {
            g_userData = userDataFromCookie;
        }
    } else {
        console.log("No credentials, not logged in.");
        g_userData = defaultUserContext;
    }

    if (g_loginByKey) {
        handleLoginByKey();
    }
}

async function logoutUser() {
    // Reset user data
    g_userData = defaultUserContext;
    // Reset the localStorage cache so we don't appear logged-in on the next page load.
    localStorage.removeItem('ppsspp-auth');
    // Reset the server cookie
    await jsonPost("logout", null);
    // Apply changes to the UI.
    applyDOMVisibility();
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

// Fastspring thankyou globals.

var g_pollInterval = 500;

// For the thank-you-page. TODO: Organize things more sensibly.

// Two references from the test backend: PPS230228-4621-17116  PPS230228-4621-52125

const tmplShowSuccessfulPurchase = `
<p>Thank you!</p>
<p>An e-mail with your login info has been sent to <strong>{{it.email}}</strong>.</p>
<p>NOTE: If no e-mail arrives within a few minutes, please check your spam box.
If it's not there or you want to use a different e-mail address for login,
then <a href="mailto:hrydgard+ppssppgold@gmail.com">e-mail me</a> and I'll sort it out.</p>
<p><a href="{{it.magicLink}}" class="download-button button-gold" style="display:block;">Click here to log in!</a></p>
<p>Paid: {{it.totalDisplay}} ({{it.currency}})</p>
`;

const tmplShowPendingPurchase = `
<p>Thank you!</p>
<p>Confirming purchase for {{it.totalDisplay}} made by {{it.email}}.</p>
<p>(If this e-mail address is incorrect, <a href="mailto:hrydgard+ppssppgold@gmail.com">contact support</a>.)</p>
`;

const tmplShowBadPurchase = `

`;

async function pollPurchase() {
    console.log("Thank you poll");

    const queryString = window.location.search;
    const urlParams = new URLSearchParams(queryString);
    const reference = urlParams.get('orderId');

    if (!reference) {
        // Page was loaded with wrong parameters.
        setStatusDisplay(ERROR, "purchaseStatus", "Missing purchase reference (?orderId)");
        return;
    }

    console.log("Order reference: " + reference);

    const statusData = await jsonFetch("getorderstatus", { reference: reference });
    if (statusData) {
        console.log(statusData);
        if (statusData.paymentStatus == "completed") {
            setStatusDisplay(SUCCESS, "purchaseStatus", "Purchase successful!");
            const purchaseInfo = document.getElementById("purchaseInfo");
            purchaseInfo.innerHTML = Sqrl.render(tmplShowSuccessfulPurchase, statusData);
        } else {
            const purchaseInfo = document.getElementById("purchaseInfo");
            purchaseInfo.innerHTML = Sqrl.render(tmplShowPendingPurchase, statusData);
            console.log("Purchase not confirmed completed, keeping polling.");
            if (g_pollInterval < 10000) {
                g_pollInterval += 1000;  // ms
            }
            window.setTimeout(pollPurchase, g_pollInterval);
        }
    } else {
        setStatusDisplay(INFO, "purchaseStatus", "Confirming purchase... (might take a while)");
        console.log("Failed, trying again.");
        if (g_pollInterval < 10000) {
            g_pollInterval += 1000;  // ms
        }
        window.setTimeout(pollPurchase, g_pollInterval);
    }
}

const tmplShowCommitList = `
<div class="col-12">
<div>Latest {{it.length}} build(s):</div>
{{@each(it) => version}}
{{@if(version.builds)}}
<div class="card">
<div class="card-title">
    <h2 class="no-icon">{{version.tag}}-{{version.revs_since_tag}}</h2>
</div>
{{@if(version.pr)}}
<p>Merge <a href="https://github.com/hrydgard/ppsspp/pull/{{version.pr.number}}">pull request {{version.pr.number}}</a> by <a href="https://github.com/{{version.pr.author_username}}">{{version.pr.author_username}}</a> on {{version.date}}:</p>
<p><strong>{{version.message}}</strong></p>
{{#else}}
<p><a href="https://github.com/hrydgard/ppsspp/commit/{{version.hash_short}}">Commit {{version.hash_short}}</a> by {{version.author_name}}</p>
<p><strong>{{version.message}}</strong></p>
{{/if}}
{{@foreach(version.builds) => platform, files}}
<p>{{platform}}:
{{@each(files) => file}}
<a href="https://builds.ppsspp.org/builds/{{version.description}}/{{file}}">{{file}}</a>
{{/each}}
</p>
{{/foreach}}
</div>
{{/if}}
{{/each}}
</div>
`;

const tmplBuildbotStatus = `
<div class="col-12">
<div class="card">
<div class="card-title">
    <h2 class="no-icon">Buildbot Status</h2>
</div>
{{@if(it.building)}}
<p>Building {{it.tag}}-{{it.revs}} for {{it.building}} ({{it.commit_date}})</p>
{{#else}}
<p>Idle</p>
{{/if}}
<p>Last updated: {{it.lastUpdated}}</p>
</div>
</div>
`;

async function loadDownloads() {
    console.log("Loading downloads...");

    const latest = document.getElementById("latestBuilds");
    const status = document.getElementById("buildbotStatus");
    if (latest || status) {
        const statusData = await fetch("https://builds.ppsspp.org/meta/status.json").then(response => response.json()).catch(error => console.error("Error fetching json: " + error));
        console.log(statusData);

        if (latest) {
            latest.innerHTML = Sqrl.render(tmplShowCommitList, [statusData.latest]);
        }
        if (status) {
            let unix = Date.now() / 1000;
            let ago = unix - statusData.unix_time;
            let lastUpdated = Math.floor(ago) + " seconds ago";
            var data;
            if (statusData.building) {
                data = {
                    "building": statusData.building_platform,
                    "tag": statusData.building.tag,
                    "revs": statusData.building.revs_since_tag,
                    "commit_date": statusData.building.date,
                    "lastUpdated": lastUpdated
                };
            } else {
                data = { "building": null, "lastUpdated": lastUpdated };
            }
            status.innerHTML = Sqrl.render(tmplBuildbotStatus, data);
        }
    }


    const twentyBuilds = document.getElementById("twentyBuilds");
    if (twentyBuilds) {
        const historyData = await fetch("https://builds.ppsspp.org/meta/history-20.json").then(response => response.json()).catch(error => console.error("Error fetching json: " + error));
        twentyBuilds.innerHTML = Sqrl.render(tmplShowCommitList, historyData);
    }
}

async function loadAllBuilds() {
    const twentyBuilds = document.getElementById("twentyBuilds");
    if (twentyBuilds) {
        const historyData = await fetch("https://builds.ppsspp.org/meta/history.json").then(response => response.json()).catch(error => console.error("Error fetching json: " + error));
        twentyBuilds.innerHTML = Sqrl.render(tmplShowCommitList, historyData);

        const showAllButton = document.getElementById("showAllButton");
        if (showAllButton) {
            showAllButton.style.display = "none";
        }
        const showAllButton2 = document.getElementById("showAllButton2");
        if (showAllButton2) {
            showAllButton2.style.display = "none";
        }
    }
}

function setupCollapsibles() {
    // UI utilities
    // See the collapsible css styles in ui.css.
    var coll = document.getElementsByClassName("collapsible");
    var i;
    for (i = 0; i < coll.length; i++) {
        coll[i].addEventListener("click", function () {
            console.log("collapse");
            this.classList.toggle("active");
            var content = this.nextElementSibling;
            if (content.style.display === "block") {
                content.style.display = "none";
            } else {
                content.style.display = "block";
            }
        });
    }
}

async function checkStillLoggedIn() {
    try {
        // 2. Use 'await' to wait for the fetch request to complete
        const code = await jsonFetchGetCode("testValidated");
        switch (code) {
        case 200:
            console.log("checkStillLoggedIn: user is still logged in");
            break;
        case 500:
        case 501:
        case 502:
        case 503:
        case 504:
        case 511:
            console.log("server error, let's not log out the user");
            break;
        case 400:
        case 401:
        case 403:
            // Bad return values. User is not logged in.
            console.log("checkStillLoggedIn: user is not logged in (status " + code + "), logging out");
            logoutUser();
            window.location.href = '/login'; // Or any other URL
            break;
        }
        // ... do something with the data ...
    } catch (error) {
        // 4. If the fetch fails, this block will run
        console.error('Fetch failed:', error);
    }
}

function onLoadPage() {
    loadCredentials();
    applyDOMVisibility();
    setupCollapsibles();
    if (g_thankYouPage) {
        window.setTimeout(pollPurchase, g_pollInterval);
    }
    if (g_buildbotPage) {
        console.log("Buildbot page detected")
        window.setTimeout(loadDownloads, 1);
    }
    if (g_downloadPage) {
        console.log("Download page detected")
        if (g_userData.loggedIn) {
            console.log("Checking logged-in state with server");
            window.setTimeout(checkStillLoggedIn, 1);
        } else {
            console.log("Not logged in, not checking server");
        }
    }

    if (typeof hljs !== 'undefined') {
        console.log("highlighting");
        hljs.highlightAll();
    }

    // Make the burger button keyboard accessible
    var toggleButton = document.getElementById('burgerButton');
    toggleButton.addEventListener('keydown', function (event) {
        if (event.key === 'Enter') {
            burgerClick();
        }
    });

    // Close the content when clicking outside the div
    document.addEventListener('click', function (event) {
        var rootSidebar = document.getElementById('rootSidebar');
        var toggleButton = document.getElementById('burgerButton');

        if (rootSidebar && !rootSidebar.contains(event.target) && event.target != toggleButton) {
            rootSidebar.classList.add('hidden');
        }
    });
}

function burgerClick() {
    var contentDiv = document.getElementById('rootSidebar');
    contentDiv.classList.toggle('hidden');
    console.log("clicked the burger");
}

// Fastspring stuff

function onFSPopupClosed(orderReference) {
    if (orderReference) {
        console.log("Order completed - redirecting!");
        console.log(orderReference.reference);
        fastspring.builder.reset();
        var url = "/thankyou?orderId=" + orderReference.reference;
        try { window.location.replace(url); }
        catch (e) { window.location = url; }
    } else {
        console.log("no order ID - won't redirect to thankyou page");
    }
}

function onFSError(code, string) {
    console.log("FastSpring error detected: ", code, string);
    setStatusDisplay(WARNING, "fastspring-error-alert", "The PPSSPP Store is having some technical issues right now. Will be back soon.")
    var productCards = document.getElementById('product-cards');
    productCards.style.display = "none";
}

// change the mode by adding the attribute 'data-theme' on the element <html>. It also creates a cookie to record the mode selected by the user.
function switchTheme() {
    if (document.documentElement.getAttribute('data-theme') === 'dark') {
        document.documentElement.setAttribute('data-theme', 'light');
        localStorage.setItem("theme", "light");
    } else {
        document.documentElement.setAttribute('data-theme', 'dark');
        localStorage.setItem("theme", "dark");
    }
}

console.log("initial script execution");

document.addEventListener('DOMContentLoaded', function () {
    console.log("DOMContentLoaded");
    onLoadPage();
});

/**
* Automatic mode assignment according to user-selected system preference.
*/
let curTheme = localStorage.getItem("theme");
if (curTheme) {
    document.documentElement.setAttribute('data-theme', curTheme);
} else {
    // Let's default to dark instead of asking the system.
    /*
    if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
        document.documentElement.setAttribute('data-theme', 'dark');
    } else {
        document.documentElement.setAttribute('data-theme', 'light');
    }
    */
    document.documentElement.setAttribute('data-theme', 'dark');
}

// Error messages
const BAD_EMAIL_ADDRESS = "Not a valid e-mail address ";
const MISSING_PASSWORD = "Please enter a password";
