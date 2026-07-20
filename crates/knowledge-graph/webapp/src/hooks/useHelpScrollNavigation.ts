import { useEffect, useRef } from 'react';

/** Accumulated excess deltaY (px) required to flip the page. */
export const FLIP_THRESHOLD_PX = 120;

/**
 * Maximum visual translateY displacement applied to contentWrapperRef during
 * the rubber-band gesture (px).
 * Note: VISUAL_MAX_PX / FLIP_THRESHOLD_PX = VISUAL_FACTOR.
 */
export const VISUAL_MAX_PX = 18;

/**
 * Milliseconds with no wheel events before the accumulated charge is released
 * without navigating (rubber-band snap-back).
 */
export const WHEEL_STOP_DURATION_MS = 180;

/**
 * Milliseconds to ignore wheel events after a page flip, preventing double-flips
 * from trackpad kinetic scroll momentum.
 */
export const POST_NAVIGATION_COOLDOWN_MS = 650;

export interface UseHelpScrollNavigationOptions {
  /** Ref to the outer container — wheel listener attaches here (fills full panel height). */
  wheelTargetRef:    React.RefObject<HTMLDivElement | null>;
  /** Ref to the inner scroll container — scrollTop/scrollHeight reads come from here. */
  scrollRef:         React.RefObject<HTMLDivElement | null>;
  /** Ref to the inner content wrapper (`.helpBodyContent`) — rubber-band target. */
  contentWrapperRef: React.RefObject<HTMLDivElement | null>;
  /** Index of the currently displayed page in the ORDERED_HELP_PAGES sequence. */
  currentIndex:      number;
  /** Total number of pages (18). */
  totalPages:        number;
  /** Called when a prev-page flip is triggered. Caller must guard index > 0. */
  onNavigatePrev:    () => void;
  /** Called when a next-page flip is triggered. Caller must guard index < N-1. */
  onNavigateNext:    () => void;
}

export function useHelpScrollNavigation(
  options: UseHelpScrollNavigationOptions,
): void {
  const {
    wheelTargetRef,
    scrollRef,
    contentWrapperRef,
    currentIndex,
    totalPages,
    onNavigatePrev,
    onNavigateNext,
  } = options;

  // ── Refs that drive logic without triggering re-renders ───────────────────
  const overscrollAccumRef  = useRef<number>(0);
  const chargeDirectionRef  = useRef<'prev' | 'next' | null>(null);
  const isNavigatingRef     = useRef<boolean>(false);
  const wheelStopTimerRef   = useRef<ReturnType<typeof setTimeout> | null>(null);

  // ── Stable callback refs (avoid stale closures in the fixed effect) ───────
  const onNavigatePrevRef = useRef(onNavigatePrev);
  const onNavigateNextRef = useRef(onNavigateNext);
  const currentIndexRef   = useRef(currentIndex);
  const totalPagesRef     = useRef(totalPages);

  useEffect(() => { onNavigatePrevRef.current = onNavigatePrev; });
  useEffect(() => { onNavigateNextRef.current = onNavigateNext; });
  useEffect(() => { currentIndexRef.current   = currentIndex;   });
  useEffect(() => { totalPagesRef.current      = totalPages;     });

  // ── Reset on page change (external navigation or server command) ──────────
  // C4: clear any pending wheel-stop timer so it cannot fire resetCharge redundantly
  // after the page has already changed.
  useEffect(() => {
    if (wheelStopTimerRef.current !== null) {
      clearTimeout(wheelStopTimerRef.current);
      wheelStopTimerRef.current = null;
    }
    // Snap back immediately (no transition — new page is already visible)
    if (contentWrapperRef.current) {
      contentWrapperRef.current.style.transition = 'none';
      contentWrapperRef.current.style.transform  = 'translateY(0)';
    }
    overscrollAccumRef.current  = 0;
    chargeDirectionRef.current  = null;
  // C2: contentWrapperRef and wheelStopTimerRef are stable ref objects; do not
  // include in deps. currentIndex is the only meaningful dependency here.
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [currentIndex]);

  // ── Mount/unmount effect — attach wheel listener ──────────────────────────
  useEffect(() => {
    // C1: capture wheelTargetRef.current at effect level, not inside handleWheel.
    const target = wheelTargetRef.current;
    if (!target) return;

    function resetCharge(): void {
      overscrollAccumRef.current = 0;
      chargeDirectionRef.current = null;
      // Animate the snap-back via CSS spring transition
      if (contentWrapperRef.current) {
        contentWrapperRef.current.style.transition =
          'transform 0.28s cubic-bezier(0.25, 0.46, 0.45, 0.94)';
        contentWrapperRef.current.style.transform  = 'translateY(0)';
      }
    }

    function handleWheel(event: WheelEvent): void {
      if (event.deltaY === 0) return; // horizontal-only event; ignore

      // scrollRef is the ONLY ref read here — for scroll-boundary metrics.
      const scrollEl = scrollRef.current;
      if (!scrollEl) return;

      const atTop    = scrollEl.scrollTop <= 0;
      const atBottom =
        scrollEl.scrollTop + scrollEl.clientHeight >= scrollEl.scrollHeight - 1;

      const goingUp   = event.deltaY < 0;
      const goingDown = event.deltaY > 0;

      const isOverscrollingPrev = atTop    && goingUp;
      const isOverscrollingNext = atBottom && goingDown;

      // ── Normal (mid-scroll) event: reset any accumulated charge ──────────
      if (!isOverscrollingPrev && !isOverscrollingNext) {
        resetCharge();
        return;
      }

      // ── Post-navigation cooldown: ignore events ───────────────────────────
      if (isNavigatingRef.current) return;

      // ── At boundary: check whether navigation is possible ────────────────
      const idx   = currentIndexRef.current;
      const total = totalPagesRef.current;
      if (isOverscrollingPrev && idx === 0)         return; // already at first page
      if (isOverscrollingNext && idx === total - 1) return; // already at last page

      // ── Accumulate overscroll ─────────────────────────────────────────────
      const incomingDir: 'prev' | 'next' = isOverscrollingPrev ? 'prev' : 'next';

      // Direction reversal mid-gesture: treat as a new gesture
      if (
        chargeDirectionRef.current !== null &&
        chargeDirectionRef.current !== incomingDir
      ) {
        resetCharge();
      }

      chargeDirectionRef.current      = incomingDir;
      overscrollAccumRef.current     += Math.abs(event.deltaY);

      // ── Apply rubber-band visual displacement ─────────────────────────────
      if (contentWrapperRef.current) {
        const sign           = incomingDir === 'prev' ? -1 : 1;
        const rawTranslate   = overscrollAccumRef.current * (VISUAL_MAX_PX / FLIP_THRESHOLD_PX);
        const clampedTranslate = Math.min(rawTranslate, VISUAL_MAX_PX) * sign;
        contentWrapperRef.current.style.transition = 'none';
        contentWrapperRef.current.style.transform  = `translateY(${clampedTranslate}px)`;
      }

      // ── Reset wheel-stop timer ────────────────────────────────────────────
      // C3: guard before clearTimeout — null is not assignable to number|undefined
      if (wheelStopTimerRef.current !== null) clearTimeout(wheelStopTimerRef.current);
      wheelStopTimerRef.current = setTimeout(resetCharge, WHEEL_STOP_DURATION_MS);

      // ── Threshold reached: flip the page ─────────────────────────────────
      if (overscrollAccumRef.current >= FLIP_THRESHOLD_PX) {
        if (wheelStopTimerRef.current !== null) clearTimeout(wheelStopTimerRef.current);
        const flippedDir = chargeDirectionRef.current;
        resetCharge(); // snap back first, then navigate
        isNavigatingRef.current = true;
        if (flippedDir === 'prev') {
          onNavigatePrevRef.current();
        } else {
          onNavigateNextRef.current();
        }
        setTimeout(() => { isNavigatingRef.current = false; }, POST_NAVIGATION_COOLDOWN_MS);
      }
    }

    target.addEventListener('wheel', handleWheel, { passive: true });
    return () => {
      // M6: clear timer on unmount to prevent setState on unmounted component
      if (wheelStopTimerRef.current !== null) clearTimeout(wheelStopTimerRef.current);
      // Note: omitting { passive: true } from removeEventListener is intentional and
      // correct. Only the `capture` option is part of a listener's identity per the
      // Web spec; `passive` is not.
      target.removeEventListener('wheel', handleWheel);
    };
  // Mount/unmount only — all mutable dependencies are accessed via stable refs
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

}
