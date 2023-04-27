all:
	cd types; yarn; yarn build;
	cd sdk; yarn add ../types; yarn; yarn build;
	cd executor; yarn add ../sdk; yarn add ../types; yarn;