/**
 * Central configuration for all playground tools.
 *
 * ─── HOW TO ADD A NEW PLAYGROUND ───────────────────────────────────────────
 *  1. Add an entry to PLAYGROUND_CONFIGS below.
 *  That's it — the route, navigation bar link, and connection dot are all
 *  derived automatically from this array. No changes to App.tsx needed.
 * ───────────────────────────────────────────────────────────────────────────
 */

// RightTab is defined in RightPanel but the config needs it to declare which
// tabs each playground shows.  We import only the type so there is no runtime
// dependency from config → component.
import type { RightTab } from '../components/RightPanel/RightPanel';

// ---------------------------------------------------------------------------
// Shared runtime limits (used by both the hook and the UI)
// ---------------------------------------------------------------------------

/** Maximum number of console messages kept in memory at once. */
export const MAX_ITEMS    = 200;

/** Maximum number of command history entries kept in localStorage. */
export const MAX_HISTORY  = 50;

/** Maximum number of history-based autocomplete suggestions shown in the dropup. */
export const MAX_AUTOCOMPLETE_SUGGESTIONS = 8;

/** Maximum payload size in characters accepted by the WebSocket send path. */
export const MAX_BUFFER = 63_488; // 62 * 1024 - some overhead for WebSocket framing, to stay safely under the 64KB limit of most browsers

/** Interval in milliseconds between keep-alive ping frames sent to the server. */
export const PING_INTERVAL = 20_000;

/** Shape of a single playground tool entry. */
export interface PlaygroundConfig {
  path:                  string;     // URL route path, must start with "/"
  label:                 string;     // Short name shown in the navigation bar
  title:                 string;     // Full heading shown on the playground page
  wsPath:                string;     // WebSocket endpoint path served by the backend
  storageKeyPayload:     string;     // localStorage key for the last payload
  storageKeyHistory:     string;     // localStorage key for command history
  storageKeyTab:         string;     // localStorage key for the last selected right-panel tab
  storageKeySavedGraphs?: string;   // localStorage key for graphs saved from the Graph Data tab (omit to hide the feature)
  supportsUpload?:       boolean;    // true when the backend supports POST /api/json/content/{id} upload
  supportsClipboard?:    boolean;    // true when clip/paste features are enabled (Minigraph only)
  supportsHelp?:         boolean;    // true when the help panel is available (Minigraph only)
  supportsAuthoring?:    boolean;    // true when UI graph authoring is enabled (Minigraph only)
  /**
   * localStorage key for the last-viewed help topic.
   * Only used when the playground includes the 'help' tab.
   * Omit to use a static fallback key (not recommended for production configs).
   */
  storageKeyHelpTopic?:  string;
  /**
   * The ordered list of right-panel tabs to show for this playground.
   * The first entry is the default selected tab on first load.
   * Tabs absent from this list are not rendered at all.
   *
   * Tab meanings:
   *  'payload'    — JSON/XML payload editor (used by playgrounds that send a data body)
   *  'graph'      — ReactFlow graph visualisation
   *  'graph-data' — Raw JSON viewer for the fetched graph model
   */
  tabs:              RightTab[];
}

export const PLAYGROUND_CONFIGS: PlaygroundConfig[] = [
  {
    path: '/json-path',
    label: 'JSON-Path',
    title: 'JSON-Path Playground',
    wsPath: '/ws/json/path',
    storageKeyPayload: 'jsonpath-last-payload',
    storageKeyHistory: 'jsonpath-command-history',
    storageKeyTab: 'jsonpath-right-tab',
    supportsUpload: true,
    // JSON-Path needs a payload input and graph views.
    tabs: ['payload', 'graph', 'graph-data'],
  },
  {
    path: '/',
    label: 'Minigraph',
    title: 'Minigraph Playground',
    wsPath: '/ws/graph/playground',
    storageKeyPayload: 'minigraph-last-payload',
    storageKeyHistory: 'minigraph-command-history',
    storageKeyTab: 'minigraph-right-tab',
    storageKeySavedGraphs: 'minigraph-saved-graphs',
    storageKeyHelpTopic: 'minigraph-help-topic',
    supportsClipboard: true,
    supportsHelp: true,
    supportsAuthoring: true,
    // Minigraph works with graph commands and text responses; it has no payload input.
    tabs: ['graph', 'graph-data'],
  },
];

/**
 * Quick-load sample payloads shown below the payload textarea.
 *
 * Keys follow the pattern "<type>_<label>".
 *   - The type prefix ("json" or "xml") groups the buttons by row.
 *   - The label part is the button text (underscores become spaces).
 *
 * Add a new entry here to make a new button appear automatically in the UI.
 */
export const SAMPLE_DATA: Record<string, string> = {
  json_simple: JSON.stringify({ name: 'John Doe', age: 30, city: 'New York' }, null, 2),
  json_nested: JSON.stringify(
    {
      user: {
        name: 'Jane Smith',
        profile: {
          email: 'jane@example.com',
          address: { city: 'San Francisco', country: 'USA' },
        },
      },
    },
    null,
    2
  ),
  json_array: JSON.stringify(
    [
      { id: 1, name: 'Item 1', status: 'active' },
      { id: 2, name: 'Item 2', status: 'pending' },
      { id: 3, name: 'Item 3', status: 'inactive' },
    ],
    null,
    2
  ),
  xml_simple: `<?xml version="1.0" encoding="UTF-8"?>
<person>
  <name>John Doe</name>
  <age>30</age>
  <city>New York</city>
</person>`,
  xml_nested: `<?xml version="1.0" encoding="UTF-8"?>
<user>
  <name>Jane Smith</name>
  <profile>
    <email>jane@example.com</email>
    <address>
      <city>San Francisco</city>
      <country>USA</country>
    </address>
  </profile>
</user>`,
  xml_array: `<?xml version="1.0" encoding="UTF-8"?>
<items>
  <item>
    <id>1</id>
    <name>Item 1</name>
    <status>active</status>
  </item>
  <item>
    <id>2</id>
    <name>Item 2</name>
    <status>pending</status>
  </item>
  <item>
    <id>3</id>
    <name>Item 3</name>
    <status>inactive</status>
  </item>
</items>`,
};
