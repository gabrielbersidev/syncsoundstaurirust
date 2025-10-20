// src-tauri/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio_capture;

use audio_capture::{AudioCapture, spawn_audio_capture_thread};
use std::sync::{Arc, Mutex};
use tauri::State;

// Estado global da aplicação
struct AppState {
    audio_receiver: Arc<Mutex<Option<Arc<Mutex<std::sync::mpsc::Receiver<Vec<f32>>>>>>>,
    selected_device: Arc<Mutex<Option<String>>>,
}

// ===========================================
// Comandos Tauri (expostos ao frontend)
// ===========================================

#[tauri::command]
fn list_audio_devices() -> Result<Vec<String>, String> {
    AudioCapture::list_input_devices()
}

#[tauri::command]
fn start_audio_capture(
    device_name: Option<String>,
    state: State<AppState>,
) -> Result<String, String> {
    // Inicia thread de captura
    let (receiver, _handle) = spawn_audio_capture_thread(device_name.clone())?;
    
    // Armazena receiver no estado
    let mut audio_rx = state.audio_receiver.lock().unwrap();
    *audio_rx = Some(receiver);
    
    // Atualiza dispositivo selecionado
    let mut device = state.selected_device.lock().unwrap();
    *device = device_name.clone();
    
    Ok(format!("Captura iniciada: {}", 
        device_name.unwrap_or_else(|| "dispositivo padrão".to_string())))
}

#[tauri::command]
fn stop_audio_capture(state: State<AppState>) -> Result<String, String> {
    let mut audio_rx = state.audio_receiver.lock().unwrap();
    *audio_rx = None;
    
    Ok("Captura parada".to_string())
}

#[tauri::command]
fn get_current_device(state: State<AppState>) -> Option<String> {
    let device = state.selected_device.lock().unwrap();
    device.clone()
}

// Comando de teste para verificar se áudio está chegando
#[tauri::command]
fn check_audio_signal(state: State<AppState>) -> Result<String, String> {
    let audio_rx_opt = state.audio_receiver.lock().unwrap();
    
    if let Some(receiver) = audio_rx_opt.as_ref() {
        let rx = receiver.lock().unwrap();
        let count = rx.try_iter().count();
        
        if count > 0 {
            Ok(format!("✓ Sinal de áudio OK ({} buffers)", count))
        } else {
            Ok("⚠ Nenhum buffer recebido ainda".to_string())
        }
    } else {
        Err("Captura não iniciada".to_string())
    }
}

// ===========================================
// Main
// ===========================================

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            audio_receiver: Arc::new(Mutex::new(None)),
            selected_device: Arc::new(Mutex::new(None)),
        })
        .invoke_handler(tauri::generate_handler![
            list_audio_devices,
            start_audio_capture,
            stop_audio_capture,
            get_current_device,
            check_audio_signal,
        ])
        .run(tauri::generate_context!())
        .expect("Erro ao executar aplicação Tauri");
}