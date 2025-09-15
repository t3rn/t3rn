/**
 * Creating a sidebar enables you to:
 - create an ordered group of docs
 - render a sidebar for each doc of that group
 - provide next/previous navigation

 The sidebars can be generated from the filesystem, or explicitly defined here.

 Create as many sidebars as you want.
 */

// @ts-check

/** @type {import('@docusaurus/plugin-content-docs').SidebarsConfig} */

export default {
  sidebar: [
    {
      type: "category",
      label: "Welcome",
      link: {
        type: "doc",
        id: "intro",
      },
      collapsed: false,
      items: [
        "protocol-architecture",
        "connect-to-t3rn",
        "tokenomics",
      ],
    },
    {
      type: "category",
      label: "Executor",
      link: {
        type: "doc",
        id: "executors",
      },
      collapsed: false,
      items: [
        "executor/executor-breakdown",
        "executor/become-an-executor/docker-setup",
        "executor/become-an-executor/binary-setup",
        "executor/become-an-executor/aixecutor-setup",
      ],
    },
    {
      type: "category",
      label: "Resources",
      link: {
        type: "doc",
        id: "resources",
      },
      collapsed: false,
      items: [
        "resources/block-explorer",
        "resources/faucet",
        "resources/supported-chains",
        "resources/executor-privacy",
        "resources/gas-cost-breakdown",
        "resources/t3rn-glossary",
        "resources/order-status",
        "resources/order-flow",
        "resources/security",
        "resources/troubleshooting",
      ],
    },
  ],
};