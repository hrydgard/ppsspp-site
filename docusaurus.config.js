// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const env = require('./env.js');

const lightCodeTheme = require('prism-react-renderer/themes/github');
const darkCodeTheme = require('prism-react-renderer/themes/dracula');

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: 'PPSSPP',
  tagline: 'A PSP emulator',
  url: env.url,
  baseUrl: '/',
  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'throw',  //was: warn
  favicon: 'img/favicon.ico',

  // Even if you don't use internalization, you can use this field to set useful
  // metadata like html lang. For example, if your site is Chinese, you may want
  // to replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  presets: [
    [
      '@docusaurus/preset-classic',
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          sidebarPath: require.resolve('./sidebars.js'),
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl:
            'https://github.com/hrydgard/ppsspp-site/tree/main/',
        },
        blog: {
          showReadingTime: true,
          blogSidebarCount: 'ALL',
          postsPerPage: 'ALL',
        },
        theme: {
          customCss: require.resolve('./src/css/custom.css'),
        },
        sitemap: {
          changefreq: 'weekly',
          priority: 0.5,
          ignorePatterns: ['/tags/**'],
          filename: 'sitemap.xml',
        },
        googleTagManager: {
          trackingID: env.analytics,
        },
      }),
    ],
  ],

  plugins: [
    './plugins/google-adsense',
    './plugins/fastspring',  // TODO: This is only needed on /buygold, can we filter it out?

    [
      '@docusaurus/plugin-content-blog',
      {
        // Required for any multi-instance plugin
        id: 'news-blog',
        // URL route for the blog section of your site. *DO NOT* include a trailing slash.
        routeBasePath: 'news',
        // Path to data on filesystem relative to site dir.
        path: './news',

        blogSidebarCount: 'ALL',
        postsPerPage: 'ALL',
      },
    ],

    './plugins/webpack-proxy',
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    {
      metadata: [{name: 'keywords', content: 'ppsspp, psp, emulator'}],
      colorMode: {
        defaultMode: 'dark',
      },

      /*
      announcementBar: {
        id: 'site_update',
        content: 'The website is currently being updated to a new design - please forgive any broken functionality, just try again soon',
        backgroundColor: "var(--ifm-color-primary)",
        textColor: "var(--ifm-color-primary-lightest)",
        isCloseable: false,
      },
      */

      navbar: {
        title: 'PPSSPP',
        logo: {
          alt: 'PPSSPP Logo',
          src: 'img/logo.svg',
        },
        items: [
          {to: '/download', label: 'Downloads', position: "left"},
          {to: '/news', label: 'News', position: 'left'},
          {to: '/blog', label: 'Blog', position: 'left'},
          {
            type: 'doc',
            docId: 'intro',
            position: 'left',
            label: 'Docs & Help',
          },
          {to: '/media', label: 'Media', position: "left"},
          {to: '/contact', label: 'Contact', position: "left"},
          {
            href: 'https://forums.ppsspp.org/',
            label: 'Forums',
            position: "left"
          },
          {
            href: 'https://github.com/hrydgard/ppsspp',
            label: 'GitHub',
            position: 'left',
          },
        ],
      },
      footer: {
        style: 'dark',
        links: [
          {
            title: 'Docs',
            items: [
              {
                label: 'Docs & Help',
                to: '/docs/intro',
              },
              {
                label: 'Getting started',
                to: '/docs/category/getting-started',
              }
            ],
          },
          {
            title: 'Community',
            items: [
              {
                label: 'Discord',
                href: 'https://discord.gg/5NJB6dD',
              },
              {
                label: 'Twitter',
                href: 'https://twitter.com/ppsspp_emu',
              },
            ],
          },
          {
            title: 'More',
            items: [
              {
                label: 'GitHub',
                href: 'https://github.com/hrydgard/ppsspp',
              },
              {
                label: 'Website Github',
                href: 'https://github.com/hrydgard/ppsspp-site',
              },
              {
                label: 'Login',
                to: '/login',
              }
            ],
          },
        ],
        copyright: `Copyright © ${new Date().getFullYear()} PPSSPP Project. <a href="/privacy">Privacy Policy</a>`,
      },
      prism: {
        theme: lightCodeTheme,
        darkTheme: darkCodeTheme,
        additionalLanguages: ['cpp', 'ini', 'glsl'],
      },
      googleAdsense: env.adsense ? {
        dataAdClient: env.adsense,
      } : null,
    },
};

module.exports = config;
