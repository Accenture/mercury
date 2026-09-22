import { rmSync } from 'fs';
import { resolve } from 'path';
// Removes exactly what deploy.js writes - the hashed assets and the Playground entry
// page. resources/public/index.html (the plain home page) is kept.
const resources = resolve(process.cwd(), '../resources');
for (const target of [resolve(resources, 'public/assets'), resolve(resources, 'template/playground.html')]) {
  console.log(`Cleaning: ${target}`);
  rmSync(target, { recursive: true, force: true });
}
