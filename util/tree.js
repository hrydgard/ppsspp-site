// Prerequisites:
// node install directory-tree
// How to run: node tree.js

const dirTree = require('directory-tree');

const tree = dirTree(process.argv[2]);

console.log(JSON.stringify(tree, null, space=2));


