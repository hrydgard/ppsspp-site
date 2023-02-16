import React, { useState } from 'react';
import clsx from 'clsx';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import { useUserData } from '@site/src/theme/Root';
import AdResponsive from '../components/AdSense';
import { compareVersions } from 'compare-versions';
import Link from '@docusaurus/Link';

// Data
import files from '../../static/downloads.json'
import goldFiles from '../../static/downloads_gold.json'

const urlBase = "https://www.ppsspp.org/files/";

function computeGoldUrlBase() {
  let {siteConfig} = useDocusaurusContext();
  return siteConfig.url + "/api/goldfiles/";
}

// Only safe-ish, but good enough.
function compareVersionsSafe(a, b) {
    if (a.indexOf(".") == -1 || b.indexOf(".") == -1) {
        return a.localeCompare(b);
    } else {
        return compareVersions(a, b);
    }
}

// Filenames are the same for every version, which really isn't great.
// Should find another way to organize these...
const platformList = [
  {
    title: 'Android',
    platform_badge: 'android-36.png',
    platform_key: 'android',
    downloads: [
      {
        name: "PPSSPP Gold for Android",
        icon: 'ppsspp-icon-gold.png',
        url: 'https://play.google.com/store/apps/details?id=org.ppsspp.ppssppgold',
        gold_color: true,
      },
      {
        name: "PPSSPP for Android (free)",
        icon: 'ppsspp-icon.png',
        url: 'https://play.google.com/store/apps/details?id=org.ppsspp.ppsspp'
      },
      {
        name: "Download APK of PPSSPP Free",
        icon: 'ppsspp-icon.png',
        filename: 'ppsspp.apk',
        short_name: 'APK',
      },
    ],
  },
  {
    title: 'Windows',
    platform_badge: 'windows-36-white.png',
    platform_key: 'windows',
    downloads: [
      {
        title: "PPSSPP Gold",
        name: "Buy PPSSPP Gold",
        icon: 'ppsspp-icon-gold.png',
        url: '/buygold',
        gold_color: true,
        login_prompt: true,
      },
      {
        name: "Download PPSSPP Gold (ZIP)",
        short_name: 'Gold (ZIP)',
        icon: 'ppsspp-icon-gold.png',
        filename: 'PPSSPPWindowsGold.zip',
        gold_file: true,
      },
      {
        name: "Download PPSSPP Gold (Installer)",
        short_name: 'Gold (installer)',
        icon: 'ppsspp-icon-gold.png',
        filename: 'PPSSPPGoldSetup.exe',
        gold_file: true,
      },
      {
        title: "PPSSPP Free",
        short_name: 'Installer',
        name: "Download PPSSPP installer",
        icon: 'ppsspp-icon.png',
        filename: 'PPSSPPSetup.exe',
      },
      {
        name: "Download ZIP file (portable)",
        short_name: 'ZIP',
        icon: 'ppsspp-icon.png',
        filename: 'ppsspp_win.zip',
      },
      {
        name: "Download ZIP file for ARM64",
        short_name: 'ZIP (ARM64)',
        icon: 'ppsspp-icon.png',
        filename: 'PPSSPPWindowsARM64.zip',
        whats_this: '/docs/faq#arm64win',
      }
    ]
  },
  {
    title: 'macOS',
    platform_badge: 'apple-36.png',
    platform_key: 'macos',
    downloads: [
      {
        name: "Download from Github CI",
        url: 'https://github.com/hrydgard/ppsspp/releases/latest',
      }
    ]
  },
  {
    title: 'Linux',
    platform_badge: 'linux-36.png',
    platform_key: 'linux',
    downloads: [
      {
        name: "PPSSPP Flatpak package",
        icon: 'ppsspp-icon.png',
        url: "https://flathub.org/apps/details/org.ppsspp.PPSSPP",
      }
    ]
  },
  {
    title: 'iOS',
    platform_badge: 'apple-36.png',
    platform_key: 'ios',
    description: "PPSSPP for iOS is not officially supported but working builds can be found.",
    downloads: [
      {
        name: "W.MS' latest .IPA",
        url: "https://pbot.w-ms.cn/getipa?PPSSPPORG",
      },
      {
        name: "W.MS' latest .deb",
        url: "https://pbot.w-ms.cn/getdeb?PPSSPPORG",
      },
    ]
  },
  {
    title: 'Standalone VR APKs',
    platform_key: 'vr',
    downloads: [
      {
        name: "PPSSPP VR for Quest",
        icon: 'ppsspp-icon.png',
        short_name: 'APK (Quest VR)',
        filename: 'ppsspp_vr_quest.apk'
      },
      {
        name: "PPSSPP VR for PICO",
        icon: 'ppsspp-icon.png',
        short_name: 'APK (Pico VR)',
        filename: 'ppsspp_vr_pico.apk'
      }
    ]
  }
];

// Takes a dictionary produced by dirtree-json, and makes something more sensible out of it.
// Once done we should probably just move this to dirtree-json.
function parseFiles(files, goldFiles) {
  var binariesPerVersion = files["children"].filter(child => "children" in child && child.name.indexOf("_") != -1).map(child => {
    return {
      "version": child.name.replaceAll("_", "."),
      "files": child.children?.map((subchild) => {
        return subchild.name;
      }),
    }
  });
  binariesPerVersion.sort((a, b) => compareVersionsSafe(b.version, a.version));

  if (goldFiles) {
    var binariesPerVersionGold = goldFiles["children"].filter(child => "children" in child && child.name.indexOf("_") != -1).map(child => {
      return {
        "version": child.name.replaceAll("_", "."),
        "files": child.children?.map((subchild) => {
          return subchild.name;
        }),
      }
    });

    for (var b of binariesPerVersion) {
      for (const g of binariesPerVersionGold) {
        if (g.version === b.version) {
          b.files = b.files.concat(g.files);
        }
      }
    }
  }

  // console.log(JSON.stringify(binariesPerVersion));

  return binariesPerVersion;
}

function pivot(binariesPerVersion) {
  // binariesPerVersion = binariesPerVersion.sort((a, b) => a.version < b.version);

  // Alright, now instead get versions per binaries.
  var versionsPerBinary = {};
  for (const entry of binariesPerVersion) {
    for (var filename of entry.files) {
      if (!(filename in versionsPerBinary)) {
        versionsPerBinary[filename] = [];
      }
      versionsPerBinary[filename].push({"filename": filename, "version": entry.version});
    }
  }
  return versionsPerBinary;
}

function determineLatestVersion(binariesPerVersion) {
    return binariesPerVersion[0].version;
}

const binariesPerVersion = parseFiles(files, goldFiles);
const parsedFiles = pivot(binariesPerVersion);
const latestVersion = determineLatestVersion(binariesPerVersion);

function GooglePlayBadge({ icon, appUrl, name, gold_color }) {
  return (
    <Link to={appUrl} className={clsx("button", "button--block", "margin-bottom--md", gold_color && "button--warning" || "button--primary")}>
      <div style={{ display: "flex", flexDirection: "row", alignItems: "center" }}>
        <img src={"/img/platform/" + icon} width="48px" />
        <span style={{ paddingLeft: "10px" }}>{name}</span>
      </div>
      <img alt='Get it on Google Play' src='https://play.google.com/intl/en_us/badges/static/images/badges/en_badge_web_generic.png' width="200" />
    </Link>
  )
}

function FlathubBadge({ icon, appUrl, name, gold_color }) {
  return (
    <Link to={appUrl} className={clsx("button", "button--block", "margin-bottom--md", gold_color && "button--warning" || "button--primary")}>
      <div style={{ display: "flex", flexDirection: "row", alignItems: "center" }}>
        <img src={"/img/platform/" + icon} width="48px" />
        <span style={{ paddingLeft: "10px" }}>{name}</span>
      </div>
      <img alt='Download on Flathub' src='https://flathub.org/assets/badges/flathub-badge-en.png' width='180' />
    </Link>
  )
}

function Download({url, text, icon, gold_color, whats_this}) {
  var icon_html = <></>;
  if (icon) {
    icon_html = <img src={"/img/platform/" + icon} width="32px" style={{marginRight: "10px"}} />;
  }

  var whats_this_link = <></>;
  if (whats_this) {
    whats_this_link = <p align="right" style={{marginTop: "0px"}}><a href={whats_this}>What's this?</a></p>;
  }

  return (
    <>
    <Link to={url} className={clsx("button", "button--block", "margin-bottom--md", gold_color && "button--warning" || "button--primary")}
        style={{display: "flex", flexDirection: "row", alignItems: "center"}}
        >{icon_html}{text}
    </Link>
    {whats_this_link}
    </>
  );
}

function DownloadsForFilename(props) {
  const {name, filename, url, title, icon, login_prompt, logged_in, gold_file, gold_color, showGold, whats_this} = props;

  if (gold_file && !showGold) {
    return <></>;
  }

  var logged_out_link = <></>;
  if (login_prompt && !logged_in) {
    logged_out_link = <p>Already bought? <Link to="/login?Forward=download">Log in</Link></p>;
  }

  var title_html = title ? <h3>{title}</h3> : <></>;

  if (url) {
    if (url.startsWith("https://play.google.com/")) {
      return <GooglePlayBadge icon={icon} name={name} appUrl={url} gold_color={gold_color}/>;
    } else if (url.startsWith("https://flathub.org/")) {
      return <FlathubBadge icon={icon} name={name} appUrl={url} gold_color={gold_color}/>;
    }
    var icon_html = icon ? <img src={"/img/platform/" + icon} width="32px" style={{marginRight: "10px"}} /> : <></>;
    return (
      <>
        {title_html}
        <Link to={url} className={clsx("button", "button--block", "margin-bottom--md", gold_color && "button--warning" || "button--primary")}
            style={{display: "flex", flexDirection: "row", alignItems: "center"}}>
            {icon_html}{name}</Link>
        {logged_out_link}
      </>);
  }

  if (!(filename in parsedFiles)) {
    return <div>ERROR: No downloads for {name}<br/>{filename}</div>
  }

  var versions = parsedFiles[filename];
  versions.sort((a, b) => compareVersions(b.version, a.version));

  var goldUrlBase = computeGoldUrlBase();

  const vs = versions.map((p, index) => {
    // TODO: Ugly hack, keep track somehow instead.
    var gold = p.filename.includes("Gold");
    var fileUrlBase = gold ? goldUrlBase : urlBase;
    return (<Download
      key={"Download_" + index}
      url={fileUrlBase + p.version.replaceAll('.', '_') + "/" + p.filename}
      text={name}
      icon={icon}
      gold_color={gold}
      whats_this={whats_this}
      />);
    }
  );

  return (
    <>
    {title_html}
    <div className="text--left ">
      {vs.slice(0, 1)}
    </div>
    {logged_out_link}
    </>
  );
}

function DownloadsForPlatform({title, platform_badge, downloads, logged_in, showGold}) {
  console.log(downloads);
  return (
    <div className={clsx('col col--4')}>
      <div className={clsx('card margin--sm shadow--tl')}>
        <div className="card__header">
          <h3 style={{display: "flex", flexDirection: "row", alignItems: "baseline" }}>
            {platform_badge ? <><img src={'/img/platform/' + platform_badge} width="18px" height="18px"/>&nbsp;</> : <></>}{title}
          </h3>
        </div>
        <div className="card__body">
          {downloads.map((props, idx) => <DownloadsForFilename key={idx} {...props} logged_in={logged_in} showGold={showGold} />)}
        </div>
      </div>
    </div>
  );
}

function PlatformDownload({href, short_name}) {
  return <><Link to={href}>{short_name}</Link><span>&nbsp;|&nbsp;</span></>;
}

function PlatformForVersion({version, platform, showGold, urlBase, goldUrlBase}) {
  return (
    <>{platform.platform_badge ? <><img src={"/img/platform/" + platform.platform_badge} width="18px" height="18px" />&nbsp;</> : <></>}
      {platform.downloads.filter((download) => { return showGold || !download.gold_file; }).map((download, idx) => {
        var fileUrlBase = download.gold_file ? goldUrlBase : urlBase;
        var href = fileUrlBase + version.replaceAll('.', '_') + "/" + download.filename;
        return <PlatformDownload key={"list" + href + download.short_name} href={href} short_name={download.short_name}/>;
      }
      )}</>);
}

// Produces a nice table row.
function DownloadsForVersion({version, files, showGold}) {
  var platforms = [];
  for (const platform of platformList) {
    var downloads = [];
    for (const download of platform.downloads) {
      if (files.includes(download.filename)) {
        downloads.push(download);
      }
    }
    if (downloads.length > 0) {
      platforms.push({
        downloads: downloads,
        platform_badge: platform.platform_badge,
        platform_key: platform.platform_key,
      });
    }
  }

  var goldUrlBase = computeGoldUrlBase();

  if (platforms.length > 0) {
    return (
      <tr>
        <td>{version}</td>
        <td style={{ display: "flex", flexDirection: "row", alignItems: "center" }}>
          {platforms.map((platform, idx) => <PlatformForVersion key={version + '_' + platform.platform_key} version={version} platform={platform} showGold={showGold} urlBase={urlBase} goldUrlBase={goldUrlBase}/>)}
        </td>
      </tr>);
  } else {
    return <></>;
  }
}

function PreviousReleases({showGold}) {
  const [expanded, setExpanded] = useState(false);

  const maxCount = expanded ? 100 : 0;
  var nVersions = binariesPerVersion.slice(0, maxCount);
  return (
    <>
      {maxCount > 0 &&
      <table>
        <thead>
          <tr><th>Version</th><th>Downloads</th></tr>
        </thead>
        <tbody>
          {nVersions.map((props, idx) => (
            <DownloadsForVersion key={"prev_" + props.version} {...props} showGold={showGold}/>
          ))}
        </tbody>
      </table>
      }
      {!expanded && <button className="button button--primary" onClick={() => setExpanded(true)}>Show list</button>}
    </>
  );
}

function DownloadPage({userData}) {
  var goldBanner = <></>;
  var showGold = false;
  if (userData.loggedIn && userData.goldUser) {
    goldBanner = <div className="alert alert--warning" role="alert">You've got PPSSPP Gold for Windows! Downloads are now available below.</div>;
    showGold = true;
  }

  return (
    <>
    <section className="downloadList">
      <div className="container">
        <div className="row">
          <div className={clsx("col col--12")}>
            <h2>Download PPSSPP {latestVersion}</h2>
            <p>
              What's new in {latestVersion}? <Link to="/news">See the news</Link>!
              The source code can be downloaded from <Link to="https://github.com/hrydgard/ppsspp">Github</Link>.
            </p>
            {goldBanner}
          </div>
        </div>
        <div className="row">
          {platformList.map((props, idx) => (
            <DownloadsForPlatform key={idx} {...props} logged_in={userData.loggedIn} showGold={showGold} />
          ))}
        </div>
      </div>
    </section>
    <section>
      <div className="container">
        <div className="row">
          <div className={clsx("col col--12")}>
            {userData.goldUser || <p><Link to="/docs/reference/whygold">Why buy the Gold version?</Link></p>}
            {userData.goldUser && <p><Link to="/requestgold">You have Gold - get PPSSPP Gold for Android for free!</Link></p>}
            <br/>
            <h2>Development builds</h2>
            <p>Download fresh development builds from <Link to="https://buildbot.orphis.net/ppsspp/index.php">Orphis' Buildbot</Link>, also included below.
              <br/>Please note that these development builds can be unstable - if one doesn't work, try an earlier one. And backup your save games!
            </p>
            <h2>Legacy builds and other downloads</h2>
            <p><Link to="/legacybuilds">Page where you can download</Link> cube.elf, and builds for Switch, Blackberry, Meego and other esoteric or old platforms.</p>
            <br/>
            <h2>Previous releases</h2>
            <PreviousReleases showGold={showGold}/>
          </div>
        </div>
      </div>
    </section>
    <br/>
    <a name="devbuilds"></a>
    <Link to="https://buildbot.orphis.net/ppsspp/index.php?m=fulllist">All builds</Link>
    <iframe seamless={true} src="https://buildbot.orphis.net/ppsspp/" style={{borderWidth: "0px", backgroundColor: "transparent", padding: "0px", overflow: "hidden", width: "100%", height: "2000px"}}/>
    </>
  );
}

export default function Home() {
  const {siteConfig} = useDocusaurusContext();
  const {userData, setUserData} = useUserData();

  return (
    <Layout
      title={`Downloads`}
      description="Downloads for PC, Android, Mac, Linux, iPhone, iPad, iOS">
      <AdResponsive />
      <DownloadPage userData={userData} />
    </Layout>
  );
}
