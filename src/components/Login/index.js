import React from 'react';

import { useUserData } from '@site/src/theme/Root';

export default function LoginCorner() {
  const { userData, setUserData } = useUserData();

  var iconName;
  if (userData['goldUser']) {
    iconName = 'ppsspp-icon-gold.png';
  } else {
    iconName = "ppsspp-icon.png";
  };
  iconName = "/img/platform/" + iconName;

  if (userData.loggedIn) {
    // TODO: Add a &Forward= attribute derived from the current page URI to the link?
    return (
      <>
        <img src={iconName} width="22px" height="22px"/>
        &nbsp;
        <a href="/login">{userData["name"]}</a>
      </>
    )
  } else {
    return (
      <a href="/login">Login</a>
    );
  }
}
