import { useEffect, useRef } from 'react';
import { type ProtocolBus } from '../protocol/bus';
import { getHelpContent } from '../data/helpContent';
import { extractHelpTopic } from '../utils/helpTopic';

export interface UseAutoHelpNavigateOptions {
  bus:          ProtocolBus;
  /** Called with the extracted topic key (empty string = root index). */
  setHelpTopic: (topic: string) => void;
  /** Called to open the help panel (a separate third resizable panel). */
  onTabSwitch:  () => void;
}

/**
 * Subscribes to `command.helpOrDescribe` events on the ProtocolBus.
 * When the command is a `help` command (not a `describe`), extracts the topic
 * key and opens the help panel — but only when the topic has locally bundled
 * content.  For non-bundled topics the server response shown in the console is
 * the only copy, so the panel is left closed.
 *
 * `describe` commands are handled separately by graph-link and other event flows.
 */
export function useAutoHelpNavigate({
  bus,
  setHelpTopic,
  onTabSwitch,
}: UseAutoHelpNavigateOptions): void {
  // Use a ref for onTabSwitch to avoid re-subscribing on every render if the
  // caller passes an inline arrow function.
  const onTabSwitchRef = useRef(onTabSwitch);
  useEffect(() => { onTabSwitchRef.current = onTabSwitch; });

  useEffect(() => {
    return bus.on('command.helpOrDescribe', (event) => {
      const lower = event.commandText.trim().toLowerCase();

      // Only handle `help` commands; `describe *` is handled by other event flows.
      if (!lower.startsWith('help')) return;

      // Strip "help " prefix and lowercase; bare "help" → empty string (root index).
      const topic = extractHelpTopic(event.commandText);

      // Only open the panel when locally bundled content exists.  For
      // non-bundled topics the server response in the console is the only
      // copy — opening a panel to show "not found" is low-value UX.
      if (getHelpContent(topic) === null) return;

      setHelpTopic(topic);
      onTabSwitchRef.current();
    });
  }, [bus, setHelpTopic]);
}
