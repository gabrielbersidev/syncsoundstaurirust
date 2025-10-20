// src-tauri/audio_capture.rs
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Stream, StreamConfig, SampleRate};
use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;

const SAMPLE_RATE: u32 = 44100;
const BUFFER_SIZE: usize = 2048; // ~46ms de latência a 44.1kHz
const CHANNELS: u16 = 1; // Mono para BPM detection

#[derive(Debug, Clone)]
pub struct AudioConfig {
    pub sample_rate: u32,
    pub channels: u16,
    pub buffer_size: usize,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            sample_rate: SAMPLE_RATE,
            channels: CHANNELS,
            buffer_size: BUFFER_SIZE,
        }
    }
}

pub struct AudioCapture {
    stream: Option<Stream>,
    audio_tx: Sender<Vec<f32>>,
    audio_rx: Arc<Mutex<Receiver<Vec<f32>>>>,
    config: AudioConfig,
}

impl AudioCapture {
    pub fn new() -> Result<Self, String> {
        let (audio_tx, audio_rx) = mpsc::channel();
        
        Ok(Self {
            stream: None,
            audio_tx,
            audio_rx: Arc::new(Mutex::new(audio_rx)),
            config: AudioConfig::default(),
        })
    }

    /// Lista todos os dispositivos de entrada disponíveis
    pub fn list_input_devices() -> Result<Vec<String>, String> {
        let host = cpal::default_host();
        let devices = host.input_devices()
            .map_err(|e| format!("Erro ao listar dispositivos: {}", e))?;

        let device_names: Vec<String> = devices
            .filter_map(|d| d.name().ok())
            .collect();

        if device_names.is_empty() {
            return Err("Nenhum dispositivo de entrada encontrado".to_string());
        }

        Ok(device_names)
    }

    /// Seleciona dispositivo por nome (ex: "Karsect UPC")
    pub fn select_device(&self, device_name: Option<&str>) -> Result<Device, String> {
        let host = cpal::default_host();

        let device = if let Some(name) = device_name {
            // Procura dispositivo específico
            host.input_devices()
                .map_err(|e| format!("Erro ao listar dispositivos: {}", e))?
                .find(|d| {
                    d.name()
                        .map(|n| n.to_lowercase().contains(&name.to_lowercase()))
                        .unwrap_or(false)
                })
                .ok_or_else(|| format!("Dispositivo '{}' não encontrado", name))?
        } else {
            // Usa dispositivo padrão
            host.default_input_device()
                .ok_or_else(|| "Nenhum dispositivo de entrada padrão encontrado".to_string())?
        };

        println!("✓ Dispositivo selecionado: {}", device.name().unwrap_or_default());
        Ok(device)
    }

    /// Inicia captura de áudio
    pub fn start_capture(&mut self, device_name: Option<&str>) -> Result<(), String> {
        let device = self.select_device(device_name)?;

        // Configura stream de entrada
        let config = StreamConfig {
            channels: self.config.channels,
            sample_rate: SampleRate(self.config.sample_rate),
            buffer_size: cpal::BufferSize::Fixed(self.config.buffer_size as u32),
        };

        println!("✓ Configuração: {}Hz, {} canal(is), buffer {}",
            config.sample_rate.0, config.channels, self.config.buffer_size);

        let tx = self.audio_tx.clone();
        let mut buffer = Vec::with_capacity(self.config.buffer_size);

        // Callback para processar áudio
        let stream = device.build_input_stream(
            &config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                // Converte para mono se necessário (pega canal 1)
                for (i, &sample) in data.iter().enumerate() {
                    if i % config.channels as usize == 0 {
                        buffer.push(sample);
                    }

                    if buffer.len() >= BUFFER_SIZE {
                        // Envia buffer completo para processamento
                        if tx.send(buffer.clone()).is_err() {
                            eprintln!("⚠ Erro ao enviar buffer de áudio");
                        }
                        buffer.clear();
                    }
                }
            },
            |err| eprintln!("✗ Erro no stream de áudio: {}", err),
            None,
        ).map_err(|e| format!("Erro ao criar stream: {}", e))?;

        stream.play().map_err(|e| format!("Erro ao iniciar stream: {}", e))?;

        self.stream = Some(stream);
        println!("✓ Captura de áudio iniciada");

        Ok(())
    }

    /// Retorna receiver para consumir buffers de áudio
    pub fn get_audio_receiver(&self) -> Arc<Mutex<Receiver<Vec<f32>>>> {
        self.audio_rx.clone()
    }

    /// Para a captura de áudio
    pub fn stop_capture(&mut self) {
        if let Some(stream) = self.stream.take() {
            drop(stream);
            println!("✓ Captura de áudio parada");
        }
    }
}

impl Drop for AudioCapture {
    fn drop(&mut self) {
        self.stop_capture();
    }
}

// ===========================================
// Thread de processamento assíncrono
// ===========================================

/// Inicia thread separada para captura contínua
pub fn spawn_audio_capture_thread(
    device_name: Option<String>,
) -> Result<(Arc<Mutex<Receiver<Vec<f32>>>>, thread::JoinHandle<()>), String> {
    let mut capture = AudioCapture::new()?;
    let receiver = capture.get_audio_receiver();
    
    let handle = thread::spawn(move || {
        if let Err(e) = capture.start_capture(device_name.as_deref()) {
            eprintln!("✗ Erro ao iniciar captura: {}", e);
            return;
        }

        // Mantém thread viva
        loop {
            thread::sleep(std::time::Duration::from_millis(100));
        }
    });

    Ok((receiver, handle))
}

// ===========================================
// Testes
// ===========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_devices() {
        let devices = AudioCapture::list_input_devices();
        assert!(devices.is_ok());
        println!("Dispositivos encontrados: {:?}", devices.unwrap());
    }

    #[test]
    fn test_capture_basic() {
        let mut capture = AudioCapture::new().unwrap();
        let rx = capture.get_audio_receiver();
        
        capture.start_capture(None).unwrap();
        
        std::thread::sleep(std::time::Duration::from_secs(2));
        
        let rx = rx.lock().unwrap();
        let count = rx.try_iter().count();
        
        println!("Buffers capturados: {}", count);
        assert!(count > 0, "Deveria ter capturado pelo menos 1 buffer");
    }
}