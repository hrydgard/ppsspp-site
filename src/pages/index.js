import React from 'react';
import clsx from 'clsx';
import AdResponsive from '../components/AdSense';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import HomepageFeatures from '@site/src/components/HomepageFeatures';

import styles from './index.module.css';

function HomepageHeader() {
  const {siteConfig} = useDocusaurusContext();

  var normalIcon = "/img/platform/ppsspp-icon.png";
  var goldIcon = "/img/platform/ppsspp-icon-gold.png";

  return (
    <header className={clsx('hero hero--primary', styles.heroBanner)}>
      <div className='light x1'></div>
      <div className='light x2'></div>
      <div className='light x3'></div>
      <div className='light x4'></div>
      <div className='light x5'></div>
      <div className='light x6'></div>
      <div className='light x7'></div>
      <div className='light x8'></div>
      <div className='light x9'></div>
      <div className="container">
        <div className="col col--3">
          <h1 className="hero__title">{siteConfig.title}</h1>
          <p className="hero__subtitle">{siteConfig.tagline}</p>
          <a href={"/download"}>
            <button className="button button--block button--primary margin-bottom--md hero-button"
              style={{display: "flex", flexDirection: "row", alignItems: "center" }}>
               <img src={normalIcon} width="26px" height="26px"/>&nbsp;&nbsp;Download
            </button>
          </a>
          <a href={"/buygold"}>
            <button className="button button--block button--primary margin-bottom--md hero-button"
              style={{display: "flex", flexDirection: "row", alignItems: "center"}}>
               <img src={goldIcon} width="26px" height="26px"/>&nbsp;&nbsp;Buy PPSSPP Gold
            </button>
          </a>

        </div>
      </div>
    </header>
  );

  // Needs a different background. should be below the buttons.
  // <p style={{"text-align": "left"}}><a href="/docs/intro">Getting started</a></p>
}

// To play with: https://kgajera.com/blog/display-recent-blog-posts-on-home-page-with-docusaurus/

function RecentNews() {
  return <></>;
}

export default function Home() {
  const {siteConfig} = useDocusaurusContext();
  return (
    <Layout
      title={siteConfig.title + " - PSP emulator for Android, Windows, Linux, iOS, macOS"}
      description="PSP emulator for Android, Windows, Linux, iOS, macOS">
      <HomepageHeader />
      <main>
        <HomepageFeatures />
      </main>
      <AdResponsive />
      <RecentNews />
    </Layout>
  );
}
