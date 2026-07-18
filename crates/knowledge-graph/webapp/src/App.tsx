import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import Playground from './components/Playground';
import { PLAYGROUND_CONFIGS } from './config/playgrounds';
import { WebSocketProvider } from './contexts/WebSocketContext';
import { ClipboardProvider } from './contexts/ClipboardContext';

/**
 * To add a new playground tool, edit src/config/playgrounds.ts only —
 * no changes needed here. Routes are generated automatically from the config.
 */
export default function App() {
  const defaultPath = PLAYGROUND_CONFIGS[0].path;

  return (
    // WebSocketProvider lives *outside* BrowserRouter's Routes so that
    // connections survive tab navigation — switching playgrounds no longer
    // closes the active socket.
    <WebSocketProvider>
      <ClipboardProvider>
        <BrowserRouter>
          <Routes>
            {/* One route per configured playground */}
            {PLAYGROUND_CONFIGS.map((cfg) => (
              <Route key={cfg.path} path={cfg.path} element={<Playground key={cfg.path} config={cfg} />} />
            ))}

            {/* Fallback: unknown routes go to the first playground */}
            <Route path="*" element={<Navigate to={defaultPath} replace />} />
          </Routes>
        </BrowserRouter>
      </ClipboardProvider>
    </WebSocketProvider>
  );
}
