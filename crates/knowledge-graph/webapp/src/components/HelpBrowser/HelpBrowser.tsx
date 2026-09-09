import React, { useEffect, useMemo, useRef } from 'react';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import {
  getHelpContent,
  resolveCategory,
  getCategoryPages,
  getChipLabel,
  getHelpCategories,
  getOrderedHelpPages,
  type HelpCategory,
  type HelpContentProfile,
} from '../../data/helpContent';
import { useHelpScrollNavigation } from '../../hooks/useHelpScrollNavigation';
import styles from './HelpBrowser.module.css';

interface HelpBrowserProps {
  /** "" = root index (help.md); "create" = help create.md; etc. */
  activeTopic: string;
  /** Called when the user navigates to a different topic (or back to root). */
  onNavigate: (topic: string) => void;
  /** Called when the user clicks the close button. Omit to hide the button. */
  onClose?: () => void;
  /** Toggle between maximized (100%) and default height. Omit to hide the button. */
  onToggleMaximize?: () => void;
  /** Whether the help panel is currently maximized. */
  isMaximized?: boolean;
  /** Selects the playground-specific content and navigation set. */
  contentProfile?: HelpContentProfile;
}

/**
 * Recursively extracts a plain-text string from a ReactNode.
 * Handles strings, numbers, arrays, and nested React elements.
 */
function extractTextContent(node: React.ReactNode): string {
  if (typeof node === 'string') return node;
  if (typeof node === 'number') return String(node);
  if (Array.isArray(node)) return node.map(extractTextContent).join('');
  if (React.isValidElement(node)) {
    return extractTextContent((node.props as { children?: React.ReactNode }).children);
  }
  return '';
}

/**
 * Extracts a bare topic key from a list-item text string such as:
 *   "help create (node)"      → "create"
 *   "help data-dictionary"    → "data-dictionary"
 *   "help tutorial 1"         → "tutorial 1"
 *   "help connect (A to B)"   → "connect"
 *
 * Returns null if the text does not start with "help ".
 */
function extractTopicFromListText(text: string): string | null {
  const lower = text.trim().toLowerCase();
  if (!lower.startsWith('help ')) return null;
  const withoutHelp = text.trim().slice('help '.length);
  // Drop trailing parenthetical annotation: "create (node)" → "create"
  const topic = withoutHelp.replace(/\s*\(.*\)\s*$/, '').trim().toLowerCase();
  return topic.length > 0 ? topic : null;
}

export default function HelpBrowser({
  activeTopic,
  onNavigate,
  onClose,
  onToggleMaximize,
  isMaximized,
  contentProfile = 'minigraph',
}: HelpBrowserProps) {
  const scrollRef         = useRef<HTMLDivElement>(null);
  const helpRootRef       = useRef<HTMLDivElement>(null);
  const contentWrapperRef = useRef<HTMLDivElement>(null);
  const chipStripRef      = useRef<HTMLDivElement>(null);

  // Scroll to top whenever the active topic changes.
  useEffect(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollTop = 0;
    }
  }, [activeTopic]);

  // Auto-scroll the chip strip so the active chip is visible.
  useEffect(() => {
    const strip = chipStripRef.current;
    if (!strip) return;
    const activeChip = strip.querySelector<HTMLElement>('[aria-current="step"]');
    if (activeChip) {
      activeChip.scrollIntoView({ block: 'nearest', inline: 'nearest', behavior: 'smooth' });
    }
  }, [activeTopic]);

  // ── Category-scoped navigation ────────────────────────────────────────────
  const helpCategories = useMemo(
    () => getHelpCategories(contentProfile),
    [contentProfile]
  );
  const orderedHelpPages = useMemo(
    () => getOrderedHelpPages(contentProfile),
    [contentProfile]
  );
  const activeCategory: HelpCategory = resolveCategory(activeTopic, contentProfile);
  const categoryPages = useMemo(
    () => getCategoryPages(activeCategory, contentProfile),
    [activeCategory, contentProfile]
  );
  const categoryTotal = categoryPages.length;

  // Label rendered inside the chip strip before the topic chips (e.g. "Chapters").
  const chipStripLabel = useMemo(
    () => helpCategories.find(cat => cat.id === activeCategory)?.chipStripLabel ?? null,
    [activeCategory, helpCategories]
  );

  // Index within the GLOBAL page list (for cross-category scroll wrapping).
  const globalIndex = orderedHelpPages.indexOf(activeTopic);
  const safeGlobalIndex = globalIndex < 0 ? 0 : globalIndex;
  const globalTotal = orderedHelpPages.length;

  // ── Scroll-navigation hook (uses global index for cross-category wrap) ────
  useHelpScrollNavigation({
    wheelTargetRef: helpRootRef,
    scrollRef,
    contentWrapperRef,
    currentIndex: safeGlobalIndex,
    totalPages:   globalTotal,
    onNavigatePrev: () => onNavigate(orderedHelpPages[safeGlobalIndex - 1] ?? ''),
    onNavigateNext: () =>
      onNavigate(
        orderedHelpPages[safeGlobalIndex + 1] ??
        orderedHelpPages[orderedHelpPages.length - 1]!
      ),
  });

  const content = getHelpContent(activeTopic, contentProfile);

  // Reusable custom `li` renderer — makes list items that match "help <topic>"
  // clickable buttons so the user can navigate without typing in the console.
  const liRenderer = ({ children, ...props }: React.ComponentPropsWithoutRef<'li'>) => {
    const text = extractTextContent(children).trim();
    const topic = extractTopicFromListText(text);
    if (topic !== null && getHelpContent(topic, contentProfile) !== null) {
      return (
        <li {...props}>
          <button
            className={styles.topicLink}
            aria-label={`Open help topic: ${topic}`}
            onClick={() => onNavigate(topic)}
          >
            {children}
          </button>
        </li>
      );
    }
    return <li {...props}>{children}</li>;
  };

  return (
    <div className={styles.helpRoot} role="region" aria-label="Help browser" ref={helpRootRef}>
      {/* ── Category tabs ──────────────────────────────────────────────── */}
      <nav className={styles.categoryNav} aria-label="Help categories">
        <div className={styles.categoryTabScroller}>
          {helpCategories.map(cat => (
            <button
              key={cat.id}
              className={[
                styles.categoryTab,
                cat.id === activeCategory ? styles.categoryTabActive : '',
              ].join(' ').trim()}
              aria-current={cat.id === activeCategory ? 'true' : undefined}
              onClick={() => {
                const pages = getCategoryPages(cat.id, contentProfile);
                onNavigate(pages[0] ?? '');
              }}
            >
              {cat.label}
            </button>
          ))}
        </div>
        {onToggleMaximize && (
          <button
            className={styles.maximizeButton}
            onClick={onToggleMaximize}
            aria-label={isMaximized ? 'Restore help panel' : 'Maximize help panel'}
          >
            {isMaximized ? '⊞' : '⛶'}
          </button>
        )}
        {onClose && (
          <button
            className={styles.closeButton}
            onClick={onClose}
            aria-label="Close help panel"
          >
            ×
          </button>
        )}
      </nav>

      {/* ── Scoped topic chips ──────────────────────────────────────── */}
      {categoryTotal > 1 && (
        <div className={styles.chipStrip} ref={chipStripRef}>
          {chipStripLabel !== null && (
            <span className={styles.chipStripLabel}>{chipStripLabel}</span>
          )}
          {categoryPages.map(topic => {
            const isActive = topic === activeTopic;
            const label    = getChipLabel(topic, activeCategory);
            return (
              <button
                key={topic}
                className={[
                  styles.topicChip,
                  isActive ? styles.topicChipActive : '',
                ].join(' ').trim()}
                aria-current={isActive ? 'step' : undefined}
                onClick={() => onNavigate(topic)}
              >
                {label}
              </button>
            );
          })}
        </div>
      )}

      {/* ── Scrollable body ────────────────────────────────────────────── */}
      <div className={styles.helpBody} ref={scrollRef}>
        <div className={styles.helpBodyContent} ref={contentWrapperRef}>
          {content === null ? (
            <div className={styles.emptyFallback}>
              <code>help {activeTopic || ''}</code>&nbsp; not found in the local bundle.
            </div>
          ) : (
            <div className={styles.helpContent}>
              <ReactMarkdown
                remarkPlugins={[remarkGfm]}
                components={
                  activeTopic === '' ? { li: liRenderer } : undefined
                }
              >
                {content}
              </ReactMarkdown>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
