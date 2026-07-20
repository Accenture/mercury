import NavMenu from '../NavMenu/NavMenu';
import type { SavedGraphEntry } from '../../hooks/useSavedGraphs';
import styles from './SavedGraphsMenu.module.css';

interface SavedGraphsMenuProps {
  /** All saved entries, newest-first. */
  savedGraphs: SavedGraphEntry[];
  /** Called when the user clicks Load on an entry. */
  onLoad:      (name: string) => void;
  /** Called when the user clicks Delete on an entry. */
  onDelete:    (name: string) => void;
  /** When false, Load buttons are disabled. */
  connected:   boolean;
}

/**
 * SavedGraphsMenu
 *
 * A header-bar dropdown listing all saved graph bookmarks.
 * Reuses NavMenu for consistent open/close, Escape, and outside-click behaviour.
 *
 * Replaces the full SavedGraphsPanel right-panel tab — the same load/delete
 * actions are accessible without consuming permanent panel real estate.
 */
export default function SavedGraphsMenu({
  savedGraphs,
  onLoad,
  onDelete,
  connected,
}: SavedGraphsMenuProps) {
  const label = savedGraphs.length > 0
    ? `Load Graph (${savedGraphs.length})`
    : 'Load Graph';

  return (
    <NavMenu label={label}>
      {savedGraphs.length === 0 ? (
        <p className={styles.empty}>No saved graphs yet.</p>
      ) : (
        <>
          {!connected && (
            <p className={styles.hint}>Connect to load a graph</p>
          )}
          <ul className={styles.list} role="list">
            {savedGraphs.map(entry => (
              <li key={entry.name} className={styles.row}>
                <div className={styles.rowInfo}>
                  <span className={styles.rowName} title={entry.name}>
                    {entry.name}
                  </span>
                  <span className={styles.rowMeta}>
                    {new Date(entry.savedAt).toLocaleString()}
                  </span>
                </div>
                <div className={styles.rowActions}>
                  <button
                    className={styles.loadBtn}
                    onClick={() => onLoad(entry.name)}
                    disabled={!connected}
                    title={
                      connected
                        ? `Run: import graph from ${entry.name}`
                        : 'Connect to the playground first'
                    }
                    aria-label={`Load graph ${entry.name}`}
                  >
                    Load
                  </button>
                  <button
                    className={styles.deleteBtn}
                    onClick={() => onDelete(entry.name)}
                    title={`Remove "${entry.name}" from local storage`}
                    aria-label={`Delete saved graph ${entry.name}`}
                  >
                    Delete
                  </button>
                </div>
              </li>
            ))}
          </ul>
        </>
      )}
    </NavMenu>
  );
}
