/**
* MIT License
*
* Copyright (c) 2020 Hyeonki Hong <hhk7734@gmail.com>
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in
* all copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/

const path = require('path');

// TODO: Replace this component with https://github.com/facebook/docusaurus/pull/8151 .

module.exports = function (context, opts) {
  const { siteConfig } = context;
  const { themeConfig } = siteConfig;
  const { googleAdsense } = themeConfig || {};

  if (!googleAdsense) {
    // console.log("Ignoring google adsense - no pub id");
    return {};
    /*
    throw new Error(
      'You need to specify `googleAdsense` object in `themeConfig` with `dataAdClient` field in it to use google-adsense'
    );
    */
  }

  const { dataAdClient } = googleAdsense;

  if (!dataAdClient) {
    throw new Error(
      'You specified the `googleAdsense` object in `themeConfig` but the `dataAdClient` field was missing. ' +
      'Please ensure this is not a mistake.'
    );
  }

  return {
    name: 'google-adsense',

    injectHtmlTags({content}) {
      return {
        headTags: [
          {
            tagName: 'script',
            attributes: {
              async: true,
              src: `https://pagead2.googlesyndication.com/pagead/js/adsbygoogle.js?client=${dataAdClient}`,
              crossorigin: 'anonymous',
            },
          },
          {
            tagName: 'script',
            innerHTML: `(adsbygoogle = window.adsbygoogle || []).push({google_ad_client: "${dataAdClient}", enable_page_level_ads: true});`
          },
        ],
      };
    },
  };
};