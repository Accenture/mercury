/**
 * Command grammar for MiniGraph Playground autocomplete.
 *
 * Each entry describes a command template:
 *  - `tokens`   — the fixed keyword tokens that make up this suggestion,
 *                 exactly as they should appear in the input.
 *  - `template` — the full text inserted when the suggestion is accepted.
 *                 Placeholders use {curly-brace} notation for user-supplied values.
 *  - `hint`     — a short description shown next to the suggestion.
 *  - `multiline`— true when accepting the suggestion should also enable multiline mode.
 *
 * Aliases handled by the backend (getWords() in GraphCommandService):
 *   start  → instantiate
 *   clear  → delete
 * Both the canonical form and the alias are listed so either triggers autocomplete.
 */

export interface CommandSuggestion {
  /** The ordered keywords that identify this suggestion (used for matching). */
  tokens: string[];
  /** Text inserted into the command input when the suggestion is accepted. */
  template: string;
  /** Short description displayed in the dropdown. */
  hint: string;
  /** When true the CommandInput multiline toggle should be activated. */
  multiline?: boolean;
}

// ---------------------------------------------------------------------------
// Built-in skill routes — shown as completions after "describe skill "
// ---------------------------------------------------------------------------
export const BUILTIN_SKILLS: string[] = [
  'graph.data.mapper',
  'graph.math',
  'graph.js',
  'graph.api.fetcher',
  'graph.extension',
  'graph.island',
  'graph.join',
];

// ---------------------------------------------------------------------------
// Quickstart reference — shown in the info-icon popover on the Command label.
// Each entry is a top-level keyword and a short description of what it does.
// ---------------------------------------------------------------------------
export interface QuickstartEntry {
  keyword:     string;
  alias?:      string;   // backend alias, shown as "(alias: <x>)"
  description: string;
  /** Text inserted into the command input when the palette row is clicked. */
  template:    string;
  /** When true, the command spans multiple lines (e.g. create / update / instantiate). */
  multiline?:  boolean;
}

export const COMMAND_QUICKSTART: QuickstartEntry[] = [
  { keyword: 'help',            description: 'List all help topics, or get help for a specific command',
    template: 'help' },
  { keyword: 'create',          description: 'Create a new graph node',
    template: 'create node {name}\nwith type {type}\nwith properties\n{key}={value}', multiline: true },
  { keyword: 'update',          description: 'Update an existing node',
    template: 'update node {name}\nwith type {type}\nwith properties\n{key}={value}', multiline: true },
  { keyword: 'edit',            description: 'Print raw node data ready for editing and re-submitting',
    template: 'edit node {name}' },
  { keyword: 'delete node',       description: 'Delete a node by name', alias: 'clear node',
    template: 'delete node {name}' },
  { keyword: 'delete connection', description: 'Delete connection(s) between two nodes', alias: 'clear connection',
    template: 'delete connection {nodeA} and {nodeB}' },
  { keyword: 'delete cache',      description: 'Clear cached API fetcher results', alias: 'clear cache',
    template: 'delete cache' },
  { keyword: 'connect',           description: 'Connect two nodes with a named relation',
    template: 'connect {node-A} to {node-B} with {relation}' },
  { keyword: 'list nodes',        description: 'List all nodes in the current graph',
    template: 'list nodes' },
  { keyword: 'list connections',  description: 'List all connections in the current graph',
    template: 'list connections' },
  { keyword: 'describe graph',    description: 'Describe the current graph model',
    template: 'describe graph' },
  { keyword: 'describe node',     description: 'Describe a specific node and its connections',
    template: 'describe node {name}' },
  { keyword: 'describe connection', description: 'Describe connection(s) between two nodes',
    template: 'describe connection {nodeA} and {nodeB}' },
  { keyword: 'describe skill',    description: 'Show documentation for a skill by route name',
    template: 'describe skill {skill.route}' },
  { keyword: 'export',            description: 'Export the graph model to a JSON file',
    template: 'export graph as {name}' },
  { keyword: 'import graph',      description: 'Import a graph model from a saved file',
    template: 'import graph from {name}' },
  { keyword: 'import node',       description: 'Import a single node from another saved graph',
    template: 'import node {node-name} from {graph-name}' },
  { keyword: 'instantiate',     description: 'Create a runnable graph instance with mock input', alias: 'start',
    template: 'instantiate graph\n{constant} -> input.body.{key}', multiline: true },
  { keyword: 'upload mock data', description: 'Print the URL to POST a JSON payload as mock input.body',
    template: 'upload mock data' },
  { keyword: 'execute',         description: 'Execute a single node skill in isolation',
    template: 'execute node {name}' },
  { keyword: 'inspect',         description: 'Inspect a state-machine variable',
    template: 'inspect {variable_name}' },
  { keyword: 'run',             description: 'Run the graph instance from root to end',
    template: 'run' },
];

// ---------------------------------------------------------------------------
// Full command grammar
// ---------------------------------------------------------------------------
export const COMMAND_SUGGESTIONS: CommandSuggestion[] = [

  // ── help ────────────────────────────────────────────────────────────────
  {
    tokens:   ['help'],
    template: 'help',
    hint:     'List all help topics',
  },
  {
    tokens:   ['help', 'create'],
    template: 'help create',
    hint:     'Help: create node',
  },
  {
    tokens:   ['help', 'update'],
    template: 'help update',
    hint:     'Help: update node',
  },
  {
    tokens:   ['help', 'edit'],
    template: 'help edit',
    hint:     'Help: edit (print raw node for re-submitting)',
  },
  {
    tokens:   ['help', 'delete'],
    template: 'help delete',
    hint:     'Help: delete node, connection or clear cache',
  },
  {
    tokens:   ['help', 'connect'],
    template: 'help connect',
    hint:     'Help: connect two nodes',
  },
  {
    tokens:   ['help', 'list'],
    template: 'help list',
    hint:     'Help: list nodes or connections',
  },
  {
    tokens:   ['help', 'describe'],
    template: 'help describe',
    hint:     'Help: describe graph / node / connection / skill',
  },
  {
    tokens:   ['help', 'export'],
    template: 'help export',
    hint:     'Help: export graph as a JSON file',
  },
  {
    tokens:   ['help', 'import'],
    template: 'help import',
    hint:     'Help: import graph or node from a file',
  },
  {
    tokens:   ['help', 'data-dictionary'],
    template: 'help data-dictionary',
    hint:     'Help: data dictionary and provider node syntax',
  },
  {
    tokens:   ['help', 'instantiate'],
    template: 'help instantiate',
    hint:     'Help: instantiate a graph instance (alias: start)',
  },
  {
    tokens:   ['help', 'execute'],
    template: 'help execute',
    hint:     'Help: execute a node skill in isolation',
  },
  {
    tokens:   ['help', 'inspect'],
    template: 'help inspect',
    hint:     'Help: inspect state-machine variables',
  },
  {
    tokens:   ['help', 'run'],
    template: 'help run',
    hint:     'Help: run a graph instance end-to-end',
  },

  // ── create ──────────────────────────────────────────────────────────────
  {
    tokens:   ['create', 'node'],
    template: 'create node {name}\nwith type {type}\nwith properties\n{key}={value}',
    hint:     'Create a new graph node (multi-line)',
    multiline: true,
  },

  // ── update ──────────────────────────────────────────────────────────────
  {
    tokens:   ['update', 'node'],
    template: 'update node {name}\nwith type {type}\nwith properties\n{key}={value}',
    hint:     'Update an existing node (multi-line)',
    multiline: true,
  },

  // ── edit ────────────────────────────────────────────────────────────────
  {
    tokens:   ['edit', 'node'],
    template: 'edit node {name}',
    hint:     'Print raw node data ready to copy-edit and re-submit',
  },

  // ── delete / clear (alias) ───────────────────────────────────────────────
  {
    tokens:   ['delete', 'node'],
    template: 'delete node {name}',
    hint:     'Delete a node by name',
  },
  {
    tokens:   ['delete', 'connection'],
    template: 'delete connection {nodeA} and {nodeB}',
    hint:     'Delete connection(s) between two nodes',
  },
  // "clear" is a backend alias for "delete"
  {
    tokens:   ['clear', 'node'],
    template: 'clear node {name}',
    hint:     'Delete a node by name (alias for: delete node)',
  },
  {
    tokens:   ['clear', 'connection'],
    template: 'clear connection {nodeA} and {nodeB}',
    hint:     'Delete connection(s) between two nodes (alias for: delete connection)',
  },
  {
    tokens:   ['clear', 'cache'],
    template: 'clear cache',
    hint:     'Clear cached API fetcher results',
  },

  // ── connect ─────────────────────────────────────────────────────────────
  {
    tokens:   ['connect'],
    template: 'connect {node-A} to {node-B} with {relation}',
    hint:     'Connect two nodes with a relation',
  },

  // ── list ────────────────────────────────────────────────────────────────
  {
    tokens:   ['list', 'nodes'],
    template: 'list nodes',
    hint:     'List all nodes in the current graph',
  },
  {
    tokens:   ['list', 'connections'],
    template: 'list connections',
    hint:     'List all connections in the current graph',
  },

  // ── describe ────────────────────────────────────────────────────────────
  {
    tokens:   ['describe', 'graph'],
    template: 'describe graph',
    hint:     'Describe the current graph model',
  },
  {
    tokens:   ['describe', 'node'],
    template: 'describe node {name}',
    hint:     'Describe a specific node',
  },
  {
    tokens:   ['describe', 'connection'],
    template: 'describe connection {node-A} and {node-B}',
    hint:     'Describe connection(s) between two nodes',
  },
  {
    tokens:   ['describe', 'skill'],
    template: 'describe skill {skill.route.name}',
    hint:     'Show documentation for a skill',
  },
  // Built-in skill shortcuts for "describe skill <route>"
  ...BUILTIN_SKILLS.map(skill => ({
    tokens:   ['describe', 'skill', skill],
    template: `describe skill ${skill}`,
    hint:     `Describe built-in skill: ${skill}`,
  })),

  // ── export ──────────────────────────────────────────────────────────────
  {
    tokens:   ['export', 'graph', 'as'],
    template: 'export graph as {name}',
    hint:     'Export the graph model to a named JSON file',
  },

  // ── import ──────────────────────────────────────────────────────────────
  {
    tokens:   ['import', 'graph', 'from'],
    template: 'import graph from {name}',
    hint:     'Import a graph model from a named JSON file',
  },
  {
    tokens:   ['import', 'node'],
    template: 'import node {node-name} from {graph-name}',
    hint:     'Import a single node from another graph model',
  },

  // ── instantiate / start (alias) ──────────────────────────────────────────
  {
    tokens:   ['instantiate', 'graph'],
    template: 'instantiate graph\n{constant} -> input.body.{key}',
    hint:     'Create a graph instance with mock input (multi-line)',
    multiline: true,
  },
  // "start" is a backend alias for "instantiate"
  {
    tokens:   ['start', 'graph'],
    template: 'start graph\n{constant} -> input.body.{key}',
    hint:     'Create a graph instance with mock input (alias for: instantiate graph)',
    multiline: true,
  },

  // ── execute ─────────────────────────────────────────────────────────────
  {
    tokens:   ['execute', 'node'],
    template: 'execute node {name}',
    hint:     'Execute the skill of a single node in isolation',
  },
  {
    tokens:   ['execute'],
    template: 'execute {node-name}',
    hint:     'Execute a node skill — short form (no "node" keyword)',
  },

  // ── inspect ─────────────────────────────────────────────────────────────
  {
    tokens:   ['inspect'],
    template: 'inspect {variable_name}',
    hint:     'Inspect a state-machine variable',
  },

  // ── run ─────────────────────────────────────────────────────────────────
  {
    tokens:   ['run'],
    template: 'run',
    hint:     'Run the graph instance from root to end',
  },
];

// ---------------------------------------------------------------------------
// Matching logic
// ---------------------------------------------------------------------------

/**
 * Given the current single-line command text, return the subset of
 * COMMAND_SUGGESTIONS that are relevant completions.
 *
 * Matching rules (all case-insensitive):
 *  1. Split the input on whitespace.
 *  2. A suggestion matches if every input token is a prefix of the
 *     corresponding suggestion token at the same position — AND the input
 *     has fewer tokens than (or equal tokens to) the suggestion.
 *  3. Exact full matches (input === template first line) are excluded so the
 *     dropdown disappears once the user has already completed the command.
 */
export function getSuggestions(rawInput: string): CommandSuggestion[] {
  // Only operate on the first line — multi-line mode has its own flow.
  const firstLine = rawInput.split('\n')[0];
  // trimStart (not trim) so a trailing space like "describe " still matches
  // sub-commands (the user has finished typing the first token).
  const trimmed = firstLine.trimStart();
  if (trimmed === '') return [];

  const inputTokens = trimmed.toLowerCase().split(/\s+/);

  return COMMAND_SUGGESTIONS.filter(suggestion => {
    const { tokens, template } = suggestion;

    // Don't suggest if the input is already the completed template first line.
    if (trimmed.toLowerCase() === template.split('\n')[0].toLowerCase()) return false;

    // The suggestion must have at least as many tokens as the input.
    if (tokens.length < inputTokens.length) return false;

    // Every input token must be a prefix of the corresponding suggestion token.
    return inputTokens.every((inputTok, i) =>
      tokens[i]?.startsWith(inputTok)
    );
  });
}
