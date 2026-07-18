import { NavLink } from 'react-router-dom';
import { PLAYGROUND_CONFIGS } from '../config/playgrounds';
import { useWebSocketContext } from '../contexts/WebSocketContext';
import { type WsPhase } from '../contexts/WebSocketContext';
import { type ToastType } from '../hooks/useToast';
import NavMenu, { type DotStatus } from './NavMenu/NavMenu';
import { makeWsUrl } from '../utils/urls';
import styles from './Navigation.module.css';

// ---------
// Helpers
// ---------

/**
 * Derives a single aggregate dot status from an array of per-playground phases.
 * - all connected  → 'connected'
 * - all idle       → 'idle'
 * - any connecting → 'connecting'
 * - mixed          → 'partial'
 */
function aggregateDotStatus(phases: WsPhase[]): DotStatus {
  if (phases.every(p => p === 'connected'))  return 'connected';
  if (phases.every(p => p === 'idle'))       return 'idle';
  if (phases.some(p => p === 'connecting'))  return 'connecting';
  return 'partial';
}

function phaseToDotStatus(phase: WsPhase): DotStatus {
  if (phase === 'connected')  return 'connected';
  if (phase === 'connecting') return 'connecting';
  return 'idle';
}

// -----------------------------
// External / server info links
// -----------------------------

const EXTERNAL_LINKS = [
  { href: '/info',        label: 'Info' },
  { href: '/info/lib',    label: 'Libraries' },
  { href: '/info/routes', label: 'Services' },
  { href: '/health',      label: 'Health' },
  { href: '/env',         label: 'Environment' },
  { href: 'http://localhost:8085/api/ws/json', label: 'Legacy JSON' },
  { href: 'http://localhost:8085/api/ws/graph', label: 'Legacy Graph' },
] as const;

// ----------
// Component
// ----------

interface NavigationProps {
  /** Forwarded from the host Playground's useToast() so connect/disconnect
   *  notifications appear in the same rendered toast stack. */
  addToast: (message: string, type?: ToastType) => void;
}

export default function Navigation({ addToast }: NavigationProps) {
  const ctx = useWebSocketContext();

  // Collect live phases for the aggregate dot on the Tools menu
  const phases = PLAYGROUND_CONFIGS.map(cfg => ctx.getSlot(cfg.wsPath).phase);
  const toolsDotStatus = aggregateDotStatus(phases);

  const allConnected  = phases.every(p => p === 'connected');
  const anyConnecting = phases.some(p => p === 'connecting');

  function handleConnectAll() {
    PLAYGROUND_CONFIGS.forEach(cfg => {
      if (ctx.getSlot(cfg.wsPath).phase === 'idle') {
        ctx.connect(cfg.wsPath, addToast);
      }
    });
  }

  function handleDisconnectAll() {
    PLAYGROUND_CONFIGS.forEach(cfg => {
      const { phase } = ctx.getSlot(cfg.wsPath);
      if (phase === 'connected' || phase === 'connecting') {
        ctx.disconnect(cfg.wsPath);
      }
    });
  }

  return (
    <nav className={styles.nav} aria-label="Main navigation">

      {/* ── Tools (nav + connection status combined) ── */}
      <NavMenu label="Tools" dotStatus={toolsDotStatus}>
        {/* ── Connect / Disconnect All ── */}
        <div className={styles.connectAllRow}>
          <button
            className={`${styles.connectAllBtn} ${allConnected ? styles.connectAllBtnStop : ''}`}
            onClick={allConnected ? handleDisconnectAll : handleConnectAll}
            disabled={anyConnecting}
            aria-label={anyConnecting ? 'Connecting…' : allConnected ? 'Disconnect all WebSockets' : 'Connect all WebSockets'}
          >
            {anyConnecting ? 'Connecting…' : allConnected ? 'Disconnect All' : 'Connect All'}
          </button>
        </div>

        <ul className={styles.menuList} role="none">
          {PLAYGROUND_CONFIGS.map((cfg) => {
            const { phase } = ctx.getSlot(cfg.wsPath);
            const dotStatus = phaseToDotStatus(phase);
            const isConnected  = phase === 'connected';
            const isConnecting = phase === 'connecting';

            const dotClass =
              dotStatus === 'connected'  ? styles.toolDotConnected  :
              dotStatus === 'connecting' ? styles.toolDotConnecting :
                                          styles.toolDotIdle;

            return (
              <li key={cfg.path} role="none" className={styles.toolRow}>
                {/* Navigate to the playground */}
                <NavLink
                  to={cfg.path}
                  role="menuitem"
                  className={({ isActive }) =>
                    `${styles.toolLink} ${isActive ? styles.toolLinkActive : ''}`
                  }
                >
                  <span className={`${styles.toolDot} ${dotClass}`} aria-hidden="true" />
                  <span className={styles.toolLabel}>{cfg.label}</span>
                </NavLink>

                {/* Connect / disconnect — kept separate from the NavLink so
                    clicking it doesn't also navigate */}
                <button
                  className={`${styles.toolConnectBtn} ${isConnected ? styles.toolConnectBtnStop : ''}`}
                  onClick={() =>
                    isConnected || isConnecting
                      ? ctx.disconnect(cfg.wsPath)
                      : ctx.connect(cfg.wsPath, addToast)
                  }
                  disabled={isConnecting}
                  aria-label={
                    isConnecting ? 'Connecting…' :
                    isConnected  ? `Disconnect ${cfg.label}` :
                                   `Connect ${cfg.label}`
                  }
                  title={isConnecting ? 'Connecting…' : makeWsUrl(cfg.wsPath)}
                >
                  {isConnecting ? '…' : isConnected ? 'Stop' : 'Start'}
                </button>
              </li>
            );
          })}
        </ul>
      </NavMenu>

      {/* ── Quick Links ─── */}
      <NavMenu label="Quick Links">
        <ul className={styles.menuList} role="none">
          {EXTERNAL_LINKS.map(link => (
            <li key={link.href} role="none">
              <a
                href={link.href}
                role="menuitem"
                className={styles.menuItem}
                target="_blank"
                rel="noopener noreferrer"
              >
                {link.label}
                <span className={styles.externalIcon} aria-hidden="true">↗</span>
              </a>
            </li>
          ))}
        </ul>
      </NavMenu>

    </nav>
  );
}
