export const GRAPH_RUN_COMMANDS = {
  instantiate: 'instantiate graph',
  requestInputUpload: 'upload mock data',
  run: 'run',
} as const;

export interface GraphInstanceCreatedResult {
  mockEntries: number;
  ttlMs: number;
}

export interface GraphRunTerminalResult {
  status: 'completed' | 'aborted';
  elapsedMs: number | null;
}

const INSTANCE_CREATED_RE = /^Graph instance created\. Loaded (\d+) mock (?:entry|entries), model\.ttl = (\d+) ms$/;
const RUN_COMPLETED_RE = /^Graph traversal completed in (\d+) ms$/;
const COMMAND_ERROR_RE = /^ERROR:\s*(.+)$/;

export function parseGraphInstanceCreated(raw: string): GraphInstanceCreatedResult | null {
  const match = raw.match(INSTANCE_CREATED_RE);
  if (!match) return null;
  return {
    mockEntries: Number.parseInt(match[1], 10),
    ttlMs: Number.parseInt(match[2], 10),
  };
}

export function isGraphInstanceCleared(raw: string): boolean {
  return raw === 'Graph instance cleared';
}

export function parseGraphRunTerminal(raw: string): GraphRunTerminalResult | null {
  const completed = raw.match(RUN_COMPLETED_RE);
  if (completed) {
    return { status: 'completed', elapsedMs: Number.parseInt(completed[1], 10) };
  }
  if (raw === 'Graph traversal aborted') {
    return { status: 'aborted', elapsedMs: null };
  }
  return null;
}

export function parseCommandError(raw: string): string | null {
  const match = raw.match(COMMAND_ERROR_RE);
  return match?.[1]?.trim() || null;
}

export function isInstantiateCommandText(commandText: string): boolean {
  const normalized = commandText.trim().toLowerCase();
  return normalized === GRAPH_RUN_COMMANDS.instantiate || normalized === `${GRAPH_RUN_COMMANDS.instantiate}...`;
}

export function isRunCommandText(commandText: string): boolean {
  return commandText.trim().toLowerCase() === GRAPH_RUN_COMMANDS.run;
}
