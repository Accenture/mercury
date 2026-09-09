import { useId, useState, type ReactNode } from 'react';
import type { GraphRunPhase } from '../../hooks/useGraphRunWorkflow';
import { InstantiateIcon, RunIcon } from '../../icons/GraphToolbarIcons';
import styles from './GraphToolbar.module.css';

export interface GraphRunControlsProps {
  phase: GraphRunPhase;
  canInstantiate: boolean;
  canRun: boolean;
  disabledReason: string;
  onInstantiate: () => void;
  onRun: () => void;
}

const SETUP_BUSY_PHASES = new Set<GraphRunPhase>([
  'instantiating',
  'requesting-input',
  'awaiting-input',
  'outcome-uncertain',
]);

function instantiateLabel(phase: GraphRunPhase): string {
  switch (phase) {
    case 'instantiating': return 'Instantiating…';
    case 'requesting-input': return 'Preparing…';
    case 'awaiting-input': return 'Input required';
    case 'outcome-uncertain': return 'Waiting…';
    default: return 'Instantiate';
  }
}

function instantiateAriaLabel(phase: GraphRunPhase): string {
  switch (phase) {
    case 'instantiating': return 'Graph is being instantiated';
    case 'requesting-input': return 'Graph input is being prepared';
    case 'awaiting-input': return 'Graph is waiting for input';
    case 'outcome-uncertain': return 'Waiting for the backend graph outcome';
    default: return 'Instantiate graph';
  }
}

function busyTitle(phase: GraphRunPhase): string {
  switch (phase) {
    case 'instantiating': return 'Graph is being instantiated';
    case 'requesting-input': return 'Preparing graph input';
    case 'awaiting-input': return 'Complete or cancel the mock graph input panel';
    case 'running': return 'Graph is running';
    case 'outcome-uncertain': return 'Waiting for the backend graph outcome';
    default: return '';
  }
}

function appendStatus(purpose: string, status: string): string {
  if (!status) return purpose;
  const statusSentence = /[.!?]$/.test(status) ? status : `${status}.`;
  return `${purpose} ${statusSentence}`;
}

interface ActionTooltipProps {
  id: string;
  text: string;
  keyboardFallback: boolean;
  children: ReactNode;
}

function ActionTooltip({ id, text, keyboardFallback, children }: ActionTooltipProps) {
  const [hovered, setHovered] = useState(false);
  const [focused, setFocused] = useState(false);

  return (
    <span
      className={styles.tooltipAnchor}
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      onFocusCapture={() => setFocused(true)}
      onBlurCapture={(event) => {
        if (!event.currentTarget.contains(event.relatedTarget)) setFocused(false);
      }}
      tabIndex={keyboardFallback ? 0 : undefined}
      aria-describedby={keyboardFallback ? id : undefined}
    >
      {children}
      <span
        id={id}
        className={styles.actionTooltip}
        role="tooltip"
        data-state={hovered || focused ? 'open' : 'closed'}
      >
        {text}
      </span>
    </span>
  );
}

export default function GraphRunControls({
  phase,
  canInstantiate,
  canRun,
  disabledReason,
  onInstantiate,
  onRun,
}: GraphRunControlsProps) {
  const tooltipId = useId();
  const instantiateTooltipId = `${tooltipId}-instantiate`;
  const runTooltipId = `${tooltipId}-run`;
  const setupBusy = SETUP_BUSY_PHASES.has(phase);
  const phaseTitle = busyTitle(phase);
  const instantiateStatus = disabledReason || phaseTitle || (
    phase === 'ready' ? 'The graph is already instantiated' : ''
  );
  const runStatus = disabledReason || phaseTitle || (!canRun ? 'Instantiate the graph first' : '');
  const instantiateTooltip = appendStatus(
    'Prepare the current graph to run and add input if required.',
    instantiateStatus,
  );
  const runTooltip = appendStatus(
    'Run the graph after it has been instantiated.',
    runStatus,
  );

  return (
    <div className={styles.runControls} role="group" aria-label="Graph run controls">
      <ActionTooltip
        id={instantiateTooltipId}
        text={instantiateTooltip}
        keyboardFallback={!canInstantiate}
      >
        <button
          type="button"
          className={styles.toolbarButton}
          onClick={onInstantiate}
          disabled={!canInstantiate}
          aria-label={instantiateAriaLabel(phase)}
          aria-describedby={instantiateTooltipId}
          aria-busy={setupBusy}
        >
          <InstantiateIcon className={styles.toolbarIcon} aria-hidden="true" focusable="false" />
          <span>{instantiateLabel(phase)}</span>
        </button>
      </ActionTooltip>
      <ActionTooltip id={runTooltipId} text={runTooltip} keyboardFallback={!canRun}>
        <button
          type="button"
          className={styles.toolbarButton}
          onClick={onRun}
          disabled={!canRun}
          aria-label={phase === 'ready'
            ? 'Run instantiated graph'
            : phase === 'running'
              ? 'Graph is running'
              : 'Run graph'}
          aria-describedby={runTooltipId}
          aria-busy={phase === 'running'}
        >
          <RunIcon className={styles.toolbarIcon} aria-hidden="true" focusable="false" />
          <span>{phase === 'running' ? 'Running…' : 'Run'}</span>
        </button>
      </ActionTooltip>
    </div>
  );
}
