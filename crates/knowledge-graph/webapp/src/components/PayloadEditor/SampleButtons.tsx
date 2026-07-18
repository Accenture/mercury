import { SAMPLE_DATA } from '../../config/playgrounds';
import styles from './PayloadEditor.module.css';

interface SampleButtonsProps {
  onLoad: (value: string) => void;
}

export default function SampleButtons({ onLoad }: SampleButtonsProps) {
  const jsonSamples = Object.keys(SAMPLE_DATA).filter((k) => k.startsWith('json_'));
  const xmlSamples  = Object.keys(SAMPLE_DATA).filter((k) => k.startsWith('xml_'));
  const formatLabel = (key: string): string => key.replace(/^(json|xml)_/, '').replace(/_/g, ' ');

  return (
    <div className={styles.sampleButtons}>
      <span className={styles.sampleLabel}>Quick load:</span>

      <div className={styles.sampleGroup}>
        <span className={styles.sampleGroupLabel}>JSON:</span>
        {jsonSamples.map((key) => (
          <button key={key} className={styles.sampleButton} onClick={() => onLoad(SAMPLE_DATA[key])}>
            {formatLabel(key)}
          </button>
        ))}
      </div>

      <div className={styles.sampleGroup}>
        <span className={styles.sampleGroupLabel}>XML:</span>
        {xmlSamples.map((key) => (
          <button key={key} className={styles.sampleButton} onClick={() => onLoad(SAMPLE_DATA[key])}>
            {formatLabel(key)}
          </button>
        ))}
      </div>
    </div>
  );
}
