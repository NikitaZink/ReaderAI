pub const WINDOW_STYLES: &str = r#"
:root {
  --bg: #FBF9F7;
  --panel-bg: #FFFFFF;
  --text: #17202A;
  --accent-1: #70B3FF;
  --accent-2: #FF8A70;
  --border: rgba(0,0,0,0.08);
  --radius: 12px;
  --gap: 16px;
  --window-border-radius: 8px;
}

* { box-sizing: border-box; margin: 0; padding: 0; }

body {
  background: var(--bg);
  color: var(--text);
  font-family: Inter, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, sans-serif;
  height: 100vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  border-radius: var(--window-border-radius);
}

/* Стили для макета приложения */
header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--gap);
  background: var(--panel-bg);
  box-shadow: 0 1px 4px var(--border);
}

header .search {
  flex: 1;
  max-width: 400px;
}

header .search input {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid var(--border);
  border-radius: var(--radius);
  outline: none;
}

header .icons {
  display: flex;
  align-items: center;
}

header .icons > * {
  margin-left: var(--gap);
  cursor: pointer;
  font-size: 1.2rem;
}

.content {
  flex: 1;
  display: flex;
  overflow: hidden;
}

.sidebar, .notes {
  width: 240px;
  padding: var(--gap);
  background: var(--panel-bg);
  overflow-y: auto;
}

.sidebar {
  border-right: 1px solid var(--border);
}

.notes {
  border-left: 1px solid var(--border);
}

.sidebar h2, .notes h2 {
  font-size: 1.2rem;
  margin-bottom: var(--gap);
  font-weight: 600;
}

.sidebar .doc, .notes .note {
  background: var(--bg);
  margin-bottom: var(--gap);
  padding: 12px;
  border-radius: var(--radius);
  box-shadow: 0 1px 3px var(--border);
  cursor: pointer;
}

.sidebar .doc.active {
  background: var(--accent-1);
  color: #fff;
}

main.reader {
  flex: 1;
  padding: var(--gap);
  overflow-y: auto;
  position: relative;
}

main.reader h1 {
  font-size: 1.5rem;
  margin-bottom: var(--gap);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

main.reader .paragraph {
  margin-bottom: var(--gap);
  line-height: 1.6;
}

footer {
  padding: var(--gap);
  background: var(--panel-bg);
  box-shadow: 0 -1px 4px var(--border);
  display: flex;
  align-items: center;
}

footer .progress {
  flex: 1;
  height: 4px;
  background: var(--border);
  border-radius: 2px;
  overflow: hidden;
  margin-right: var(--gap);
}

footer .progress div {
  width: 40%;
  height: 100%;
  background: var(--accent-1);
}

/* Тулбар для выделенного текста */
#selection-toolbar {
  position: absolute;
  background: var(--panel-bg);
  border-radius: 8px;
  box-shadow: 0 2px 10px rgba(0,0,0,0.15);
  padding: 8px;
  display: flex;
  gap: 8px;
  z-index: 100;
}

#selection-toolbar button {
  border: none;
  background: none;
  cursor: pointer;
  font-size: 16px;
  padding: 4px;
  border-radius: 4px;
}

#selection-toolbar button:hover {
  background: var(--border);
}

/* Стили для управления Tauri окном */
.titlebar-drag-region {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 30px;
  -webkit-app-region: drag;
  app-region: drag;
  z-index: 9999;
}

.window-controls {
  position: absolute;
  top: 8px;
  right: 8px;
  display: flex;
  gap: 6px;
  z-index: 10000;
  -webkit-app-region: no-drag;
  app-region: no-drag;
}

.window-controls button {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  border: none;
  cursor: pointer;
}

.window-controls .minimize {
  background: #FFBD44;
}

.window-controls .maximize {
  background: #00CA4E;
}

.window-controls .close {
  background: #FF605C;
}

.window-controls button:hover {
  filter: brightness(0.9);
}
"#;

pub const TITLEBAR_SCRIPT: &str = r#"
// Немедленно выполняемая функция для инициализации приложения
(function() {
  // Убедимся, что DOM готов
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initializeApp);
  } else {
    initializeApp();
  }
  
  // Функция инициализации
  function initializeApp() {
    // Добавляем область для драга окна и контролы окна
    setupWindowControls();
    
    // Создаем разметку в соответствии с примером HTML
    document.body.innerHTML += `
      <header>
        <div class="search">
          <input type="text" placeholder="Search..." />
        </div>
        <div class="icons">
          <span>🔍</span>
          <span>💬</span>
          <span>⚙️</span>
        </div>
      </header>

      <div class="content">
        <aside class="sidebar">
          <h2>Library</h2>
          <div class="doc active">Document 1</div>
          <div class="doc">Document 2</div>
          <div class="doc">Document 3</div>
        </aside>

        <main class="reader">
          <h1>Document Title <small style="color:#555; font-size:0.8rem; margin-left:auto;">Page 1 of 5</small></h1>
          <div class="paragraph">1. Lorem ipsum dolor sit amet, consectetur adipiscing elit.</div>
          <div class="paragraph">2. Quisque elementum urna vel tortor cursus, a volutpat mi egestas.</div>
          <div class="paragraph">3. Curabitur vitae enim eu purus cursus convallis.</div>
        </main>

        <aside class="notes">
          <h2>Notes</h2>
          <div class="note">Note 1: key ideas...</div>
          <div class="note">Note 2: AI explanation...</div>
        </aside>
      </div>

      <footer>
        <div class="progress"><div></div></div>
        <div>🅰️</div>
      </footer>
    `;
    
    // Настраиваем обработчики событий
    setupEventHandlers();
    
    // Настраиваем выделение текста
    setupTextSelection();
    
    // Настраиваем обработчик меню
    setupMenuHandler();
  }

  // Создание элементов управления окном
  function setupWindowControls() {
    // Область для перетаскивания окна
    const dragRegion = document.createElement('div');
    dragRegion.className = 'titlebar-drag-region';
    document.body.appendChild(dragRegion);
    
    // Кнопки управления окном
    const windowControls = document.createElement('div');
    windowControls.className = 'window-controls';
    
    const minimize = document.createElement('button');
    minimize.className = 'minimize';
    minimize.title = 'Свернуть';
    minimize.addEventListener('click', () => window.__TAURI__.window.appWindow.minimize());
    
    const maximize = document.createElement('button');
    maximize.className = 'maximize';
    maximize.title = 'Развернуть';
    maximize.addEventListener('click', () => window.__TAURI__.window.appWindow.toggleMaximize());
    
    const close = document.createElement('button');
    close.className = 'close';
    close.title = 'Закрыть';
    close.addEventListener('click', () => window.__TAURI__.window.appWindow.close());
    
    windowControls.appendChild(minimize);
    windowControls.appendChild(maximize);
    windowControls.appendChild(close);
    
    document.body.appendChild(windowControls);
  }

  // Настройка обработчиков событий
  function setupEventHandlers() {
    // Переключение активного документа
    document.querySelectorAll('.doc').forEach(doc => {
      doc.addEventListener('click', function() {
        document.querySelectorAll('.doc').forEach(d => d.classList.remove('active'));
        this.classList.add('active');
      });
    });
    
    // Обработка кнопок в хедере
    document.querySelector('.icons').addEventListener('click', function(e) {
      if (e.target.textContent === '⚙️') {
        showPreferences();
      } else if (e.target.textContent === '💬') {
        console.log('Чат');
      } else if (e.target.textContent === '🔍') {
        console.log('Поиск');
      }
    });
  }

  // Функция для показа настроек
  function showPreferences() {
    const modal = document.createElement('div');
    modal.style.cssText = `
      position: fixed;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
      background: var(--panel-bg);
      padding: 20px;
      border-radius: var(--radius);
      box-shadow: 0 4px 20px rgba(0,0,0,0.2);
      z-index: 1000;
    `;
    modal.innerHTML = '<h2>Настройки</h2><p>Здесь будут настройки приложения</p><button id="close-modal">Закрыть</button>';
    document.body.appendChild(modal);
    
    document.getElementById('close-modal').addEventListener('click', () => {
      modal.remove();
    });
  }

  // Настройка функционала выделения текста
  function setupTextSelection() {
    const reader = document.querySelector('.reader');
    
    reader.addEventListener('mouseup', function() {
      const selection = window.getSelection();
      if (selection.toString().length > 0) {
        const range = selection.getRangeAt(0);
        const rect = range.getBoundingClientRect();
        
        let toolbar = document.getElementById('selection-toolbar');
        if (!toolbar) {
          toolbar = document.createElement('div');
          toolbar.id = 'selection-toolbar';
          document.body.appendChild(toolbar);
        }
        
        toolbar.style.left = rect.left + (rect.width / 2) - 50 + 'px';
        toolbar.style.top = rect.top - 40 + 'px';
        
        toolbar.innerHTML = '';
        
        const actions = [
          { icon: '📝', title: 'Добавить заметку', action: () => addNote(selection.toString()) },
          { icon: '🔍', title: 'Объяснить', action: () => explainText(selection.toString()) },
          { icon: '🖌️', title: 'Выделить', action: () => highlightText(selection.getRangeAt(0)) }
        ];
        
        actions.forEach(({ icon, title, action }) => {
          const button = document.createElement('button');
          button.textContent = icon;
          button.title = title;
          button.onclick = () => {
            action();
            toolbar.remove();
          };
          toolbar.appendChild(button);
        });
        
        const hideToolbar = () => {
          toolbar.remove();
          document.removeEventListener('mousedown', hideToolbar);
        };
        
        setTimeout(() => {
          document.addEventListener('mousedown', hideToolbar);
        }, 100);
      }
    });
  }

  // Функции для работы с выделенным текстом
  function addNote(text) {
    const notes = document.querySelector('.notes');
    const noteDiv = document.createElement('div');
    noteDiv.className = 'note';
    noteDiv.textContent = text;
    notes.appendChild(noteDiv);
  }

  function explainText(text) {
    console.log('Объяснение текста:', text);
    const notes = document.querySelector('.notes');
    const noteDiv = document.createElement('div');
    noteDiv.className = 'note';
    noteDiv.innerHTML = `<strong>AI объяснение:</strong><br>${text}`;
    notes.appendChild(noteDiv);
  }

  function highlightText(range) {
    const span = document.createElement('span');
    span.style.backgroundColor = 'rgba(112, 179, 255, 0.3)';
    span.style.borderRadius = '2px';
    range.surroundContents(span);
  }

  // Функция для обработки действий из меню
  function setupMenuHandler() {
    if (window.__TAURI__) {
      window.__TAURI__.event.listen('menu-action', (event) => {
        if (event.payload === 'open') {
          window.__TAURI__.dialog.open({
            multiple: false,
            filters: [{
              name: 'Документы',
              extensions: ['txt', 'pdf', 'epub', 'md']
            }]
          }).then(selected => {
            if (selected) {
              window.__TAURI__.invoke('open_document', { filepath: selected })
                .then(content => {
                  updateContent(selected, content);
                })
                .catch(err => {
                  console.error('Error loading document:', err);
                });
            }
          });
        } else if (event.payload === 'preferences') {
          showPreferences();
        }
      });
    }
  }

  // Функция для обновления контента в ридере
  function updateContent(filename, content) {
    const reader = document.querySelector('.reader');
    const title = reader.querySelector('h1');
    title.textContent = filename.split('/').pop();
    
    // Добавляем счетчик страниц
    const pageCounter = document.createElement('small');
    pageCounter.style.cssText = 'color:#555; font-size:0.8rem; margin-left:auto;';
    pageCounter.textContent = 'Page 1 of 1';
    title.appendChild(pageCounter);
    
    // Очищаем текущие параграфы
    document.querySelectorAll('.reader .paragraph').forEach(p => p.remove());
    
    // Добавляем новые параграфы
    const paragraphs = content.split('\n\n');
    paragraphs.forEach((text, index) => {
      const p = document.createElement('div');
      p.className = 'paragraph';
      p.textContent = `${index + 1}. ${text}`;
      reader.appendChild(p);
    });
  }
})();
"#; 