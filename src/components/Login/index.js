import React from 'react';

import { useUserData } from '@site/src/theme/Root';
import Link from '@docusaurus/Link';

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
        <Link to="/login">{userData["name"]}</Link>
      </>
    )
  } else {
    return (
      <Link to="/login">Login</Link>
    );
  }
}
