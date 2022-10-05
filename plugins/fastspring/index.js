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
const env = require('../../env');

// TODO: Replace this plugin with https://github.com/facebook/docusaurus/pull/8151 .
module.exports = function (context, opts) {
  const { siteConfig } = context;

  var devMode = env.mode !== "prod";
  var debugLogging = devMode && false;

  return {
    name: 'fastspring',

    injectHtmlTags({content}) {
      return {
        headTags: [
          {
            tagName: 'script',
            attributes: {
              defer: true,
              id: 'fsc-api',
              src: 'https://d1f8f9xcsvx3ha.cloudfront.net/sbl/0.7.4/fastspring-builder.min.js',
              type: 'text/javascript',
              "data-storefront": devMode ? "ppsspp.test.onfastspring.com/popup-gold" : "ppsspp.onfastspring.com/popup-gold",
              "data-popup-closed": "onFSPopupClosed",
              "data-debug": debugLogging ? "true" : "false",
            },
          },
          {
            // This one can't be async.
            tagName: 'script',
            innerHTML: 'function onFSPopupClosed(orderReference) {\n\
               if (orderReference) {\n\
                 console.log("Order completed - redirecting!");\n\
                 fastspring.builder.reset();\n\
                 var url = "/thankyou?orderId=" + orderReference.reference;\n\
                 try {window.location.replace(url);} catch(e) {window.location = url;}\n\
               } else {\n\
                 console.log("no order ID - will not redirect to thankyou page");\n\
               }\n\
            }'
           },
         ],
      };
    },
  };
};