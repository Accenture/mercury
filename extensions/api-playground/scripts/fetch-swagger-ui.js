#!/usr/bin/env node
/*
 * Fetch the Swagger UI distribution into resources/public/swagger-ui.
 *
 * The API playground's page (resources/public/index.html, ours) loads the Swagger UI assets
 * from the swagger-ui subfolder. Those assets are third-party code that goes stale quickly, so they
 * are not committed (the subfolder is in .gitignore). Run this script before `cargo run -p api-playground`:
 *
 *     node scripts/fetch-swagger-ui.js                 # the latest GitHub release
 *     node scripts/fetch-swagger-ui.js --version v5.33.1
 *     node scripts/fetch-swagger-ui.js --target /some/other/folder
 *
 * What it does: resolves the latest release tag of github.com/swagger-api/swagger-ui (unless
 * --version names one), downloads that tag's source tarball from codeload.github.com, extracts the
 * files of its `dist` folder into the target folder - all but upstream's own page (`index.html`,
 * `index.css`, `swagger-initializer.js`), which this application replaces - and writes
 * `swagger-ui-version.txt` recording what was fetched. The target folder is emptied
 * first; as a guard against a mistyped --target, a non-empty folder is emptied only when it holds a
 * previous fetch (the version file or swagger-ui-bundle.js).
 *
 * Node.js 18+, no dependencies (global fetch, zlib, a minimal tar reader). The Python twin is
 * scripts/fetch-swagger-ui.py.
 */

'use strict';

const fs = require('fs');
const path = require('path');
const zlib = require('zlib');

const REPO = 'swagger-api/swagger-ui';
const LATEST_RELEASE_URL = `https://api.github.com/repos/${REPO}/releases/latest`;
const TARBALL_URL = (tag) => `https://codeload.github.com/${REPO}/tar.gz/refs/tags/${tag}`;
const USER_AGENT = 'mercury-api-playground-fetch-swagger-ui';
const DIST_MEMBER = /^[^/]+\/dist\/([^/]+)$/;
const UPSTREAM_PAGE = new Set(['index.html', 'index.css', 'swagger-initializer.js']);
const VERSION_FILE = 'swagger-ui-version.txt';
const MARKERS = [VERSION_FILE, 'swagger-ui-bundle.js'];
const DEFAULT_TARGET = path.resolve(__dirname, '..', 'resources', 'public', 'swagger-ui');

function fail(message) {
  console.error(message);
  process.exit(1);
}

function parseArgs(argv) {
  const args = { version: 'latest', target: DEFAULT_TARGET };
  for (let i = 0; i < argv.length; i++) {
    const arg = argv[i];
    if (arg === '--version' && argv[i + 1]) {
      args.version = argv[++i];
    } else if (arg === '--target' && argv[i + 1]) {
      args.target = path.resolve(argv[++i]);
    } else if (arg === '-h' || arg === '--help') {
      console.log('Usage: node scripts/fetch-swagger-ui.js [--version vX.Y.Z] [--target DIR]');
      process.exit(0);
    } else {
      fail(`Unknown argument '${arg}' - use --version vX.Y.Z and/or --target DIR`);
    }
  }
  return args;
}

async function fetchBytes(url, accept) {
  const response = await fetch(url, { headers: { 'User-Agent': USER_AGENT, Accept: accept } });
  if (!response.ok) {
    const error = new Error(`${response.status} from ${url}`);
    error.status = response.status;
    throw error;
  }
  return Buffer.from(await response.arrayBuffer());
}

async function latestTag() {
  let release;
  try {
    release = JSON.parse((await fetchBytes(LATEST_RELEASE_URL, 'application/vnd.github+json')).toString('utf8'));
  } catch (e) {
    fail(`Cannot resolve the latest release (${e.message}) - name one with --version vX.Y.Z ` +
      `(see https://github.com/${REPO}/releases)`);
  }
  if (typeof release.tag_name !== 'string' || !release.tag_name) {
    fail('The GitHub API answered without a tag_name - name one with --version vX.Y.Z');
  }
  return release.tag_name;
}

// A minimal ustar/pax reader: 512-byte headers, the entry's data padded to 512, two zero blocks at
// the end. GitHub's tarballs come from `git archive`, which writes a pax global header ('g') with the
// commit id and may write a pax extended header ('x') carrying a long `path` - both are honoured.
function* tarEntries(tar) {
  let offset = 0;
  let paxPath = null;
  while (offset + 512 <= tar.length) {
    const header = tar.subarray(offset, offset + 512);
    if (header.every((byte) => byte === 0)) {
      break;
    }
    const field = (start, length) => header.subarray(start, start + length).toString('utf8').replace(/\0.*$/s, '');
    const size = parseInt(field(124, 12).trim() || '0', 8);
    const type = field(156, 1);
    const prefix = field(345, 155);
    let name = prefix ? `${prefix}/${field(0, 100)}` : field(0, 100);
    const data = tar.subarray(offset + 512, offset + 512 + size);
    offset += 512 + Math.ceil(size / 512) * 512;
    if (type === 'x') {
      // pax extended header: "<len> path=<value>\n" records; keep the path for the next entry
      for (const record of data.toString('utf8').split('\n')) {
        const match = /^\d+ path=(.*)$/.exec(record);
        if (match) {
          paxPath = match[1];
        }
      }
      continue;
    }
    if (type === 'g') {
      continue;
    }
    if (paxPath !== null) {
      name = paxPath;
      paxPath = null;
    }
    yield { name, type, data };
  }
}

function distFiles(tarball) {
  const files = new Map();
  for (const entry of tarEntries(zlib.gunzipSync(tarball))) {
    const match = DIST_MEMBER.exec(entry.name);
    if (match && (entry.type === '0' || entry.type === '') && !UPSTREAM_PAGE.has(match[1])) {
      files.set(match[1], Buffer.from(entry.data));
    }
  }
  if (files.size === 0) {
    fail('The tarball holds no dist folder - the upstream layout may have changed');
  }
  return files;
}

function prepareTarget(target) {
  if (fs.existsSync(target)) {
    if (!fs.statSync(target).isDirectory()) {
      fail(`${target} is not a folder`);
    }
    const entries = fs.readdirSync(target);
    if (entries.length > 0 && !MARKERS.some((marker) => fs.existsSync(path.join(target, marker)))) {
      fail(`${target} is not empty and holds no previous fetch - refusing to empty it`);
    }
    for (const entry of entries) {
      fs.rmSync(path.join(target, entry), { recursive: true, force: true });
    }
  }
  fs.mkdirSync(target, { recursive: true });
}


async function main() {
  const args = parseArgs(process.argv.slice(2));
  const tag = args.version === 'latest' ? await latestTag() : args.version;
  if (!/^v?\d+\.\d+\.\d+(-[0-9A-Za-z.]+)?$/.test(tag)) {
    fail(`'${tag}' does not look like a release tag (expected vX.Y.Z)`);
  }
  const url = TARBALL_URL(tag);
  console.log(`Fetching Swagger UI ${tag} from ${url}`);
  let tarball;
  try {
    tarball = await fetchBytes(url, 'application/octet-stream');
  } catch (e) {
    fail(`Download failed (${e.message}) - is ${tag} a release tag of ${REPO}?`);
  }

  const files = distFiles(tarball);
  prepareTarget(args.target);
  let total = 0;
  for (const name of [...files.keys()].sort()) {
    const content = files.get(name);
    fs.writeFileSync(path.join(args.target, name), content);
    total += content.length;
  }
  const stamp = new Date().toISOString().replace(/\.\d{3}Z$/, 'Z');
  const record = `swagger-ui ${tag}\nfetched ${stamp}\nfrom ${url}\n`;
  fs.writeFileSync(path.join(args.target, VERSION_FILE), record, 'utf8');

  console.log(`Wrote ${files.size} files (${total.toLocaleString('en-US')} bytes) to ${args.target}`);
  console.log('Next: cargo run -p api-playground');
}

main().catch((e) => fail(e.stack || String(e)));
