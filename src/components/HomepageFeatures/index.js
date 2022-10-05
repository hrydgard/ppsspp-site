import React from 'react';
import clsx from 'clsx';
import styles from './styles.module.css';

// Gold is a smaller list, faster to parse, and enough to compute the newest version.
// We should probably hardcode it somewhere, generate it offline...
import goldFiles from '../../../static/downloads_gold.json'



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
