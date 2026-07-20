// Thin boundary between the authoring lifecycle and the transport layer.
// useGraphAuthoring depends on this interface so it can be tested with a fake
// executor and does not need to import or understand the WebSocket hook.
export interface GraphAuthoringExecutor {
  execute(commandText: string): boolean;
}

// Production adapter for the raw command transport. It delegates to sendRawText
// and preserves its boolean "accepted by open socket" result.
export function createGraphAuthoringExecutor(
  sendRawText: (text: string) => boolean,
): GraphAuthoringExecutor {
  return {
    execute(commandText) {
      return sendRawText(commandText);
    },
  };
}
