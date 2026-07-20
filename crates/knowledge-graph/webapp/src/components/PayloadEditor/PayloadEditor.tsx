import SampleButtons from './SampleButtons';
import styles from './PayloadEditor.module.css';
import { type ValidationResult } from '../../utils/validators';

interface PayloadEditorProps {
  payload:    string;
  onChange:   (value: string) => void;
  validation: ValidationResult;
  onFormat:   () => void;
  onUpload?:  () => void;
}

export default function PayloadEditor({ payload, onChange, validation, onFormat, onUpload }: PayloadEditorProps) {
  return (
    <div className={styles.payloadRoot}>
      <div className={styles.labelRow}>
        <label htmlFor="payload" className={styles.label}>JSON/XML Payload</label>
        <div className={styles.payloadControls}>
          <span className={styles.charCounter}>size: {payload.length}</span>
          {payload && validation.type && (
            <span className={styles.typeIndicator}>{validation.type.toUpperCase()}</span>
          )}
          {payload && (
            <span className={styles.validationIcon}>{validation.valid ? '✅' : '❌'}</span>
          )}
          <button
            className={styles.formatButton}
            onClick={onFormat}
            disabled={!payload || validation.type !== 'json'}
            title={validation.type === 'xml' ? 'Format only available for JSON' : 'Format JSON'}
          >
            Format
          </button>
          {onUpload !== undefined && (
            <button
              className={styles.uploadButton}
              onClick={onUpload}
              disabled={!payload || !validation.valid || validation.type !== 'json'}
              title="Upload JSON payload to current session via REST"
            >
              Upload
            </button>
          )}
        </div>
      </div>

      <textarea
        id="payload"
        className={`${styles.textarea} ${validation.valid ? '' : styles.textareaError}`}
        placeholder="Paste your JSON/XML payload here"
        value={payload}
        onChange={(e) => onChange(e.target.value)}
      />

      {!validation.valid && (
        <div className={styles.errorMessage}>{validation.error}</div>
      )}

      <div className={styles.sampleButtonsRow}>
        <SampleButtons onLoad={onChange} />
      </div>
    </div>
  );
}

