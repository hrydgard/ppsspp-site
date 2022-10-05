import React from 'react';
import BrowserOnly from '@docusaurus/BrowserOnly';
import { createContext, useMemo, useState, useContext } from 'react';

// Global context keeping the current login.
// We also store this in a cookie.
const UserDataContext = createContext({});

export const useUserData = () => useContext(UserDataContext);

 // NOTE: This struct should match the LoginResponse used on the go side.
export const defaultUserContext = {
    // Permissions. These are also checked on the server each request,
    // so these state bools are only here to easily hide irrelevant UI etc.
    loggedIn: false,
    goldUser: false,
    admin: false,
    loginType: "password",

    name: "NotLoggedIn",
    email: "notloggedin@fake.com",
};

function ContentWrapper({children}) {
    var initialUserData = defaultUserContext;

    const cookie = localStorage.getItem('ppsspp-auth');
    if (cookie) {
        var userDataFromCookie = JSON.parse(cookie);
        if (userDataFromCookie) {
            // console.log("Found cookie: ");
            // console.log(userDataFromCookie);
            initialUserData = userDataFromCookie;
        }
    }

    const [userData, setUserData] = useState(initialUserData);
    const context = useMemo(() => ({ userData, setUserData }), [userData, setUserData]);

    return (
        <>
        <UserDataContext.Provider value={context}>
            {children}
        </UserDataContext.Provider>
        </>
    );
}

export default function Root({children}) {
    // TODO: Find a way to only surround the navbar (with the login indicator) on most pages
    // with <BrowserOnly>, instead of wrapping the whole page in it, which kind of defeats much
    // of the purpose of static hosting.
    return (<BrowserOnly>{
        () => {
            return <ContentWrapper children={children}/>
        }
    }
    </BrowserOnly>);
}
