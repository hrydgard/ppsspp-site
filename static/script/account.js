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
        console.log("401, logging out");
        logoutUser();
    }
}

async function jsonFetch(apiName, requestBody) {
    return fetch('/api/' + apiName, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: requestBody ? JSON.stringify(requestBody) : null,
    }).then(data => {
        statusCodeAction(data.status);
        if (data.status == 200) {
            return data.json();
        } else {
            return null;
        }
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
        statusCodeAction(data.status, setUserData);
        return data.status == 200;
    });
}

function setDisplayMode(className, mode) {
    console.log("Setting all with " + className + " to " + mode);
    for (let element of document.getElementsByClassName(className)) {
        console.log(element);
        element.style.display = mode;
    }
}

// It's a little awkward that unlike handlebars, sqrl has these "it." everywhere.
// Also, ifs and each look a bit different.

// TODO: See if we can find a better client-side templating solution. While it's nice to
// avoid react, this does get a little awkward, vscode fails to syntax color this.
const tmplUserInfo = `
<div class="ms-card ms-fill">
<p>{{it.name}}</p>
<p>Email: {{it.email}}.</p>
<p>Gold: {{it.goldUser}}.</p>
</div>
`;

// Not really secret panel, even if you know what this can do you can't do anything :P
const tmplAdminPanel = `
<div class="ms-card ms-fill">
<p>{{it.name}}.</p>
<p>Email: {{it.email}}.</p>
<p>Gold: {{it.goldUser}}.</p>
<p>ADMIN STATUS</p>
</div>

<div class="ms-card ms-fill">
<h2>Give free gold</h2>
<form action="#" onSubmit="return handleGiveFreeGold(event)">
<div class="alert alert--warning hidden" id="error_message" role="alert"></div>
<label>
    <div>Name</div>
    <span><input type="text" size="38" id="freegold_name" /></span>
    <div>E-mail address</div>
    <span><input type="text" size="38" id="freegold_email" /></span>
</label>
<div>
    <button className="button button--primary margin-top--md" type="submit">Give free gold</button>
</div>
</form>
</div>

<div class="ms-card ms-fill">
<h2>Get magic link</h2>
<form action="#" onSubmit="return handleGetMagicLink(event)">
<div class="alert alert--warning hidden" id="error_message" role="alert"></div>
<label>
    <div>E-mail address</div>
    <span><input type="text" size="38" id="magiclink_email" /></span>
</label>
<div>
    <button className="button button--primary margin-top--md" type="submit">Get magic link!</button>
</div>
<div id="magic_link">...</div>
</form>
</div>

<div class="ms-card ms-fill">
<h2>Google Play Codes</h2>

<div id="add_codes_result">...</div>

<form action="#" onSubmit="return handleAddGooglePlayCodes(event)">
<label>
  <div>Codes to add:</div>
  <textarea rows="10" cols="24" id="pending_codes"}></textarea>
</label>
<div>
  <button className="button button--primary margin-top--md" type="submit">Add promo codes</button>
</div>
</form>

</div>

</div>
`;

const tmplPlayCodes = `
<p>Codes used: {codesUsed}</p>
<p>Codes left: {codesLeft}</p>
`;

const tmplLoginCorner = `
<a href='/login'>
{{@if(it.loggedIn)}}
<img
{{@if(it.goldUser)}}
src="/static/img/platform/ppsspp-icon-gold.png"
{{#else}}
src="/static/img/platform/ppsspp-icon.png"
{{/if}}
width="22px" height="22px" />&nbsp;{{it.name}}
{{#else}}
Login
{{/if}}
</a>
`;

function applyDOMVisibility() {
    // Hides and shows stuff depending on your current login state.
    if (g_userData.loggedIn) {
        setDisplayMode("logged-in-only", g_userData.loggedIn ? "block" : "none");
        setDisplayMode("not-logged-in-only", g_userData.loggedIn ? "none" : "block");
        setDisplayMode("logged-in-only-inline", g_userData.loggedIn ? "inline" : "none");
        setDisplayMode("not-logged-in-only-inline", g_userData.loggedIn ? "none" : "inline");
    } else {
        setDisplayMode("logged-in-only", "none");
        setDisplayMode("not-logged-in-only", "block");
    }

    setDisplayMode("gold-only", g_userData.goldUser ? "block" : "none");
    setDisplayMode("no-gold-only", g_userData.goldUser ? "none" : "block");

    // Update various textual fields if present on the page.
    const loginName = document.getElementById("displayName");
    if (loginName) {
        loginName.innerHTML = "<a href='/login'>" + g_userData.name + "</a>";
    }
    const loginEmail = document.getElementById("displayEmail");
    if (loginEmail) {
        loginEmail.innerText = g_userData.email;
    }
    const loginCorner = document.getElementById("loginCorner");
    if (loginCorner) {
        loginCorner.innerHTML = Sqrl.render(tmplLoginCorner, g_userData);
    }
    const loginInfo = document.getElementById("loginInfo");
    if (loginInfo) {
        loginInfo.innerHTML = Sqrl.render(g_userData.admin ? tmplAdminPanel : tmplUserInfo, g_userData);
    }
}

async function handleGiveFreeGold(event) {
    event.preventDefault();
    let userName = document.getElementById("freegold_name").value.trim();
    let userEmail = document.getElementById("freegold_email").value.trim();
    console.log("Was gonna give free gold to " + userEmail + " " + userName);

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

        console.log(userName + " '" + userEmail + "'");
    }

    if (!userEmail) {
        return;
    }

    // OK, let's actually give gold!

    var freeGoldUser = {
        'name': userName,
        'email': userEmail,
    };
    const response = await jsonFetch("freegold", freeGoldUser);
    if (response) {
        console.log("gave free gold to " + userName + " '" + userEmail + "'");
    }
    return false;
}

async function handleGetMagicLink(event) {
    event.preventDefault();
    const email = document.getElementById("magiclink_email").value.trim();
    var magicLink = await jsonFetch("getmagiclink", {
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
    } else {
        if (magicLinkDest) {
            magicLinkDest.innerHTML = "";
        }
    }
    return false;
}

async function handleAddGooglePlayCodes(event) {
    event.preventDefault();
    console.log("google play codes");


    return false;
}

async function handleLoginForm(event) {
    event.preventDefault();

    const email = document.getElementById("email").value.trim();
    const password = document.getElementById("password").value.trim();

    const queryParams = new URLSearchParams(location.search);
    const forward = translateForward(queryParams.get('Forward'), "set form forward");

    // TODO: Pre-verify email here.

    var credentials = {
        'email': email,
        'password': password,
        'key': "",
    };
    console.log(credentials);
    const loginData = await jsonFetch("login", credentials);
    console.log(loginData);
    if (loginData) {
        console.log(loginData);
        var userData = {
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

        if (forward) {
            console.log("Forwarding to " + forward);
            window.location.href = "/" + forward;
        }
    } else {
        setErrorMessage("Email/Password didn't match.");
    }

    applyDOMVisibility();

    return false;  // irrelevant since we're in async context here
}

function loadCredentials() {
    console.log("Loading credentials...");
    // This is done on every page load. We get credentials from localStorage
    // and update the UI accordingly.
    const cookie = localStorage.getItem('ppsspp-auth');
    if (cookie) {
        var userDataFromCookie = JSON.parse(cookie);
        if (userDataFromCookie) {
            // console.log("Found cookie with below userdata:");
            // console.log(userDataFromCookie);
            g_userData = userDataFromCookie;
        }
    }
}

async function logoutUser() {
    // Reset user data immediately
    g_userData = defaultUserContext;
    // Reset the local cookie so we don't auto-login on the next load.
    localStorage.removeItem('ppsspp-auth');
    // Reset the server cookie
    await jsonPost("logout", null);

    applyDOMVisibility();
}

function setErrorMessage(msg) {
    console.log("Error: " + msg);
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

function onLoadPage() {
    loadCredentials();
    applyDOMVisibility();
}

console.log("initial script execution");

document.addEventListener('DOMContentLoaded', function () {
    console.log("DOMContentLoaded");
    onLoadPage();
});

// window.onload = onLoadPage;