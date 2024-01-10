/*
* dark-mode-switcher - Pure JS script allow you to simply switch your web interface in dark mode.
* Repo : https://github.com/Airmime/dark-mode-switcher
* Licence : MIT
* Version : v0.0.1
*/

/**
* Function allowing to change the mode by adding the attribute 'data-theme' on the element <html>. It also creates a cookie to record the mode selected by the user.
*/
function switchTheme() {
    if (document.documentElement.getAttribute('data-theme') === 'dark') {
        document.documentElement.setAttribute('data-theme', 'light');
        setcookie('light');
    } else {
        document.documentElement.setAttribute('data-theme', 'dark');
        setcookie('dark');
    }
}

/**
* Function to create a cookie for the selected theme.
* @param cookieValue Mode to be valued in the cookie.
* The cookie will have a lifespan of 14 days.
*/
function setcookie(cookieValue) {
    var today = new Date();
    var expire = new Date();
    expire.setTime(today.getTime() + 3600000 * 24 * 14);
    document.cookie = "theme =" + encodeURI(cookieValue) + "; expires=" + expire.toGMTString() + ";path=/";
}

/**
* A method of retrieving the value of a cookie.
* @param name Cookie name.
*/
function getCookie(name) {
    const value = `; ${document.cookie}`;
    const parts = value.split(`; ${name}=`);
    if (parts.length === 2) return parts.pop().split(';').shift();
}

/**
* Automatic mode assignment according to user-selected system preference.
*/
if (getCookie('theme') != null) {
    document.documentElement.setAttribute('data-theme', getCookie('theme'));
} else {
    /*
    if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
        document.documentElement.setAttribute('data-theme', 'dark');
    } else {
        document.documentElement.setAttribute('data-theme', 'light');
    }
    */
    // we always want dark as default.
    document.documentElement.setAttribute('data-theme', 'dark');
}
