#!/usr/bin/env node
/*
    Copyright 2018-2026 Accenture Technology

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
 */
/**
 * Deploy a graph set to a running MiniGraph engine (RFC-0017), for an engine that has no Playground:
 * the production path, where the authoring UI is off.
 *
 *   node scripts/deploy-graph-set.mjs --engine https://gateway.example.com/engine --file settlement.pack \
 *        --sha256 9f2c... --header "authorization: Bearer $DEPLOY_TOKEN"
 *   node scripts/deploy-graph-set.mjs --file settlement.pack --digest
 *
 * The package is posted as application/octet-stream to POST /api/graph-set/deploy under the engine's
 * base URL (or the gateway's path in front of it); the engine reads it strictly, requires the set to be
 * named after its one main graph, refuses a graph id a manifest or another set owns, gates every graph
 * and registers the set all or none. The answer is printed as the engine gives it, one JSON document.
 *
 * Flags
 *   --engine <url>     the engine's base URL, or the gateway's path in front of it (the entry's path is appended)
 *   --file <path>      the graph set, <set>.pack
 *   --sha256 <hex>     the digest the field's ledger holds, sent as X-Graph-Set-SHA256; the engine refuses other bytes
 *   --header 'n: v'    a transport header, repeatable - a gateway's token, taken from the shell's environment
 *   --digest           print the package's SHA-256 and exit: the ledger step, without an upload
 *   --list             print the deployed sets the engine records and exit
 *
 * Exit codes: 0 on success; 1 on a refusal, with the engine's status and reasons on stderr; 2 on a usage error.
 * No prompt: a script in a pipeline runs unattended. Zero dependencies; Node.js 26 LTS (minimum 22).
 */
import { readFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import { parseArgs } from 'node:util';

const DEPLOY_PATH = '/api/graph-set/deploy';
const DEPLOYED_PATH = '/api/graph-set/deployed';
const DIGEST_HEADER = 'X-Graph-Set-SHA256';

function usage(message) {
  if (message) console.error(`error: ${message}`);
  console.error('usage: node scripts/deploy-graph-set.mjs --engine <url> --file <set>.pack [--sha256 <hex>]'
    + " [--header 'name: value']... | --file <set>.pack --digest | --engine <url> --list");
  process.exit(2);
}

/** The entry's URL under the base: the base's own path is kept, so a gateway's prefix reaches the engine. */
export function entryUrl(engine, path) {
  return engine.replace(/\/+$/, '') + path;
}

/** Transport headers from repeated --header 'name: value' flags. */
export function headersOf(entries) {
  const headers = {};
  for (const entry of entries ?? []) {
    const colon = entry.indexOf(':');
    if (colon < 1) usage(`--header '${entry}' is not 'name: value'`);
    headers[entry.slice(0, colon).trim()] = entry.slice(colon + 1).trim();
  }
  return headers;
}

export function sha256Hex(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

async function answer(response) {
  const text = await response.text();
  try {
    return JSON.parse(text);
  } catch {
    return { status: response.status, message: text };
  }
}

/** Print the engine's answer; a refusal goes to stderr with its status and reasons, and the exit code says so. */
async function report(response) {
  const body = await answer(response);
  if (response.ok) {
    console.log(JSON.stringify(body, null, 2));
    return 0;
  }
  const reason = typeof body.message === 'string' ? body.message : JSON.stringify(body);
  const hint = response.status === 503 ? ' (hot deployment is off on this engine - graph.hot.deploy / graph.hot.undeploy)'
    : response.status === 404 && !reason.includes('set ') ? ' (no such entry - an engine older than hot deployment, or a wrong base URL)' : '';
  console.error(`refused (${response.status}): ${reason}${hint}`);
  return 1;
}

async function main() {
  const { values } = parseArgs({
    options: {
      engine: { type: 'string' },
      file: { type: 'string' },
      sha256: { type: 'string' },
      header: { type: 'string', multiple: true },
      digest: { type: 'boolean', default: false },
      list: { type: 'boolean', default: false },
      help: { type: 'boolean', default: false },
    },
    strict: true,
  });
  if (values.help) usage();
  const headers = headersOf(values.header);
  if (values.list) {
    if (!values.engine) usage('--list needs --engine');
    const response = await fetch(entryUrl(values.engine, DEPLOYED_PATH), { headers: { ...headers, accept: 'application/json' } });
    return report(response);
  }
  if (!values.file) usage('--file names the graph set to deploy');
  const bytes = await readFile(values.file);
  const digest = sha256Hex(bytes);
  if (values.digest) {
    console.log(digest);
    return 0;
  }
  if (!values.engine) usage('--engine names the engine or the gateway in front of it');
  if (values.sha256 && values.sha256.trim().toLowerCase() !== digest) {
    console.error(`refused before upload: the file's SHA-256 is ${digest}, not ${values.sha256.trim()}`);
    return 1;
  }
  const response = await fetch(entryUrl(values.engine, DEPLOY_PATH), {
    method: 'POST',
    headers: {
      ...headers,
      'content-type': 'application/octet-stream',
      accept: 'application/json',
      ...(values.sha256 ? { [DIGEST_HEADER]: values.sha256.trim() } : {}),
    },
    body: bytes,
  });
  return report(response);
}

if (import.meta.url === `file://${process.argv[1]}`) {
  main().then(code => process.exit(code), err => {
    const cause = err?.cause?.code ? ` (${err.cause.code})` : '';
    console.error(`failed: ${err.message}${cause}`);
    process.exit(1);
  });
}
