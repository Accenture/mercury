import { ConsoleErrorBoundary } from './ConsoleErrorBoundary';
import ConsoleMessage from './ConsoleMessage';
import styles from './Console.module.css';
import type { ProtocolEvent } from '../../protocol/events';

interface ConsoleProps {
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
}

export default function Console({
  messages,
  classificationMap,
  onCopy,
  onClear,
  consoleRef,
  onGraphLinkMessage,
  onCopyMessage,
  onSendToJsonPath,
  onUploadMockData,
  successfulUploadPaths,
}: ConsoleProps) {
  return (
    <div className={styles.consoleRoot}>
      <div className={styles.consoleHeader}>
        <span className={styles.consoleTitle}>Console Output</span>
        <div className={styles.consoleControls}>
          <button
            className={styles.controlButton}
            onClick={onCopy}
            title="Copy console output"
            aria-label="Copy console output to clipboard"
          >
            📑
          </button>
          <button
            className={styles.controlButton}
            onClick={onClear}
            title="Clear console"
            aria-label="Clear console"
          >
            🗑️
          </button>
        </div>
      </div>

      <div className={styles.console} ref={consoleRef} role="log" aria-live="polite">
        {messages.map((msg) => (
          <ConsoleErrorBoundary key={msg.id} fallback={msg.raw}>
            <ConsoleMessage
              message={msg.raw}
              msgId={msg.id}
              classificationMap={classificationMap}
              onGraphLink={onGraphLinkMessage ? () => onGraphLinkMessage(msg) : undefined}
              onCopyMessage={onCopyMessage}
              onSendToJsonPath={onSendToJsonPath}
              onUploadMockData={onUploadMockData}
              successfulUploadPaths={successfulUploadPaths}
            />
          </ConsoleErrorBoundary>
        ))}
        {messages.length === 0 && (
          <div className={styles.emptyConsole}>
            No messages yet. Use the <strong>Start</strong> button in the header to connect.
          </div>
        )}
      </div>
    </div>
  );
}
