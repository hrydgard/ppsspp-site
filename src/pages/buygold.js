import React from 'react';
import clsx from 'clsx';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import { useUserData } from '@site/src/theme/Root';

function CategoryCard({title, description, productId}) {
  return (
    <>
      <div className={clsx("col col--4")}>
        <div className="card">
          <div className="card__header">
            <h3>{title}</h3>
            <p>{description}<br/><span data-fsc-item-path={productId} data-fsc-item-pricetotal></span></p>
          </div>
          <div className="card__body">
            <button className="button button--primary margin-bottom--md"
              data-fsc-item-path={productId}
              data-fsc-item-path-value={productId}
              data-fsc-action="Reset,Add,Checkout">
              <div style={{ display: "flex", flexDirection: "row", alignItems: "center" }}>
                <img src="/img/platform/ppsspp-icon-gold.png" width="48px" />
                <span style={{ paddingLeft: "10px" }}>Buy PPSSPP Gold</span>
              </div>
            </button>
          </div>
        </div>
      </div>
    </>
  )
}

function BuyGoldPage({goldUser}) {
    const vcenterStyle = { display: 'flex', alignItems: 'center', width: '..', height: '..'}

    var goldAlready = <>{goldUser}</>;
    if (goldUser) {
      goldAlready =
        <div className="row">
          <div className={clsx("col col--8")}>
            <p>You already have PPSSPP Gold for Windows!</p>
            <p>Of course, if you want to support the project a little extra, feel free!</p>
          </div>
        </div>;
    }

    var androidGold = <>
      <h3>Buy PPSSPP Gold for Android</h3>
      <p>
        <a className={clsx("btn btn-warning btn-large")} href="https://play.google.com/store/apps/details?id=org.ppsspp.ppssppgold">
          <img alt="PPSSPP Gold on Google Play" src="https://developer.android.com/images/brand/en_generic_rgb_wo_60.png" />
        </a>
      </p>

    </>;
    return (
        <>
        <br/>
        <div className="container">
          <div className="row">
            <div className={clsx("col col--8")}>
              <h1 style={vcenterStyle}><img src="/img/platform/ppsspp-icon-gold.png"  width="64px" height="64px"/>&nbsp;PPSSPP Gold</h1>
            </div>
          </div>

          {androidGold}
          <hr/>

          <h3>Buy PPSSPP Gold for Windows</h3>
          {goldAlready}

          <p>Choose your level:</p>

          <div className="row">
            <CategoryCard title="Hangaround" description="A show of support." productId="ppsspp-gold-l1"/>
            <CategoryCard title="Supporter" description="This will help!" productId="ppsspp-gold-l2"/>
            <CategoryCard title="Fan" description="A strong statement." productId="ppsspp-gold-l3"/>
          </div>

          <br/>

          <div className="row">
            <CategoryCard title="Gamer" description="A real contribution." productId="ppsspp-gold-l4"/>
            <CategoryCard title="Powergamer" description="A love of emulation." productId="ppsspp-gold-l5"/>
            <CategoryCard title="Grandmaster" description="You totally rock!" productId="ppsspp-gold-l6"/>
          </div>

          <br/>

          <div className="row">
            <div className={clsx("col col--8")}>
              <h3>Links and information</h3>
              <ul>
              <li><Link to="/docs/reference/whygold">Why buy PPSSPP Gold? What's the difference?</Link></li>
              <li><Link to="https://play.google.com/store/apps/details?id=org.ppsspp.ppssppgold">PPSSPP Gold for Android</Link></li>
              <li><Link to="/docs/reference/goldplatforms">PPSSPP Gold for other platforms</Link></li>
              </ul>
            </div>
          </div>
        </div>

        </>
    )
}

export default function Home() {
  const {siteConfig} = useDocusaurusContext();
  const {userData, setUserData} = useUserData();

  return (
    <Layout
      title={`${siteConfig.title} - Buy Gold`}
      description="Buy PPSSPP Gold for PC">
      <BuyGoldPage goldUser={userData.goldUser} />
    </Layout>
  );
}
