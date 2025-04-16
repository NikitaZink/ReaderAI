import { useState } from 'react';

function App() {
  const [file, setFile] = useState<string | null>(null);

  return (
    <div className="container">
      <header>
        <h1>ReaderAI</h1>
        <p>Интеллектуальное чтение и управление заметками</p>
      </header>

      <main>
        {!file ? (
          <div className="welcome">
            <h2>Добро пожаловать в ReaderAI</h2>
            <p>
              ReaderAI поможет вам лучше понимать и структурировать информацию при чтении.
              Используйте AI для создания заметок, пояснения сложных абзацев и
              организации полученных знаний.
            </p>
            <div className="actions">
              <button className="primary-button">Открыть документ</button>
              <button className="secondary-button">Создать новый документ</button>
            </div>
          </div>
        ) : (
          <div className="reader">
            {/* Здесь будет интерфейс чтения */}
            <p>Контент документа будет отображаться здесь</p>
          </div>
        )}
      </main>

      <footer>
        <p>ReaderAI v0.1.0</p>
      </footer>
    </div>
  );
}

export default App; 