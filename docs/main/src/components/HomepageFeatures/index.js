import React from 'react';
import clsx from 'clsx';
import styles from './styles.module.css';
import Link from '@docusaurus/Link';

const FeatureList = [
  {
    title: 't3rn Concepts',
    Svg: require('@site/static/img/undraw_docusaurus_mountain.svg').default,
    description: (
      <>
        A deep dive into the concepts that power t3rn
      </>
    ),
    link: "/concepts/intro",
    button: "Concepts"
  },
  {
    title: 'Getting started',
    Svg: require('@site/static/img/undraw_docusaurus_tree.svg').default,
    description: (
      <>
        Want to jump right in and prefer code examples? 
      </>
    ),
    link: "/docs/intro",
    button: "Getting Started"
  },
  {
    title: 'Typescript Client Library',
    Svg: require('@site/static/img/undraw_docusaurus_react.svg').default,
    description: (
      <>
        Checkout the TS package documentation.
      </>
    ),
    link: "http://google.com",
    button: "TS Docs"
  },
];

function Feature({Svg, title, description, link, button}) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center">
        <Svg className={styles.featureSvg} role="img" />
      </div>
      <div className="text--center padding-horiz--md">
        <h3>{title}</h3>
        <p>{description}</p>
      </div>
      <div className={styles.buttons}>
          <Link
            className="button button--secondary button--lg"
            to={link}>
              {button}
          </Link>
        </div>
    </div>
  );
}

export default function HomepageFeatures() {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
