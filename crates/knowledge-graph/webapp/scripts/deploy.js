import { cpSync } from 'fs';
import { resolve } from 'path';
const src = resolve(process.cwd(), 'dist');
const dest = resolve(process.cwd(), '../resources/public');
console.log(`Deploying: ${src} → ${dest}`);
cpSync(src, dest, { recursive: true });
