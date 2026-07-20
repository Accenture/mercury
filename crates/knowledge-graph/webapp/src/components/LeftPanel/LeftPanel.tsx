import React from 'react';
import Console from '../Console/Console';
import CommandInput from '../CommandInput/CommandInput';
import styles from './LeftPanel.module.css';
import type { ProtocolEvent } from '../../protocol/events';

interface LeftPanelProps {
  // Console props
  messages:           { id: number; raw: string }[];
  classificationMap?: Map<number, ProtocolEvent[]>;
  onCopy:             () => void;
  onClear:            () => void;
  consoleRef:         React.RefObject<HTMLDivElement | null>;
  onGraphLinkMessage?: (msg: { id: number; raw: string }) => void;
  /** Called after any per-message copy succeeds — use this to show a toast. */
  onCopyMessage?:     () => void;
  /** When provided, a "Send to JSON-Path" button appears on JSON messages. */
  onSendToJsonPath?:  (json: string) => void;
  /** When provided, a "⬆️ Upload JSON…" re-open button appears on mock-upload invitation rows. */
  onUploadMockData?:  (uploadPath: string) => void;
  /** Set of POST paths for which a mock upload has succeeded this session. */
  successfulUploadPaths?: Set<string>;
  // CommandInput props
  command:            string;
  onCommandChange:    (value: string) => void;
  onCommandKeyDown:   (e: React.KeyboardEvent<HTMLElement>) => void;
  onSend:             () => void;
  sendDisabled:       boolean;
  inputDisabled:      boolean;
  /** Ordered command history (newest-first) for history-based autocomplete. */
  commandHistory:     string[];
}

export default function LeftPanel({
  messages, classificationMap, onCopy, onClear, consoleRef,
  onGraphLinkMessage, onCopyMessage, onSendToJsonPath,
  onUploadMockData, successfulUploadPaths,
  command, onCommandChange, onCommandKeyDown, onSend,
  sendDisabled, inputDisabled, commandHistory,
}: LeftPanelProps) {
  return (
    <div className={styles.root}>
      <Console
        messages={messages}
        classificationMap={classificationMap}
        onCopy={onCopy}
        onClear={onClear}
        consoleRef={consoleRef}
        onGraphLinkMessage={onGraphLinkMessage}
        onCopyMessage={onCopyMessage}
        onSendToJsonPath={onSendToJsonPath}
        onUploadMockData={onUploadMockData}
        successfulUploadPaths={successfulUploadPaths}
      />
      <CommandInput
        command={command}
        onChange={onCommandChange}
        onKeyDown={onCommandKeyDown}
        onSend={onSend}
        disabled={inputDisabled}
        sendDisabled={sendDisabled}
        history={commandHistory}
      />
    </div>
  );
}
