import type { ProtocolEvent, ProtocolEventKind } from './events';

type Listener<K extends ProtocolEventKind> = (
  event: Extract<ProtocolEvent, { kind: K }>
) => void;

/**
 * Lightweight typed event emitter for protocol events.
 *
 * Synchronous dispatch — `emit` calls listeners in registration order.
 * No microtask deferral.
 *
 * **Ordering invariant:** Listeners for a given `kind` fire in insertion
 * order, guaranteed by the ECMAScript `Set` iteration spec (§24.2.5.4).
 */
export class ProtocolBus {
  private listeners = new Map<string, Set<Function>>();

  /**
   * Subscribe to a specific event kind.
   * Returns an unsubscribe function (idiomatic for `useEffect` cleanup).
   */
  on<K extends ProtocolEventKind>(kind: K, listener: Listener<K>): () => void {
    const key = kind as string;
    if (!this.listeners.has(key)) this.listeners.set(key, new Set());
    this.listeners.get(key)!.add(listener);
    return () => { this.listeners.get(key)?.delete(listener); };
  }

  /** Emit an event to all listeners registered for its `kind`. */
  emit(event: ProtocolEvent): void {
    const set = this.listeners.get(event.kind);
    if (set) {
      set.forEach(fn => {
        try { (fn as (e: ProtocolEvent) => void)(event); }
        catch (err) { console.error(`[ProtocolBus] listener for '${event.kind}' threw:`, err); }
      });
    }
  }

  /** Remove all listeners.  Used in tests; not called in production code. */
  clear(): void {
    this.listeners.clear();
  }
}
