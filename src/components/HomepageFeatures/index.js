import React from 'react';
import clsx from 'clsx';
import styles from './styles.module.css';

const FeatureList = [
  {
    title: 'Play your PSP games in HD!',
//    Svg: require('@site/static/img/undraw_docusaurus_mountain.svg').default,
    description: (
      <>
          PPSSPP can run your PSP games on your PC in full HD resolution, and play them on Android too. It can even upscale textures that would otherwise be too blurry as they were made for the small screen of the original PSP.
          Even on modern Android phones and tablets, you can often run at double the original resolution.
      </>
    ),
  },
  {
    title: 'Enhance your experience!',
//    Svg: require('@site/static/img/undraw_docusaurus_tree.svg').default,
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
//    Svg: require('@site/static/img/undraw_docusaurus_react.svg').default,
    description: (
      <>
          <p>PPSSPP is an open source project, licensed under the GPL 2.0 (or later). 
          Anyone is welcome to contribute improvements to the code.
          Thanks to many such contributions, PPSSPP's compatibility is steadily increasing,
          letting us all play our PSP games on the devices of our choice.</p>
          <p><a class="btn" href="/doc/development">Development &raquo;</a>
          <br></br>
          <a class="btn" href="https://www.github.com/hrydgard/ppsspp">GitHub &raquo;</a></p>
       </>
    ),
  },
];

function Feature({Svg, title, description}) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--left padding-horiz--md">
        <h3>{title}</h3>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures() {
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
