// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const lightCodeTheme = require('prism-react-renderer/themes/github');
const darkCodeTheme = require('prism-react-renderer/themes/dracula');

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: 'PPSSPP',
  tagline: 'PSP emulator',
  url: 'https://www.ppsspp.org',
  baseUrl: '/',
  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',
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
      'classic',
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          sidebarPath: require.resolve('./sidebars.js'),
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl:
            'https://github.com/hrydgard/ppsspp-site/',
        },
        blog: {
          showReadingTime: true,
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl:
            'https://github.com/hrydgard/ppsspp-site/',
        },
        theme: {
          customCss: require.resolve('./src/css/custom.css'),
        },
      }),
    ],
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      colorMode: {
        defaultMode: 'dark',
      },
      navbar: {
        title: 'PPSSPP',
        logo: {
          alt: 'PPSSPP Logo',
          src: 'img/logo.svg',
        },
        items: [
          {to: '/download', label: 'Downloads', position: "left"},
          {to: '/media', label: 'Screenshots and video', position: "left"},
          {
            type: 'doc',
            docId: 'intro',
            position: 'left',
            label: 'Guides & Help',
          },
          {to: '/blog', label: 'Blog', position: 'left'},
          {
            href: 'https://github.com/hrydgard/ppsspp',
            label: 'GitHub',
            position: 'right',
          },
          {to: 'https://forums.ppsspp.org/', label: 'Forums', position: "left"},
          {to: '/contact', label: 'Contact', position: "left"},
        ],
      },
      footer: {
        style: 'dark',
        links: [
          {
            title: 'Docs',
            items: [
              {
                label: 'Tutorial',
                to: '/docs/intro',
              },
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
                label: 'Blog',
                to: '/blog',
              },
              {
                label: 'GitHub',
                href: 'https://github.com/hrydgard/ppsspp',
              },
            ],
          },
        ],
        copyright: `Copyright © ${new Date().getFullYear()} PPSSPP Project. Built with Docusaurus. <a href="/privacy">Privacy Policy</a>`,
      },
      prism: {
        theme: lightCodeTheme,
        darkTheme: darkCodeTheme,
      },
    }),
};

module.exports = config;
