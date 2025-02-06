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
      type: "html",
      value: "<span style='font-size: 20px; font-weight: bold;'>Welcome</span>",
      defaultStyle: true,
    },
    {
      type: "doc",
      id: "intro",
    },
    {
      type: "doc",
      id: "protocol-architecture",
    },
    // {
    //   type: "doc",
    //   id: "tokenomics",
    // },
    {
      type: "html",
      value:
        "<span style='font-size: 20px; font-weight: bold;'>Executor</span>",
      defaultStyle: true,
    },
    { type: "doc", id: "executor/executor-overview" },
    { type: "doc", id: "executor/executor-breakdown" },
    { type: "doc", id: "executor/become-an-executor/binary-setup" },
    { type: "doc", id: "executor/become-an-executor/gui-setup" },
    {
      type: "html",
      value:
        "<span style='font-size: 20px; font-weight: bold;'>Resources</span>",
      defaultStyle: true,
    },
    { type: "doc", id: "resources/block-explorer" },
    { type: "doc", id: "resources/faucet" },
    { type: "doc", id: "resources/supported-chains" },
    { type: "doc", id: "resources/executor-privacy" },
    { type: "doc", id: "resources/gas-cost-breakdown" },
    { type: "doc", id: "resources/t3rn-glossary" },
    { type: "doc", id: "resources/order-status" },
    { type: "doc", id: "resources/order-flow" },
    { type: "doc", id: "resources/security" },
    { type: "doc", id: "resources/troubleshooting" },
  ],
};
