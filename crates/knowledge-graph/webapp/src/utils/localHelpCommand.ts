import { extractHelpTopic } from './helpTopic';       // src/utils/helpTopic.ts
import { getHelpContent } from '../data/helpContent'; // src/data/helpContent.ts

/**
 * Resolves the bundled help topic key for a command, or null if the command
 * should be sent to the backend.
 *
 * @param commandText  The trimmed command string as typed by the user (no "> " prefix).
 * @param supportsHelp Whether the current playground has the help panel enabled.
 * @returns            The topic key ("" = root index, "create" = help create page),
 *                     or null when the command should go to the backend.
 */
export function resolveBundledHelpTopic(
  commandText: string,
  supportsHelp: boolean,
): string | null {
  if (!supportsHelp) return null;
  const lower = commandText.trim().toLowerCase();
  if (lower !== 'help' && !lower.startsWith('help ')) return null;
  const topic = extractHelpTopic(commandText);          // strips "help " prefix + lowercases
  return getHelpContent(topic) !== null ? topic : null; // null when not bundled
}
