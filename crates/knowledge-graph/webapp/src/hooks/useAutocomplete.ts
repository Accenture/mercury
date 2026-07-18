import { useState, useCallback, useMemo } from 'react';
import { getSuggestions, type CommandSuggestion } from '../utils/commandSuggestions';

export interface UseAutocompleteReturn {
  /** Filtered list of suggestions for the current command. */
  suggestions: CommandSuggestion[];
  /**
   * Whether the dropdown is currently visible.
   * Suggestions can exist without the dropdown being open — e.g. when the
   * user navigated to a previous command via history that happens to match
   * a keyword.  Arrow keys only route into the dropdown when isOpen is true.
   */
  isOpen: boolean;
  /** Index of the keyboard-highlighted suggestion (-1 = none). */
  activeIndex: number;
  /**
   * Call whenever the command text changes (i.e. from a real keystroke /
   * onChange event).  Opens the dropdown and resets the cursor.
   */
  onCommandChange: (value: string) => void;
  /** Call on keydown in the input — handles Tab / ArrowUp / ArrowDown / Escape. */
  onKeyDown: (
    e: React.KeyboardEvent<HTMLElement>,
    setCommand: (v: string) => void,
    onToggleMultiline?: (force?: boolean) => void,
  ) => boolean; // returns true if the event was consumed
  /** Accept a specific suggestion by index. */
  accept: (
    index: number,
    setCommand: (v: string) => void,
    onToggleMultiline?: (force?: boolean) => void,
  ) => void;
  /** Explicitly close the dropdown without changing the command text. */
  dismiss: () => void;
}

export function useAutocomplete(command: string): UseAutocompleteReturn {
  // Whether the dropdown is open (user is actively typing, not browsing history).
  const [isOpen, setIsOpen] = useState(false);
  const [activeIndex, setActiveIndex] = useState(-1);

  // Recompute suggestions whenever the command changes.
  // Note: suggestions may be non-empty even when isOpen is false (e.g. after
  // history navigation loads a command that matches a keyword).  The two are
  // intentionally independent.
  const suggestions = useMemo(() => getSuggestions(command), [command]);

  /**
   * Must be called on every real change event (user typing).
   * Opens the dropdown and resets the cursor position.
   */
  const onCommandChange = useCallback((_value: string) => {
    setIsOpen(true);
    setActiveIndex(-1);
  }, []);

  const dismiss = useCallback(() => {
    setIsOpen(false);
    setActiveIndex(-1);
  }, []);

  const accept = useCallback(
    (
      index: number,
      setCommand: (v: string) => void,
      onToggleMultiline?: (force?: boolean) => void,
    ) => {
      const suggestion = suggestions[index];
      if (!suggestion) return;
      setCommand(suggestion.template);
      setIsOpen(false);
      setActiveIndex(-1);
      if (suggestion.multiline && onToggleMultiline) {
        onToggleMultiline(true);
      }
    },
    [suggestions],
  );

  const onKeyDown = useCallback(
    (
      e: React.KeyboardEvent<HTMLElement>,
      setCommand: (v: string) => void,
      onToggleMultiline?: (force?: boolean) => void,
    ): boolean => {
      switch (e.key) {
        case 'ArrowDown': {
          // Only intercept arrow keys when the dropdown is open.
          if (!isOpen || suggestions.length === 0) return false;
          e.preventDefault();
          setActiveIndex(i => (i + 1) % suggestions.length);
          return true;
        }
        case 'ArrowUp': {
          // Only intercept arrow keys when the dropdown is open.
          if (!isOpen || suggestions.length === 0) return false;
          e.preventDefault();
          setActiveIndex(i => (i <= 0 ? suggestions.length - 1 : i - 1));
          return true;
        }
        case 'Tab': {
          // Tab opens the dropdown (if there are suggestions) and accepts.
          if (suggestions.length === 0) return false;
          e.preventDefault();
          const idx = activeIndex >= 0 ? activeIndex : 0;
          accept(idx, setCommand, onToggleMultiline);
          return true;
        }
        case 'Escape': {
          if (!isOpen) return false;
          dismiss();
          return true;
        }
        case 'Enter': {
          // Only intercept Enter to accept a highlighted row.
          if (!isOpen || activeIndex < 0) return false;
          e.preventDefault();
          accept(activeIndex, setCommand, onToggleMultiline);
          return true;
        }
        default:
          return false;
      }
    },
    [isOpen, suggestions, activeIndex, accept, dismiss],
  );

  return { suggestions, isOpen, activeIndex, onCommandChange, onKeyDown, accept, dismiss };
}
