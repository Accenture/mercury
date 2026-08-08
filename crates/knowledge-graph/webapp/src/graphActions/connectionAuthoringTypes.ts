export interface ConnectionFormState {
  sourceAlias: string;
  targetAlias: string;
  /** Free-text relation name (validated for command-safe token format, not a fixed list). */
  relation: string;
}

export type ConnectionFormValidationErrors = Record<string, string>;
