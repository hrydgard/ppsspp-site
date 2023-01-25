import React from 'react';
import clsx from 'clsx';
import AdResponsive from '../components/AdSense';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Link from '@docusaurus/Link';
import Layout from '@theme/Layout';

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
          <Link to={"/download"} className="button button--block button--primary margin-bottom--md hero-button"
            style={{display: "flex", flexDirection: "row", alignItems: "center" }}>
              <img src={normalIcon} width="26px" height="26px"/>&nbsp;&nbsp;Download
          </Link>
          <Link to={"/buygold"} className="button button--block button--primary margin-bottom--md hero-button"
            style={{display: "flex", flexDirection: "row", alignItems: "center"}}>
              <img src={goldIcon} width="26px" height="26px"/>&nbsp;&nbsp;Buy PPSSPP Gold
          </Link>

        </div>
      </div>
    </header>
  );

  // Needs a different background. should be below the buttons.
  // <p style={{"text-align": "left"}}><a href="/docs/intro">Getting started</a></p>
}

const FeatureList = [
  {
    title: 'Play your PSP games in HD!',
    description: (
      <>
          PPSSPP can run your PSP games on your PC in full HD resolution, and play them on Android too. It can even upscale textures that would otherwise be too blurry as they were made for the small screen of the original PSP.
          On modern Android phones and tablets, you can run at double the original resolution or more.
      </>
    ),
  },
  {
    title: 'Enhance your experience!',
    description: (
      <>
          <ul>
            <li>Play in HD resolutions and more</li>
            <li>Play on a tablet for big-screen mobile gaming</li>
            <li>Customize on-screen touch controls or use an external controller or keyboard</li>
            <li>Save and restore game state anywhere, anytime</li>
            <li>Crank up the anisotropic filtering and texture scaling</li>
            <li>Continue where you left off by transferring saves from your real PSP</li>
          </ul>
      </>
    ),
  },
  {
    title: 'Free & Open Source',
    description: (
      <>
          <p>PPSSPP is an open source project, licensed under the GPL 2.0 (or later).
          Anyone is welcome to contribute improvements to the code.
          Thanks to many such contributions, PPSSPP's compatibility is steadily increasing,
          letting us all play our PSP games on the devices of our choice.</p>
          <p><a className="btn" href="/doc/development">Development &raquo;</a>
          <br></br>
          <a className="btn" href="https://github.com/hrydgard/ppsspp">GitHub &raquo;</a></p>
       </>
    ),
  },
];

function Feature({Svg, title, description}) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--left padding-horiz--md padding-vert--md">
        <h3>{title}</h3>
        {description}
      </div>
    </div>
  );
}

function HomepageFeatures() {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
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
