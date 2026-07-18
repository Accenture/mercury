import { Fragment } from 'react';
import { getMinigraphNodeTypeMeta } from '../../utils/minigraphNodeTheme';
import styles from './NodeTypes.module.css';

interface MinigraphNodeBodyProps {
  alias: string;
  nodeType: string;
  properties: Record<string, unknown>;
}

function Row({ label, value }: { label: string; value: string }) {
  return (
    <div className={styles.row}>
      <span className={styles.label}>{label}</span>
      <span className={styles.value} title={value}>{value}</span>
    </div>
  );
}

function PropertyRows({ properties }: { properties: Record<string, unknown> }) {
  const entries = Object.entries(properties).filter(([, value]) => value !== undefined && value !== null);
  if (entries.length === 0) return null;

  return (
    <>
      {entries.map(([key, value]) => {
        if (Array.isArray(value)) {
          return value.map((item, index) => {
            const renderedValue = typeof item === 'string' ? item : JSON.stringify(item);
            return (
              <Row
                key={`${key}-${index}`}
                label={index === 0 ? key : ''}
                value={renderedValue}
              />
            );
          });
        }

        const renderedValue = typeof value === 'string' ? value : JSON.stringify(value);
        return <Row key={key} label={key} value={renderedValue} />;
      })}
    </>
  );
}

export function MinigraphNodeBody({ alias, nodeType, properties }: MinigraphNodeBodyProps) {
  const meta = getMinigraphNodeTypeMeta(nodeType);

  return (
    <Fragment>
      <div className={styles.content}>
        <div className={styles.header}>
          <span className={styles.icon}>{meta.icon}</span>
          <span className={styles.alias}>{alias}</span>
          <span className={styles.badge}>{meta.label}</span>
        </div>

        <div className={styles.body}>
          <PropertyRows properties={properties} />
        </div>
      </div>
    </Fragment>
  );
}