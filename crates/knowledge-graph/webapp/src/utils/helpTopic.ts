/**
 * Extracts the bare topic key from a help command's text.
 *
 * The input is the command text WITHOUT the "> " echo prefix
 * (i.e. `event.commandText` from a `command.helpOrDescribe` event).
 *
 * Examples:
 *   "help create"    → "create"
 *   "help"           → ""        (root index)
 *   "HELP Tutorial 1"→ "tutorial 1"
 *
 * Returns the lowercased, trimmed topic key.
 */
export function extractHelpTopic(commandText: string): string {
  return commandText.replace(/^help\s*/i, '').trim().toLowerCase();
}
