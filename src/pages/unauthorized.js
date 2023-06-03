import React from 'react';
import Link from '@docusaurus/Link';
import Layout from '@theme/Layout';

import { useUserData } from '@site/src/theme/Root';

export default function Home() {
  const {userData, setUserData} = useUserData();

  if (userData.loggedIn) {
    return (
      <Layout title="You're logged in">
        <p>Logged in. <Link to="/">Back to home</Link></p>
      </Layout>
    )
  }

  return (
    <Layout title="Something went wrong">
        <p>Session timed out, or there was a server problem.</p>
        <p>Please <Link to="/login">login again</Link>.</p>
    </Layout>
  );
}
