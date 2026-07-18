import type { CSSProperties } from 'react';

export interface MinigraphNodeTypeMeta {
  icon: string;
  label: string;
}

const TYPE_META: Record<string, MinigraphNodeTypeMeta> = {
  Root:        { icon: '🚀', label: 'Root' },
  End:         { icon: '🏁', label: 'End' },
  Fetcher:     { icon: '🌐', label: 'Fetcher' },
  mapper:      { icon: '🗺️', label: 'Mapper' },
  Math:        { icon: '🔢', label: 'Math' },
  JavaScript:  { icon: '📜', label: 'JavaScript' },
  Provider:    { icon: '🔌', label: 'Provider' },
  Dictionary:  { icon: '📖', label: 'Dictionary' },
  Join:        { icon: '🔀', label: 'Join' },
  Extension:   { icon: '🧩', label: 'Extension' },
  Island:      { icon: '🏝️', label: 'Island' },
  Decision:    { icon: '❓', label: 'Decision' },
};

const BASE_NODE_STYLE: CSSProperties = {
  boxSizing: 'border-box',
  borderRadius: '8px',
  borderWidth: '1.5px',
  borderStyle: 'solid',
  background: 'var(--bg-secondary, #1e1e2e)',
  color: 'var(--text-primary, #cdd6f4)',
  fontSize: '0.75rem',
  boxShadow: '0 2px 8px rgba(0,0,0,0.45)',
  overflow: 'visible',
  padding: 0,
};

const NODE_ACCENT: Record<string, string> = {
  Root: '#15803d',
  End: '#dc2626',
  Fetcher: '#2563eb',
  mapper: '#ea580c',
  Math: '#a16207',
  JavaScript: '#7e22ce',
  Provider: '#be185d',
  Dictionary: '#0e7490',
  Join: '#65a30d',
  Extension: '#4338ca',
  Island: '#475569',
  Decision: '#b45309',
};

const UNKNOWN_ACCENT = '#6c7086';

export function getMinigraphNodeTypeMeta(nodeType: string): MinigraphNodeTypeMeta {
  return TYPE_META[nodeType] ?? { icon: '📦', label: nodeType };
}

export function getMinigraphNodeShellStyle(nodeType: string): CSSProperties {
  const accent = NODE_ACCENT[nodeType] ?? UNKNOWN_ACCENT;
  return {
    ...BASE_NODE_STYLE,
    borderColor: accent,
    ['--node-accent' as string]: accent,
  };
}