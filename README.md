# 🥁 Real-Time BPM Sync for Live Bands

![Rust](https://img.shields.io/badge/rust-%23E32F26.svg?style=for-the-badge&logo=rust&logoColor=white)
![Tauri](https://img.shields.io/badge/tauri-%2324C8DB.svg?style=for-the-badge&logo=tauri&logoColor=FFFFFF)
![License](https://img.shields.io/badge/license-MIT-green.svg?style=for-the-badge)
![Status](https://img.shields.io/badge/status-in--development-yellow.svg?style=for-the-badge)

**Real-Time BPM Sync** é um sistema de alta performance projetado para detecção e correção de BPM em tempo real durante performances musicais ao vivo. Utilizando processamento de sinal de baixa latência, o projeto ajuda bandas que dependem de precisão rítmica absoluta para sincronizar playbacks, iluminação, timecode e loops eletrônicos.

---

## 📖 Sobre o Projeto

Manter a consistência rítmica em um ambiente de show ao vivo pode ser um desafio, especialmente quando elementos humanos interagem com sequências eletrônicas. Este sistema atua como um "vigia rítmico", capturando o áudio da banda, processando o tempo atual e fornecendo feedback instantâneo para garantir que o click track e os elementos digitais estejam sempre em fase com a performance orgânica.

### ✨ Principais Objetivos
* **Captura de Alta Fidelidade:** Monitoramento contínuo via interface de áudio.
* **Detecção Inteligente:** Algoritmos para identificar o BPM real da performance.
* **Comparação em Tempo Real:** Análise instantânea contra um BPM alvo (Target BPM).
* **Estabilidade:** Correção de tempo e feedback visual para o baterista/maestro.

---

## 🏗️ Arquitetura do Sistema

O projeto é construído sobre o ecossistema **Tauri**, garantindo segurança e performance nativa com uma interface moderna.

### Backend (Rust)
* **Gerenciamento de Áudio:** Utiliza a biblioteca `cpal` para acesso de baixo nível ao hardware.
* **Processamento de Sinal:** Buffers de áudio processados em threads dedicadas para evitar jitter.
* **Comunicação:** Sistema de comandos e eventos assíncronos via Tauri.

### Frontend (Tauri Framework)
* **Dashboard de Controle:** Seleção dinâmica de dispositivos e monitoramento de sinal.
* **Visualização:** Interface reativa para exibição de métricas e status do motor de áudio.

---

## 📁 Estrutura de Diretórios

O projeto segue a estrutura padrão do Tauri com módulos Rust organizados:

```text
bpm-sync-root/
|-- src/                    (Frontend - Interface Gráfica)
|   |-- components/         (Componentes UI)
|   |-- App.tsx             (Lógica principal da interface)
|
|-- src-tauri/              (Backend - Motor em Rust)
|   |-- src/
|   |   |-- audio_capture.rs (Módulo de captura de sinal)
|   |   |-- main.rs          (Entry point e comandos Tauri)
|   |-- Cargo.toml          (Dependências Rust)
|   |-- tauri.conf.json     (Configurações do app)
|
|-- public/                 (Assets estáticos)
|-- package.json            (Gerenciador Node.js)


## ⚙️ Configurações Técnicas de Áudio

Para garantir a precisão necessária em palcos, o motor de áudio opera sob os seguintes parâmetros:

| Parâmetro | Valor | Observação |
| :--- | :--- | :--- |
| **Sample Rate** | 44.1 kHz | Padrão de indústria para áudio digital |
| **Canais** | Mono | Focado em detecção de transientes de percussão |
| **Buffer Size** | 2048 samples | Equilíbrio entre estabilidade e resposta |
| **Latência Estimada** | ~46ms | Processamento otimizado para tempo real |

---

## 🚀 Comandos Tauri Disponíveis

O frontend comunica-se com o core em Rust através dos seguintes comandos:

* **`list_audio_devices`**: Retorna uma lista de todos os dispositivos de entrada disponíveis no sistema.
* **`start_audio_capture`**: Inicializa o fluxo de captura e o processamento de buffer.
* **`stop_audio_capture`**: Encerra a captura e libera os recursos da interface de áudio.
* **`get_current_device`**: Consulta qual dispositivo está atualmente configurado para captura.
* **`check_audio_signal`**: Retorna o status de integridade dos buffers recebidos.

---

## 🛠️ Instalação e Execução

### Pré-requisitos
* **Rust**: [Instalar Rust](https://www.rust-lang.org/tools/install)
* **Node.js**: [Instalar Node.js](https://nodejs.org/)
* **Dependências de Sistema**: Webview2 no Windows ou bibliotecas de áudio (`alsa`, `libudev`) no Linux.

### Passo a Passo

1.  **Clone o repositório:**
    ```bash
    git clone [https://github.com/seu-usuario/bpm-sync-project.git](https://github.com/seu-usuario/bpm-sync-project.git)
    cd bpm-sync-project
    ```

2.  **Instale as dependências do frontend:**
    ```bash
    npm install
    ```

3.  **Execute o projeto em modo de desenvolvimento:**
    ```bash
    npm run tauri dev
    ```

---

## 🗺️ Roadmap de Desenvolvimento

- [ ] **Fase 1**: Implementação do algoritmo de *Onset Detection*.
- [ ] **Fase 2**: Filtro Passa-Banda (*Band-pass*) focado em frequências de bumbo/caixa.
- [ ] **Fase 3**: Janela deslizante (*Sliding Window*) para estabilização de leitura.
- [ ] **Fase 4**: Integração com MIDI Clock e Ableton Link.
- [ ] **Fase 5**: Sistema de alertas visuais e exportação de métricas de performance pós-show.

---

## 🤝 Contribuição

Contribuições são o que fazem a comunidade open source um lugar incrível para aprender e criar.

1.  Faça um **Fork** do projeto.
2.  Crie uma **Branch** para sua feature (`git checkout -b feature/NovaFeature`).
3.  Dê um **Commit** em suas alterações (`git commit -m 'Add: Nova Feature'`).
4.  Faça um **Push** para a Branch (`git push origin feature/NovaFeature`).
5.  Abra um **Pull Request**.

---

## 📄 Licença

Distribuído sob a licença **MIT**. Veja o arquivo `LICENSE` para mais informações.