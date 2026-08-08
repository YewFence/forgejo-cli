import { defineConfig } from 'vitepress'

export default defineConfig({
  base: '/forgejo-cli/',
  lang: 'en-US',
  title: 'Forgejo CLI',
  description: 'A command-line interface for Forgejo',

  themeConfig: {
    nav: [
      { text: 'Guide', link: '/guide/getting-started' },
      { text: 'GitHub', link: 'https://github.com/YewFence/forgejo-cli' }
    ],

    sidebar: [
      {
        text: 'Guide',
        items: [
          { text: 'Getting Started', link: '/guide/getting-started' }
        ]
      }
    ],

    search: {
      provider: 'local'
    },

    socialLinks: [
      { icon: 'github', link: 'https://github.com/YewFence/forgejo-cli' }
    ],

    footer: {
      message: 'Released under the Apache-2.0 or MIT license.',
      copyright: 'Copyright © YewFence'
    },

    docFooter: {
      prev: 'Previous page',
      next: 'Next page'
    },

    outline: {
      label: 'On this page'
    },

    lastUpdated: {
      text: 'Last updated'
    }
  }
})
