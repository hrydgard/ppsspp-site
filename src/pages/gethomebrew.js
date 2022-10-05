import React from 'react';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import AdResponsive from '../components/AdSense';

// This is the landing page from the "How to get homebrew" button in PPSSPP.
// I feel it's better to have a custom page for this than to link directly into the guide.

export default function Home() {
    const {siteConfig} = useDocusaurusContext();
    return (
      <Layout
        title={`How to get homebrew and demos`}
        description="How to get homebrew and demos">
        <AdResponsive />
        <section>
          <div className="container">
            <div className="row">
              <div className="col col--12">
                <h1 style={{paddingTop: "10px"}}>Demos and homebrew for PPSSPP</h1>

                <h2>PSP Demos</h2>
                <p>Many game companies released downloadable demo versions of their games for the PSP.
                Some of these can run in PPSSPP. These files are legally free to download.
                Unfortunately the PSP Demo Center that used to host them is no longer online.</p>

                <p>There's a new site though that has quite a few: <a href="https://playdreamcreate.com/">PlayDreamCreate.</a></p>

                <h2>PSP Homebrew</h2>
                <p>The PSP was released in 2005, and contained a number of bugs that made it easy to run
                your own code on the device. A large community of hackers emerged who wrote their own software
                for the device. There's now a lot of little homemade games to run.</p>

                <h3>Where can I find them?</h3>
                <ul>
                    <li>PPSSPP has a built-in homebrew store which hosts a selection of them.</li>
                    <li><a href="">Homebrew library at Archive.org</a></li>
                    <li>Many other sites, use Google.</li>
                </ul>

                <h3>How do I get them on my device?</h3>
                <p>If you have manually downloaded homebrew, unzip it and put the game folder in PSP/GAME in your memstick directory.</p>

                <p><a href="/docs/getting-started/how-to-get-demos-and-homebrew">More details!</a></p>
              </div>
            </div>
          </div>
        </section>
      </Layout>
    );
  }
