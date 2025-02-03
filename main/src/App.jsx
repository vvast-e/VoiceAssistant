import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

function App() {
    const [status, setStatus] = useState('Нажмите кнопку, чтобы начать запись');
    const [generalListeningActive, setGeneralListeningActive] = useState(false);

    const toggleGeneralListening = async () => {
        try {
            const response = await invoke('toggle_general_listening');
            setStatus(response);
            setGeneralListeningActive(!generalListeningActive);
        } catch (error) {
            setStatus('Ошибка при переключении общего потока: ' + error);
        }
    };

    const activateCommandListening = async () => {
        if (!generalListeningActive) {
            setStatus('Общий поток не активен. Сначала включите общий поток.');
            return;
        }

        setStatus('Активировано прослушивание команд. Говорите команды...');
        try {
            const response = await invoke('activate_command_listening');
            setStatus(response);
        } catch (error) {
            setStatus('Ошибка при активации прослушивания команд: ' + error);
        }
    };

    return (
        <div className="App">
            <h1>Голосовой помощник</h1>
            <button onClick={toggleGeneralListening}>
                {generalListeningActive ? 'Выключить общий поток' : 'Включить общий поток'}
            </button>
            <button onClick={activateCommandListening} disabled={!generalListeningActive}>
                Активировать прослушивание команд
            </button>
            <p>{status}</p>
        </div>
    );
}

export default App;