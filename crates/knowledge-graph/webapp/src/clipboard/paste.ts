import { buildNodeCommand } from './commandBuilder';
import type { ClipboardItemRecord } from './db';
import type { MinigraphGraphData } from '../utils/graphTypes';

export interface ClipboardPastePlan {
  verb: 'create' | 'update';
  command: string;
}

export function buildClipboardPastePlan(
  item: ClipboardItemRecord,
  graphData: MinigraphGraphData | null,
): ClipboardPastePlan {
  const verb = graphData?.nodes.some(node => node.alias === item.node.alias)
    ? 'update'
    : 'create';

  return {
    verb,
    command: buildNodeCommand(verb, item.node),
  };
}