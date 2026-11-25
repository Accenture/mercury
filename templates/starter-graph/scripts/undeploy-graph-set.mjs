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
 * Undeploy a graph set from a running MiniGraph engine (RFC-0017), by the set's name - which is its
 * main graph's id - or list what the engine has deployed.
 *
 *   node scripts/undeploy-graph-set.mjs --engine https://gateway.example.com/engine --graph settlement \
 *        --header "authorization: Bearer $DEPLOY_TOKEN"
 *   node scripts/undeploy-graph-set.mjs --engine https://gateway.example.com/engine --list
 *
 * POST /api/graph-set/undeploy/{graph-id}: the set's graphs leave the registry in one swap, the record
 * is deleted first and then its files, and every other instance over the same folder drops the set at its
 * next lookup. A set the folder does not hold answers 404 - a bundled graph too, since it never has a record.
 *
 * Flags
 *   --engine <url>     the engine's base URL, or the gateway's path in front of it (the entry's path is appended)
 *   --graph <id>       the main graph's id, which is the set's name
 *   --header 'n: v'    a transport header, repeatable - a gateway's token, taken from the shell's environment
 *   --list             print the deployed sets the engine records and exit
 *
 * Exit codes: 0 on success; 1 on a refusal, with the engine's status and reasons on stderr; 2 on a usage error.
 * No prompt: a script in a pipeline runs unattended. Zero dependencies; Node.js 26 LTS (minimum 22).
 */
import { parseArgs } from 'node:util';

const DEPLOYED_PATH = '/api/graph-set/deployed';

function usage(message) {
  if (message) console.error(`error: ${message}`);
  console.error("usage: node scripts/undeploy-graph-set.mjs --engine <url> --graph <id> [--header 'name: value']..."
    + ' | --engine <url> --list');
  process.exit(2);
}

export function entryUrl(engine, path) {
  return engine.replace(/\/+$/, '') + path;
}

export function undeployPath(graphId) {
  return `/api/graph-set/undeploy/${encodeURIComponent(graphId)}`;
}

export function headersOf(entries) {
  const headers = {};
  for (const entry of entries ?? []) {
    const colon = entry.indexOf(':');
    if (colon < 1) usage(`--header '${entry}' is not 'name: value'`);
    headers[entry.slice(0, colon).trim()] = entry.slice(colon + 1).trim();
  }
  return headers;
}

async function report(response) {
  const text = await response.text();
  let body;
  try {
    body = JSON.parse(text);
  } catch {
    body = { status: response.status, message: text };
  }
  if (response.ok) {
    console.log(JSON.stringify(body, null, 2));
    return 0;
  }
  const reason = typeof body.message === 'string' ? body.message : JSON.stringify(body);
  const hint = response.status === 503 ? ' (hot undeploy is off on this engine - graph.hot.undeploy)' : '';
  console.error(`refused (${response.status}): ${reason}${hint}`);
  return 1;
}

async function main() {
  const { values } = parseArgs({
    options: {
      engine: { type: 'string' },
      graph: { type: 'string' },
      header: { type: 'string', multiple: true },
      list: { type: 'boolean', default: false },
      help: { type: 'boolean', default: false },
    },
    strict: true,
  });
  if (values.help) usage();
  if (!values.engine) usage('--engine names the engine or the gateway in front of it');
  const headers = { ...headersOf(values.header), accept: 'application/json' };
  if (values.list) {
    return report(await fetch(entryUrl(values.engine, DEPLOYED_PATH), { headers }));
  }
  if (!values.graph || !/^[A-Za-z0-9_-]+$/.test(values.graph)) {
    usage("--graph names the set's main graph: letters, digits, '_' and '-' only");
  }
  return report(await fetch(entryUrl(values.engine, undeployPath(values.graph)), { method: 'POST', headers }));
}

if (import.meta.url === `file://${process.argv[1]}`) {
  main().then(code => process.exit(code), err => {
    const cause = err?.cause?.code ? ` (${err.cause.code})` : '';
    console.error(`failed: ${err.message}${cause}`);
    process.exit(1);
  });
}
