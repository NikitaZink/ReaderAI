pub const WINDOW_STYLES: &str = r#"
:root {
  --window-border-radius: 100px;
}

body {
  border-radius: var(--window-border-radius);
  overflow: hidden;
}

#titlebar {
  height: 40px;
  background: #4B6EAF;
  color: white;
  display: flex;
  justify-content: space-between;
  align-items: center;
  user-select: none;
  -webkit-user-select: none;
  padding: 0 15px;
  border-top-left-radius: var(--window-border-radius);
  border-top-right-radius: var(--window-border-radius);
  -webkit-app-region: drag;
  app-region: drag;
}

#titlebar-buttons {
  display: flex;
  gap: 8px;
  -webkit-app-region: no-drag;
  app-region: no-drag;
}

.titlebar-button {
  width: 16px;
  height: 16px;
  border-radius: 50%;
  display: flex;
  justify-content: center;
  align-items: center;
  cursor: pointer;
}

#minimize {
  background: #FFBD44;
}

#maximize {
  background: #00CA4E;
}

#close {
  background: #FF605C;
}

.drag-region {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 40px;
  -webkit-app-region: drag;
  app-region: drag;
  z-index: 1000;
}
"#;

pub const TITLEBAR_SCRIPT: &str = r#"
// Функция для создания кастомного тайтлбара
function setupTitlebar() {
    const dragRegion = document.createElement('div');
    dragRegion.className = 'drag-region';
    document.body.appendChild(dragRegion);

    const titlebar = document.createElement('div');
    titlebar.id = 'titlebar';
    titlebar.setAttribute('data-tauri-drag-region', '');
    
    const title = document.createElement('div');
    title.textContent = 'ReaderAI';
    title.setAttribute('data-tauri-drag-region', '');
    titlebar.appendChild(title);
    
    const buttons = document.createElement('div');
    buttons.id = 'titlebar-buttons';
    
    const minimize = document.createElement('div');
    minimize.id = 'minimize';
    minimize.className = 'titlebar-button';
    minimize.addEventListener('click', () => window.__TAURI__.window.appWindow.minimize());
    
    const maximize = document.createElement('div');
    maximize.id = 'maximize';
    maximize.className = 'titlebar-button';
    maximize.addEventListener('click', () => window.__TAURI__.window.appWindow.toggleMaximize());
    
    const close = document.createElement('div');
    close.id = 'close';
    close.className = 'titlebar-button';
    close.addEventListener('click', () => window.__TAURI__.window.appWindow.close());
    
    buttons.appendChild(minimize);
    buttons.appendChild(maximize);
    buttons.appendChild(close);
    titlebar.appendChild(buttons);
    
    document.body.prepend(titlebar);
    
    titlebar.addEventListener('mousedown', (e) => {
        if (e.target === titlebar || e.target === title) {
            window.__TAURI__.window.appWindow.startDragging();
        }
    });
    
    window.__TAURI__.event.listen('menu-action', (event) => {
        console.log('Menu action received:', event.payload);
        if (event.payload === 'open') {
            console.log('Opening document dialog...');
            window.__TAURI__.dialog.open({
                multiple: false,
                filters: [{
                    name: 'Документы',
                    extensions: ['txt', 'pdf', 'epub', 'md']
                }]
            }).then(selected => {
                if (selected) {
                    console.log('Selected file:', selected);
                }
            });
        } else if (event.payload === 'preferences') {
            console.log('Showing preferences...');
            const modal = document.createElement('div');
            modal.style.cssText = `
                position: fixed;
                top: 50%;
                left: 50%;
                transform: translate(-50%, -50%);
                background: white;
                padding: 20px;
                border-radius: 10px;
                box-shadow: 0 4px 20px rgba(0,0,0,0.2);
                z-index: 1000;
            `;
            modal.innerHTML = '<h2>Настройки</h2><p>Здесь будут настройки приложения</p><button id="close-modal">Закрыть</button>';
            document.body.appendChild(modal);
            
            document.getElementById('close-modal').addEventListener('click', () => {
                modal.remove();
            });
        }
    });
}

if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', setupTitlebar);
} else {
    setupTitlebar();
}
"#; 