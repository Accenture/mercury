import { useEffect, useMemo, useRef } from 'react';
import { ProtocolBus } from './bus';
import { classifyMessage } from './classifier';
import type { ProtocolEvent } from './events';

export interface UseProtocolKernelOptions {
  /** The messages array for this playground's WebSocket slot. */
  messages: { id: number; raw: string }[];
  /** The shared ProtocolBus instance for this playground. */
  bus: ProtocolBus;
}

export interface UseProtocolKernelReturn {
  /**
   * Lookup map from message ID to the array of ProtocolEvents produced by
   * classifyMessage().  Computed synchronously from the full `messages`
   * array via `useMemo`.
   */
  classificationMap: Map<number, ProtocolEvent[]>;
}

/**
 * Drives the protocol kernel lifecycle for one playground slot:
 *
 *  1. Maintains a single message-ID watermark (init at mount, advance on
 *     each batch).
 *  2. Classifies each new message via classifyMessage() (once).
 *  3. Emits the resulting typed events on the shared ProtocolBus.
 *
 * The classificationMap covers ALL visible messages (not just new ones)
 * so ConsoleMessage can look up any message's classification.
 */
export function useProtocolKernel({
  messages,
  bus,
}: UseProtocolKernelOptions): UseProtocolKernelReturn {
  const watermarkRef = useRef<number>(-1);

  // Effect 1: Watermark initialization (mount only)
  useEffect(() => {
    if (messages.length > 0) {
      watermarkRef.current = messages[messages.length - 1].id;
    }
  }, []); // eslint-disable-line react-hooks/exhaustive-deps

  // Render-synchronous classification map (covers all visible messages)
  const classificationMap = useMemo(() => {
    const map = new Map<number, ProtocolEvent[]>();
    for (const msg of messages) {
      map.set(msg.id, classifyMessage(msg.id, msg.raw));
    }
    return map;
  }, [messages]);

  // Effect 2: Emit new events on the bus (watermark-gated)
  useEffect(() => {
    if (messages.length === 0) return;
    const newMessages = messages.filter(m => m.id > watermarkRef.current);
    if (newMessages.length === 0) return;
    watermarkRef.current = messages[messages.length - 1].id;

    for (const msg of newMessages) {
      const events = classificationMap.get(msg.id);
      if (events) {
        for (const event of events) {
          bus.emit(event);
        }
      }
    }
  }, [messages, bus, classificationMap]);

  return { classificationMap };
}
