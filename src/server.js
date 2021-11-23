const { build } = require("esbuild");
const chokidar = require("chokidar");
const liveServer = require("live-server");

const nodenv = JSON.stringify(process.env.NODE_ENV) || "development";
const isDev = nodenv === "development";
const isProd = nodenv === "production";

let didError = false;

build({
  define: {
    "process.env.NODE_ENV": nodenv,
  },
  entryPoints: ["src/index.ts"],
  outfile: "dist/script.js",
  logLevel: 'warning',
  bundle: true,
  incremental: true,
  minify: isProd,
  sourcemap: isDev,
  // need to figure out what I'm building for
  // target: ['chrome58', 'firefox57', 'safari11', 'edge16'],
}).then((builder) => {
console.log('built and running on port 3000')
  chokidar.watch("src/**/*.ts").on("all", () => {
    builder.rebuild()
      .then(() => {
        if (didError) {

          console.log('✔️ built')
      didError = false;
        }
      })
      .catch((e) => {
        console.error(e.message)
      didError = true;

    });
  });
})
  .catch(() => {
  console.log('❌ failed on first build, fix the error and then retry')
  process.exit(1);
});

liveServer.start({
  open: false,
  port: 3000,
  root: "dist",
  logLevel: 0, // errors only
});
