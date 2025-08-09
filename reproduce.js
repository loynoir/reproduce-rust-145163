const debug = require('./build/debug.node');
const release = require('./build/release.node');

console.log("=== Calling debug reproduce ===")
debug.reproduce()
console.log()

console.log("=== Calling release reproduce ===")
try { release.reproduce(); } catch (e) { console.error(e); }
console.log()

console.log("=== Calling release workaround ===")
release.workaround();
console.log()
