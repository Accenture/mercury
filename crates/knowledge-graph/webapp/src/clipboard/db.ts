import { openDB, deleteDB, type DBSchema, type IDBPDatabase } from 'idb';
import type { MinigraphNode, MinigraphConnection } from '../utils/graphTypes';

/**
 * A snapshot of a single node plus its direct connections at the time of
 * clipping. This is the primary record stored in IndexedDB.
 */
export interface ClipboardItemRecord {
  /** Primary key. Generated via crypto.randomUUID() at clip time. */
  id: string;

  /** ISO 8601 timestamp of when the node was clipped. */
  clippedAt: string;

  /** The wsPath of the playground from which the node was clipped. */
  sourceWsPath: string;

  /** Human-readable label of the source playground. */
  sourceLabel: string;

  /** Full snapshot of the MinigraphNode at clip time. */
  node: MinigraphNode;

  /** All direct connections for this node, excluding self-connections. */
  connections: MinigraphConnection[];
}

const DB_NAME    = 'minigraph-clipboard';
const DB_VERSION = 1;
const STORE_NAME = 'items';

interface ClipboardDB extends DBSchema {
  [STORE_NAME]: {
    key: string;
    value: ClipboardItemRecord;
    indexes: {
      'by-alias':     string;
      'by-clippedAt': string;
    };
  };
}

/** Singleton database promise. Reused by all callers. */
let dbInstance: Promise<IDBPDatabase<ClipboardDB>> | null = null;

function openClipboardDB(): Promise<IDBPDatabase<ClipboardDB>> {
  return openDB<ClipboardDB>(DB_NAME, DB_VERSION, {
    upgrade(db) {
      // Delete any stale object store left over from a prior schema version
      // so the fresh store + indexes are created cleanly.
      if (db.objectStoreNames.contains(STORE_NAME)) {
        db.deleteObjectStore(STORE_NAME);
      }
      const store = db.createObjectStore(STORE_NAME, { keyPath: 'id' });
      store.createIndex('by-alias', 'node.alias', { unique: true });
      store.createIndex('by-clippedAt', 'clippedAt');
    },
  });
}

export function getDB(): Promise<IDBPDatabase<ClipboardDB>> {
  if (!dbInstance) {
    dbInstance = openClipboardDB().catch(async (err) => {
      // Self-healing: if the database is corrupt or has a stale schema from
      // a prior development cycle, delete it entirely and recreate.
      console.warn('[clipboard/db] openDB failed, deleting and recreating:', err);
      dbInstance = null;
      await deleteDB(DB_NAME);
      return openClipboardDB();
    });
  }
  return dbInstance;
}

/** Retrieve all clipboard items, sorted newest-first by clippedAt. */
export async function getAllItems(): Promise<ClipboardItemRecord[]> {
  const db = await getDB();
  const items = await db.getAllFromIndex(STORE_NAME, 'by-clippedAt');
  return items.reverse();
}

/** Find an existing clipboard item by node alias. */
export async function findByAlias(alias: string): Promise<ClipboardItemRecord | undefined> {
  const db = await getDB();
  return db.getFromIndex(STORE_NAME, 'by-alias', alias);
}

/** Insert a new clipboard item. */
export async function addItem(item: ClipboardItemRecord): Promise<void> {
  const db = await getDB();
  await db.add(STORE_NAME, item);
}

/** Replace an existing clipboard item (same id). */
export async function putItem(item: ClipboardItemRecord): Promise<void> {
  const db = await getDB();
  await db.put(STORE_NAME, item);
}

/** Atomically remove the old item and insert a new one in a single transaction. */
export async function replaceItem(
  previousId: string,
  newItem: ClipboardItemRecord,
): Promise<void> {
  const db = await getDB();
  const tx = db.transaction(STORE_NAME, 'readwrite');
  await tx.store.delete(previousId);
  await tx.store.add(newItem);
  await tx.done;
}

/** Remove a single clipboard item by id. */
export async function removeItem(id: string): Promise<void> {
  const db = await getDB();
  await db.delete(STORE_NAME, id);
}

/** Remove all clipboard items. */
export async function clearAll(): Promise<void> {
  const db = await getDB();
  await db.clear(STORE_NAME);
}
