const rust = import("../pkg/index.js");

rust.then((mod) => {
  mod.start();
});
