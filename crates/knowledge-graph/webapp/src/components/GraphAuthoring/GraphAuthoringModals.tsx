import NodeDialog from '../NodeDialog/NodeDialog';
import type { NodeFormState } from '../../graphActions/nodeAuthoringTypes';
import type { AuthoringState } from './useGraphAuthoring';

interface GraphAuthoringModalsProps {
  state: AuthoringState;
  validationErrors: Record<string, string>;
  onFormStateChange: (formState: NodeFormState) => void;
  onSubmit: () => void;
  onClose: () => void;
}

export default function GraphAuthoringModals({
  state,
  validationErrors,
  onFormStateChange,
  onSubmit,
  onClose,
}: GraphAuthoringModalsProps) {
  if (state.status === 'closed') return null;

  const lockReason =
    state.phase === 'sending'
      ? 'sending'
      : state.connectionLost
        ? 'disconnected'
        : null;

  return (
    <NodeDialog
      open
      mode={state.action === 'edit-node' ? 'edit' : 'create'}
      aliasReadOnly={state.action === 'edit-node'}
      formState={state.formState}
      phase={state.phase}
      lockReason={lockReason}
      serverMessage={state.serverMessage}
      validationErrors={validationErrors}
      onFormStateChange={onFormStateChange}
      onSubmit={onSubmit}
      onClose={onClose}
    />
  );
}
