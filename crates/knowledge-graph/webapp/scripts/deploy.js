import { cpSync, mkdirSync } from 'fs';
import { resolve } from 'path';
// The built bundle is deployed in two parts. The hashed assets are static content
// (resources/public/assets). The web app's entry page goes OUTSIDE the static folder,
// to resources/template/playground.html, where get.index.html serves it only when
// app.env=dev - so a production deployment never shows the Playground UI.
// resources/public/index.html stays the plain home page and is not touched.
const src = resolve(process.cwd(), 'dist');
const resources = resolve(process.cwd(), '../resources');
const assets = resolve(resources, 'public/assets');
const page = resolve(resources, 'template/playground.html');
console.log(`Deploying: ${src}/assets → ${assets}`);
mkdirSync(assets, { recursive: true });
cpSync(resolve(src, 'assets'), assets, { recursive: true });
console.log(`Deploying: ${src}/index.html → ${page}`);
cpSync(resolve(src, 'index.html'), page);
