// src/components/AudioSetup.tsx
import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

interface AudioSetupProps {
  onDeviceSelected?: (device: string | null) => void;
}

export default function AudioSetup({ onDeviceSelected }: AudioSetupProps) {
  const [devices, setDevices] = useState<string[]>([]);
  const [selectedDevice, setSelectedDevice] = useState<string | null>(null);
  const [isCapturing, setIsCapturing] = useState(false);
  const [status, setStatus] = useState<string>('');
  const [error, setError] = useState<string>('');
  const [signalCheck, setSignalCheck] = useState<string>('');

  // Lista dispositivos ao montar
  useEffect(() => {
    loadDevices();
  }, []);

  const loadDevices = async () => {
    try {
      const deviceList = await invoke<string[]>('list_audio_devices');
      setDevices(deviceList);
      setError('');
      
      // Tenta encontrar Karsect automaticamente
      const karsect = deviceList.find(d => 
        d.toLowerCase().includes('karsect') || 
        d.toLowerCase().includes('upc')
      );
      
      if (karsect) {
        setSelectedDevice(karsect);
        setStatus(`✓ Karsect detectado: ${karsect}`);
      }
    } catch (err) {
      setError(`Erro ao listar dispositivos: ${err}`);
    }
  };

  const startCapture = async () => {
    try {
      const result = await invoke<string>('start_audio_capture', {
        deviceName: selectedDevice
      });
      setStatus(result);
      setIsCapturing(true);
      setError('');
      onDeviceSelected?.(selectedDevice);
      
      // Inicia verificação de sinal
      startSignalCheck();
    } catch (err) {
      setError(`Erro ao iniciar captura: ${err}`);
      setIsCapturing(false);
    }
  };

  const stopCapture = async () => {
    try {
      const result = await invoke<string>('stop_audio_capture');
      setStatus(result);
      setIsCapturing(false);
      setSignalCheck('');
      onDeviceSelected?.(null);
    } catch (err) {
      setError(`Erro ao parar captura: ${err}`);
    }
  };

  const startSignalCheck = () => {
    const interval = setInterval(async () => {
      try {
        const result = await invoke<string>('check_audio_signal');
        setSignalCheck(result);
      } catch (err) {
        setSignalCheck(`Erro: ${err}`);
      }
    }, 1000);

    // Cleanup após 10 segundos
    setTimeout(() => clearInterval(interval), 10000);
  };

  return (
    <div className="bg-slate-900 border border-slate-700 rounded-lg p-6 space-y-4">
      <div className="flex items-center justify-between">
        <h2 className="text-xl font-bold text-white">Configuração de Áudio</h2>
        <button
          onClick={loadDevices}
          className="px-3 py-1 text-sm bg-slate-700 hover:bg-slate-600 text-white rounded transition"
        >
          Atualizar
        </button>
      </div>

      {/* Seleção de dispositivo */}
      <div className="space-y-2">
        <label className="text-sm text-slate-300 font-medium">
          Dispositivo de Entrada (Interface de Áudio)
        </label>
        <select
          value={selectedDevice || ''}
          onChange={(e) => setSelectedDevice(e.target.value || null)}
          disabled={isCapturing}
          className="w-full bg-slate-800 border border-slate-600 text-white rounded px-3 py-2 disabled:opacity-50"
        >
          <option value="">Dispositivo Padrão</option>
          {devices.map((device, i) => (
            <option key={i} value={device}>
              {device}
            </option>
          ))}
        </select>
        {devices.length === 0 && (
          <p className="text-sm text-amber-400">
            Nenhum dispositivo encontrado. Conecte sua interface de áudio.
          </p>
        )}
      </div>

      {/* Controles */}
      <div className="flex gap-3">
        <button
          onClick={isCapturing ? stopCapture : startCapture}
          disabled={devices.length === 0}
          className={`flex-1 py-2 px-4 rounded font-semibold transition disabled:opacity-50 ${
            isCapturing
              ? 'bg-red-600 hover:bg-red-700 text-white'
              : 'bg-green-600 hover:bg-green-700 text-white'
          }`}
        >
          {isCapturing ? '⏹ Parar Captura' : '▶ Iniciar Captura'}
        </button>
      </div>

      {/* Status */}
      {status && (
        <div className="bg-slate-800 border border-slate-600 rounded p-3">
          <p className="text-sm text-green-400">{status}</p>
        </div>
      )}

      {/* Verificação de sinal */}
      {signalCheck && (
        <div className="bg-slate-800 border border-blue-600 rounded p-3">
          <p className="text-sm text-blue-400">{signalCheck}</p>
        </div>
      )}

      {/* Erro */}
      {error && (
        <div className="bg-red-900/20 border border-red-600 rounded p-3">
          <p className="text-sm text-red-400">{error}</p>
        </div>
      )}

      {/* Indicador visual */}
      {isCapturing && (
        <div className="flex items-center gap-2 text-green-400">
          <div className="w-2 h-2 bg-green-400 rounded-full animate-pulse"></div>
          <span className="text-sm font-medium">Capturando áudio...</span>
        </div>
      )}

      {/* Instruções */}
      <div className="bg-slate-800/50 rounded p-4 space-y-2 text-sm text-slate-300">
        <p className="font-semibold text-white">📌 Instruções:</p>
        <ul className="list-disc list-inside space-y-1 ml-2">
          <li>Conecte sua interface Karsect UPC ao computador</li>
          <li>Certifique-se que o canal 1 está recebendo sinal da mesa</li>
          <li>Selecione o dispositivo na lista acima</li>
          <li>Clique em "Iniciar Captura" para começar</li>
          <li>O áudio será processado automaticamente</li>
        </ul>
      </div>
    </div>
  );
}