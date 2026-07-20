import { rmSync, mkdirSync } from 'fs';
import { resolve } from 'path';
const target = resolve(process.cwd(), '../resources/public');
console.log(`Cleaning: ${target}`);
rmSync(target, { recursive: true, force: true });
mkdirSync(target, { recursive: true });
