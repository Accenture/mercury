import { describe, expect, it } from 'vitest';
import { PLAYGROUND_CONFIGS } from '../playgrounds';

describe('playground Help configuration', () => {
  it('enables an isolated JSON Path Help profile', () => {
    const jsonPath = PLAYGROUND_CONFIGS.find(config => config.path === '/json-path');
    const minigraph = PLAYGROUND_CONFIGS.find(config => config.path === '/');

    expect(jsonPath).toMatchObject({
      supportsHelp: true,
      helpContentProfile: 'json-path',
      storageKeyHelpTopic: 'jsonpath-help-topic',
    });
    expect(minigraph).toMatchObject({
      supportsHelp: true,
      helpContentProfile: 'minigraph',
      storageKeyHelpTopic: 'minigraph-help-topic',
    });
  });

  it('enables header graph-run controls only for MiniGraph', () => {
    const jsonPath = PLAYGROUND_CONFIGS.find(config => config.path === '/json-path');
    const minigraph = PLAYGROUND_CONFIGS.find(config => config.path === '/');

    expect(minigraph?.supportsGraphRun).toBe(true);
    expect(jsonPath?.supportsGraphRun).toBeUndefined();
  });
});
