import { describe, it, expect } from 'vitest';
import { classifyMessage } from '../classifier';
import graphMutations from './fixtures/graph-mutations.json';
import graphLinks from './fixtures/graph-links.json';
import exportGraph from './fixtures/export-graph.json';
import largePayloads from './fixtures/large-payloads.json';
import mockUploads from './fixtures/mock-uploads.json';
import uploadContentPaths from './fixtures/upload-content-paths.json';
import helpDescribeCommands from './fixtures/help-describe-commands.json';
import importGraphCommands from './fixtures/import-graph-commands.json';
import lifecycleEvents from './fixtures/lifecycle-events.json';
import jsonResponses from './fixtures/json-responses.json';
import markdownCandidates from './fixtures/markdown-candidates.json';
import negativeCases from './fixtures/negative-cases.json';
import multiEvent from './fixtures/multi-event.json';

interface TestVector {
  name:              string;
  raw:               string;
  expectedKinds:     string[];
  expectedProps?:    Record<string, unknown>;
  notExpectedKinds?: string[];
  expectedLength?:   number;
}

function runFixture(vectors: TestVector[]) {
  for (const v of vectors) {
    it(v.name, () => {
      const events = classifyMessage(1, v.raw);
      const kinds = events.map(e => e.kind);

      // All expected kinds are present
      expect(kinds).toEqual(expect.arrayContaining(v.expectedKinds));

      // Verify length if specified
      if (v.expectedLength !== undefined) {
        expect(kinds).toHaveLength(v.expectedLength);
      }

      // No unexpected kinds
      if (v.notExpectedKinds) {
        for (const nk of v.notExpectedKinds) {
          expect(kinds).not.toContain(nk);
        }
      }

      // At least one emitted event must contain all expected props
      if (v.expectedProps) {
        const match = events.find(e =>
          Object.entries(v.expectedProps!).every(([k, val]) =>
            (e as unknown as Record<string, unknown>)[k] === val
          )
        );
        expect(match).toBeDefined();
      }
    });
  }
}

describe('classifier — graph mutations', () => runFixture(graphMutations));
describe('classifier — graph links', () => runFixture(graphLinks));
describe('classifier — export graph', () => runFixture(exportGraph));
describe('classifier — large payloads', () => runFixture(largePayloads));
describe('classifier — mock uploads', () => runFixture(mockUploads));
describe('classifier — upload content paths', () => runFixture(uploadContentPaths));
describe('classifier — help/describe commands', () => runFixture(helpDescribeCommands));
describe('classifier — import graph commands', () => runFixture(importGraphCommands));
describe('classifier — lifecycle events', () => runFixture(lifecycleEvents));
describe('classifier — json responses', () => runFixture(jsonResponses));
describe('classifier — markdown candidates', () => runFixture(markdownCandidates));
describe('classifier — negative cases', () => runFixture(negativeCases));
describe('classifier — multi-event', () => runFixture(multiEvent));

describe('classifier — invariants', () => {
  it('always returns at least one event', () => {
    const events = classifyMessage(1, '');
    expect(events.length).toBeGreaterThanOrEqual(1);
  });

  it('every event carries the original msgId and raw string', () => {
    const events = classifyMessage(42, 'Node foo created');
    for (const e of events) {
      expect(e.msgId).toBe(42);
      expect(e.raw).toBe('Node foo created');
    }
  });

  it('is pure — same input gives same output', () => {
    const a = classifyMessage(1, '> help');
    const b = classifyMessage(1, '> help');
    expect(a).toEqual(b);
  });

  it('lifecycle JSON does not produce json.response', () => {
    const events = classifyMessage(1, '{"type":"info","message":"connected"}');
    const kinds = events.map(e => e.kind);
    expect(kinds).toContain('lifecycle');
    expect(kinds).not.toContain('json.response');
  });

  it('non-lifecycle JSON produces json.response, not lifecycle', () => {
    const events = classifyMessage(1, '{"name":"test"}');
    const kinds = events.map(e => e.kind);
    expect(kinds).toContain('json.response');
    expect(kinds).not.toContain('lifecycle');
  });

  it('emits create-node text result without replacing graph.mutation', () => {
    const events = classifyMessage(1, 'node root created');
    const kinds = events.map(e => e.kind);
    expect(kinds).toContain('minigraph.nodeAction.textResult');
    expect(kinds).toContain('minigraph.createNode.textResult');
    expect(kinds).toContain('graph.mutation');
    expect(events).toContainEqual(expect.objectContaining({
      kind: 'minigraph.nodeAction.textResult',
      status: 'accepted',
      action: 'create-node',
      alias: 'root',
      message: 'node root created',
    }));
    expect(events).toContainEqual(expect.objectContaining({
      kind: 'minigraph.createNode.textResult',
      status: 'accepted',
      alias: 'root',
      message: 'node root created',
    }));
  });

  it('emits duplicate create-node text result as rejected', () => {
    const events = classifyMessage(1, 'node root already exists');
    expect(events).toContainEqual(expect.objectContaining({
      kind: 'minigraph.nodeAction.textResult',
      status: 'rejected',
      action: 'create-node',
      alias: 'root',
      message: 'node root already exists',
    }));
    expect(events).toContainEqual(expect.objectContaining({
      kind: 'minigraph.createNode.textResult',
      status: 'rejected',
      alias: 'root',
      message: 'node root already exists',
    }));
  });

  it('emits edit/delete node-action text results without create compatibility events', () => {
    const updated = classifyMessage(1, 'node root updated');
    expect(updated).toContainEqual(expect.objectContaining({
      kind: 'minigraph.nodeAction.textResult',
      status: 'accepted',
      action: 'edit-node',
      alias: 'root',
      message: 'node root updated',
    }));
    expect(updated.map(e => e.kind)).not.toContain('minigraph.createNode.textResult');

    const deleted = classifyMessage(1, 'node root deleted');
    expect(deleted).toContainEqual(expect.objectContaining({
      kind: 'minigraph.nodeAction.textResult',
      status: 'accepted',
      action: 'delete-node',
      alias: 'root',
      message: 'node root deleted',
    }));
    expect(deleted.map(e => e.kind)).not.toContain('minigraph.createNode.textResult');
  });

  it('emits not-found node-action text result as rejected', () => {
    const events = classifyMessage(1, 'node root not found');
    expect(events).toContainEqual(expect.objectContaining({
      kind: 'minigraph.nodeAction.textResult',
      status: 'rejected',
      action: null,
      alias: 'root',
      message: 'node root not found',
    }));
  });

  it('emits connection success as both graph mutation and node-action text result', () => {
    const events = classifyMessage(1, 'node root connected to mapper');
    const kinds = events.map(e => e.kind);
    expect(kinds).toContain('graph.mutation');
    expect(kinds).toContain('minigraph.nodeAction.textResult');
    expect(events).toContainEqual(expect.objectContaining({
      kind: 'minigraph.nodeAction.textResult',
      status: 'accepted',
      action: 'create-connection',
      alias: 'root',
      targetAlias: 'mapper',
      message: 'node root connected to mapper',
    }));
  });

  it('emits session collaboration events from backend text', () => {
    const started = classifyMessage(1, 'session ws-123456-1 started\nCompanion endpoint: /api/companion/ws-123456-1');
    expect(started).toContainEqual(expect.objectContaining({
      kind: 'minigraph.session.started',
      sessionId: 'ws-123456-1',
      companionEndpoint: '/api/companion/ws-123456-1',
    }));

    const status = classifyMessage(2, [
      'Session ws-178443-2 started since 2026-06-02 10:20:32.054',
      'subscribed to ws-111111-1',
      'subscribed by [ws-485844-4, ws-222222-3]',
    ].join('\n'));
    expect(status).toContainEqual(expect.objectContaining({
      kind: 'minigraph.session.status',
      sessionId: 'ws-178443-2',
      subscribedTo: 'ws-111111-1',
      subscribers: ['ws-485844-4', 'ws-222222-3'],
    }));

    const subscribeResult = classifyMessage(3, 'Subscribed to ws-178443-2');
    expect(subscribeResult).toContainEqual(expect.objectContaining({
      kind: 'minigraph.session.commandResult',
      command: 'subscribe',
      status: 'accepted',
      sessionId: 'ws-178443-2',
    }));

    const notification = classifyMessage(4, 'ws-485844-4 unsubscribed from your session');
    expect(notification).toContainEqual(expect.objectContaining({
      kind: 'minigraph.session.notification',
      type: 'subscriber-left',
      sessionId: 'ws-485844-4',
    }));
  });

  it('does not classify session command echoes as session events', () => {
    const events = classifyMessage(1, '> session subscribe ws-178443-2');
    expect(events.map(e => e.kind)).not.toContain('minigraph.session.commandResult');
    expect(events.map(e => e.kind)).toContain('command.echo');
  });
});
