export type NodeFormSource = 'empty-graph' | 'pane-context-menu' | 'edit-node';
export type NodeAction = 'create-node' | 'edit-node' | 'delete-node';

export interface PropertyRow {
  id: string;
  key: string;
  value: string;
}

export interface NodeFormState {
  alias: string;
  nodeType: string;
  properties: PropertyRow[];
  source: NodeFormSource;
}

export type NodeFormValidationErrors = Record<string, string>;

export interface NodeFormConversionResult {
  valid: boolean;
  formState: NodeFormState | null;
  message: string | null;
}
