const esbuild = require("esbuild");

esbuild.build({
  entryPoints: ["src/index.ts"],
  outdir: "dist",
  bundle: true,
  sourcemap: true,
  minify: false,
  format: "cjs",
  target: ["es2020"],
});
