import React from 'react';
import clsx from 'clsx';
import styles from './styles.module.css';

const PlatformList = [
  {
    title: 'Windows',
    filename: "PPSSPPWindows.zip",
  },
  {
    title: 'Android',
    filename: "ppsspp.apk",
  },
];

function DownloadsForPlatform({Svg, title, filename}) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center">
      </div>
      <div className="text--center padding-horiz--md">
        <h3>{title}</h3>
        <p>{filename}</p>
      </div>
    </div>
  );
}

export default function DownloadList() {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {PlatformList.map((props, idx) => (
            <DownloadsForPlatform key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
