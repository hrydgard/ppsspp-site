import React from 'react';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import DownloadList from '@site/src/components/DownloadList';
import { useUserData } from '@site/src/theme/Root';
import AdResponsive from '../components/AdSense';

export default function Home() {
  const {siteConfig} = useDocusaurusContext();
  const {userData, setUserData} = useUserData();

  return (
    <Layout
      title={`Downloads`}
      description="Downloads for PC, Android, Mac, Linux, iPhone, iPad, iOS">
      <AdResponsive />
      <DownloadList userData={userData} />
    </Layout>
  );
}
