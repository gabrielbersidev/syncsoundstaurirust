// src/App.tsx
import { useState } from 'react';
import AudioSetup from './components/AudioSetup';
import './App.css';

function App() {
  const [isAudioActive, setIsAudioActive] = useState(false);
  const [currentDevice, setCurrentDevice] = useState<string | null>(null);

  const handleDeviceSelected = (device: string | null) => {
    setCurrentDevice(device);
    setIsAudioActive(device !== null);
    
    if (device) {
      console.log('✓ Captura ativa:', device);
      // TODO: Iniciar processamento de BPM aqui (Etapa 2)
    } else {
      console.log('✗ Captura parada');
    }
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-950 via-slate-900 to-slate-950 p-8">
      <div className="max-w-4xl mx-auto space-y-8">
        {/* Header */}
        <header className="text-center space-y-2">
          <h1 className="text-5xl font-bold text-white tracking-tight">
            TempoSync Pro
          </h1>
          <p className="text-slate-400 text-lg">
            Monitor de BPM em Tempo Real
          </p>
        </header>

        {/* Status Global */}
        <div className="bg-slate-900/50 border border-slate-700 rounded-lg p-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              <div className={`w-3 h-3 rounded-full ${
                isAudioActive ? 'bg-green-500 animate-pulse' : 'bg-slate-600'
              }`}></div>
              <span className="text-white font-semibold">
                {isAudioActive ? 'Sistema Ativo' : 'Sistema Inativo'}
              </span>
            </div>
            {currentDevice && (
              <span className="text-sm text-slate-400">
                📡 {currentDevice}
              </span>
            )}
          </div>
        </div>

        {/* Configuração de Áudio */}
        <AudioSetup onDeviceSelected={handleDeviceSelected} />

        {/* Placeholder para BPM Display (Etapa 2) */}
        {isAudioActive && (
          <div className="bg-slate-900 border border-slate-700 rounded-lg p-8 text-center">
            <div className="space-y-4">
              <div className="text-slate-500">
                <svg 
                  className="w-24 h-24 mx-auto animate-pulse" 
                  fill="none" 
                  stroke="currentColor" 
                  viewBox="0 0 24 24"
                >
                  <path 
                    strokeLinecap="round" 
                    strokeLinejoin="round" 
                    strokeWidth={2} 
                    d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3" 
                  />
                </svg>
              </div>
              <p className="text-slate-400 text-lg">
                Aguardando detecção de BPM...
              </p>
              <p className="text-sm text-slate-500">
                O módulo de BPM será implementado na Etapa 2
              </p>
            </div>
          </div>
        )}

        {/* Footer com Informações */}
        <footer className="text-center text-slate-500 text-sm space-y-2">
          <p>
            Etapa 1/5 concluída: ✅ Captura de Áudio em Tempo Real
          </p>
          <p className="text-xs">
            Próxima: Detecção de BPM com Aubio
          </p>
        </footer>
      </div>
    </div>
  );
}

export default App;