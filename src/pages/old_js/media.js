import React from 'react';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import AdResponsive from '../components/AdSense';
import YoutubeEmbed from '../components/YoutubeEmbed';
import ScreenshotGallery from '@site/src/components/Media';

export default function Home() {
    const {siteConfig} = useDocusaurusContext();
    return (
      <Layout
        title={`Media`}
        description="Media">
        <AdResponsive />
        <section>
          <div className="container">
            <h2 style={{paddingTop: "10px"}}>Screenshots</h2>
            <ScreenshotGallery />
            <h2 style={{paddingTop: "10px"}}>Videos</h2>
            <YoutubeEmbed title="PPSSPP Original Trailer" embedId="10_i9cQdQA0"/>
            <YoutubeEmbed title="PPSSPP VR Trailer" embedId="y3dgEeDW5Xw"/>
          </div>
        </section>
      </Layout>
    );
  }
