import React from 'react';
import clsx from 'clsx';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import { useUserData } from '@site/src/theme/Root';
import { useState, useEffect } from 'react';

const env = require('../../env');

// Fastspring loader cribbed from https://codesandbox.io/s/pwf02?file=/src/App.js.
// It turned out a bit hacky, but it seems to work reliably now.
// Note to future self - see private document "IT Infrastructure" for test purchase account details.

function CategoryCard({title, description, productId, items}) {
  const item = items ? items[productId] : null;
  const price = item ? item.price : "...";

  const buyProduct = () => {
    console.log("Buy button clicked for " + productId);
    console.log(item);
    window.fastspring.builder.push({
      products: [
        {
          path: item.path,
          quantity: 1
        }
      ],
      checkout: true
    });
  }

  return (
    <>
      <div className={clsx("col col--4")}>
        <div className="card">
          <div className="card__header">
            <h3>{title}</h3>
            <p>{description}<br/><span>{price}</span></p>
          </div>
          <div className="card__body">
            <button className="button button--primary margin-bottom--md" onClick={buyProduct}>
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

function BuyGoldPage({goldUser, devMode}) {
  const [items, setItems] = useState(null);
  const [country, setCountry] = useState("US");

  const storeFrontToUse = devMode ? "ppsspp.test.onfastspring.com/popup-gold" : "ppsspp.onfastspring.com/popup-gold";
  const fastSpringBuilderUrl = "https://d1f8f9xcsvx3ha.cloudfront.net/sbl/0.8.5/fastspring-builder.min.js";

  var debugLogging = devMode && true;

  useEffect(() => {
    console.log("In useEffect");

    const fastspringCallback = (data) => {
      console.log("fastspringCallback");
      /* Iterate over payload to find whatever information is relevant
        For this example we'll want to display localized price of
        the product 'demo-product-4'
        */
      var collectedItems = {}
      if (data && data.groups) {
        setCountry(data.country);
        data.groups.forEach((group) => {
          if (group.items && Array.isArray(group.items)) {
            group.items.forEach((item) => {
              // Update local state to add localized product information
              collectedItems[item.path] = item;
            });
          }
        });
        console.log(collectedItems);
        setItems(collectedItems);
      }
    };

    const fastspringPopupClosed = (orderReference) => {
      console.log("fastspringPopupClosed!");
      if (orderReference) {
        console.log("Order completed - redirecting!");
        console.log(orderReference)
        fastspring.builder.reset();
        var url = "/thankyou?orderId=" + orderReference.reference;
        try {window.location.replace(url);} catch(e) {window.location = url;}
      } else {
        console.log("no order ID - will not redirect to thankyou page");
      }
    }

    // Add SBL script programmatically
    const addSBL = () => {
      const scriptId = "fsc-api";
      const existingScript = document.getElementById(scriptId);
      if (existingScript) {
        // HACK!
        existingScript.parentNode.removeChild(existingScript);
      }

      // TODO: The !items check is needed but something is wrong suspicious, why does items get nulled on navigation?
      if (!existingScript || !items) {
        console.log("Loading script");

        const script = document.createElement("script");

        script.type = "text/javascript";
        script.id = scriptId;
        script.src = fastSpringBuilderUrl;
        script.dataset.storefront = storeFrontToUse;
        script.dataset.debug = debugLogging ? "true" : "false";
        // Make sure to add callback function to window so that the DOM is aware of it
        window.fastspringCallback = fastspringCallback;
        window.fastspringPopupClosed = fastspringPopupClosed;
        script.setAttribute("data-data-callback", "fastspringCallback");
        script.setAttribute("data-popup-closed", "fastspringPopupClosed");

        document.body.appendChild(script);
      } else {
        console.log("Existing script");
        console.log(items)
      }
    };
    addSBL();
  }, []);

  const vcenterStyle = { display: 'flex', alignItems: 'center', width: '..', height: '..'}

  var goldAlready = <>{goldUser}</>;
  if (goldUser) {
    goldAlready =
      <div className="row">
        <div className={clsx("col col--8")}>
          <p>You already have PPSSPP Gold for Windows! Of course, if you want to support the project a little extra, feel free!</p>
        </div>
      </div>;
  }

  return (
    <>
    <br/>
    <div className="container">
      <div className="row">
        <div className={clsx("col col--8")}>
          <h1 style={vcenterStyle}><img src="/img/platform/ppsspp-icon-gold.png"  width="64px" height="64px"/>&nbsp;PPSSPP Gold</h1>
        </div>
      </div>

      <h3>Buy PPSSPP Gold for Android</h3>
      <p>
        <Link className={clsx("btn btn-warning btn-large")} to="https://play.google.com/store/apps/details?id=org.ppsspp.ppssppgold">
          <img alt="PPSSPP Gold on Google Play" src="https://developer.android.com/images/brand/en_generic_rgb_wo_60.png" />
        </Link>
      </p>

      {devMode && <p>DEV MODE</p>}

      <h3>Buy PPSSPP Gold for Windows</h3>
      {goldAlready}

      <p>Choose your level:</p>

      <div className="row">
        <CategoryCard title="Hangaround" description="A show of support." productId="ppsspp-gold-l1" items={items}/>
        <CategoryCard title="Supporter" description="This will help!" productId="ppsspp-gold-l2" items={items}/>
        <CategoryCard title="Fan" description="A strong statement." productId="ppsspp-gold-l3" items={items}/>
      </div>

      <br/>

      <div className="row">
        <CategoryCard title="Gamer" description="A real contribution." productId="ppsspp-gold-l4" items={items}/>
        <CategoryCard title="Powergamer" description="A love of emulation." productId="ppsspp-gold-l5" items={items}/>
        <CategoryCard title="Grandmaster" description="You totally rock!" productId="ppsspp-gold-l6" items={items}/>
      </div>

      <br/>

      {!goldUser &&
      <p>Do you have PPSSPP Gold for Android already? <Link href="/requestgold">Request PPSSPP Gold for Windows here.</Link></p> }

      <hr/>

      <div className="row">
        <div className={clsx("col col--8")}>
          <h3>Links and information</h3>
          <ul>
          <li><Link to="/docs/reference/whygold">Why buy PPSSPP Gold? What's the difference?</Link></li>
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

  var devMode = env.mode !== "prod";

  return (
    <Layout
      title="Buy Gold"
      description="Buy PPSSPP Gold for PC">
      <BuyGoldPage goldUser={userData.goldUser} devMode={devMode} />
    </Layout>
  );
}
