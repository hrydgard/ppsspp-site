
function statusCodeAction(status, setUserData) {
    if (status == 401) {
        // If any API call returns Unauthorized, we'll simply
        // log out the user, since this should only ever happen if
        // (1) The user doesn't have the required status, in which case there might be something suspicious afoot
        // (2) The user login expired

        // Reset user data immediately
        if (setUserData) {
            console.log("401, logging out");
            setUserData(defaultUserContext);
            // Reset the local cookie so we don't auto-login on the next load.
            localStorage.removeItem('ppsspp-auth');
            // Reset the server cookie
            jsonPost("logout", null);
        } else {
            console.log("401, not logging out because !setUserData");
        }
    }
}

export async function jsonFetch(apiName, requestBody, setUserData) {
    return fetch('/api/' + apiName, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: requestBody ? JSON.stringify(requestBody) : null,
    }).then(data => {
        statusCodeAction(data.status, setUserData);
        if (data.status == 200) {
            return data.json();
        } else {
            return null;
        }
    });
}

// This one doesn't expect a JSON response, just a status.
export async function jsonPost(apiName, requestBody, setUserData) {
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