import { useState, useEffect, useRef } from 'react';
import Head from 'next/head';
import styles from '../styles/Reader.module.css';

export default function Home() {
  const [file, setFile] = useState<string | null>(null);
  const [content, setContent] = useState<string>("");
  const [showWelcome, setShowWelcome] = useState(true);
  const [notes, setNotes] = useState<string[]>(['Note 1: key ideas...', 'Note 2: AI explanation...']);
  const [selectedDoc, setSelectedDoc] = useState(0);
  const [toolbarPosition, setToolbarPosition] = useState<{left: number, top: number} | null>(null);
  const [selectedText, setSelectedText] = useState<string>('');
  const [isTauriAvailable, setIsTauriAvailable] = useState(false);
  const [isMaximized, setIsMaximized] = useState(false);
  
  const readerRef = useRef<HTMLDivElement>(null);
  const toolbarRef = useRef<HTMLDivElement>(null);

  // Проверка доступности Tauri API
  useEffect(() => {
    const checkTauriAvailability = async () => {
      if (typeof window !== 'undefined') {
        try {
          // Динамический импорт Tauri API
          const tauriApp = await import('@tauri-apps/api/app');
          setIsTauriAvailable(true);
          console.log('Tauri is available');
          
          // Проверяем состояние окна
          const { appWindow } = await import('@tauri-apps/api/window');
          const isMaximized = await appWindow.isMaximized();
          setIsMaximized(isMaximized);
          
          // Слушаем события изменения размера окна
          await appWindow.onResized(async () => {
            const maximized = await appWindow.isMaximized();
            setIsMaximized(maximized);
          });
        } catch (e) {
          console.log('Tauri is not available', e);
          setIsTauriAvailable(false);
        }
      }
    };
    
    checkTauriAvailability();
  }, []);

  // Инициализация прослушивания событий Tauri
  useEffect(() => {
    if (!isTauriAvailable || typeof window === 'undefined') return;
    
    const setupTauriEvents = async () => {
      try {
        // Динамический импорт API событий
        const { listen } = await import('@tauri-apps/api/event');
        
        // Слушаем события меню
        const unlisten = await listen('menu-action', async (event) => {
          console.log('Received menu event:', event);
          if (event.payload === 'open') {
            await handleOpenDocument();
          } else if (event.payload === 'preferences') {
            // Реализация показа настроек
            console.log('Открываем настройки');
          }
        });
  
        // Отписка при размонтировании компонента
        return () => {
          unlisten();
        };
      } catch (error) {
        console.error('Failed to setup Tauri events:', error);
      }
    };
    
    setupTauriEvents();
  }, [isTauriAvailable]);

  // Обработка выделения текста
  useEffect(() => {
    if (typeof window === 'undefined') return;
    
    const checkSelection = () => {
      if (!readerRef.current) return;
      
      const selection = window.getSelection();
      if (selection && selection.toString().length > 0) {
        const range = selection.getRangeAt(0);
        const rect = range.getBoundingClientRect();
        
        setSelectedText(selection.toString());
        setToolbarPosition({
          left: rect.left + (rect.width / 2) - 50,
          top: rect.top - 40
        });
      } else {
        setToolbarPosition(null);
      }
    };

    // Закрытие тулбара при клике в другом месте
    const handleClickOutside = (e: MouseEvent) => {
      if (toolbarRef.current && !toolbarRef.current.contains(e.target as Node)) {
        setToolbarPosition(null);
      }
    };

    if (!showWelcome && readerRef.current) {
      readerRef.current.addEventListener('mouseup', checkSelection);
      document.addEventListener('mousedown', handleClickOutside);
    }

    return () => {
      if (readerRef.current) {
        readerRef.current.removeEventListener('mouseup', checkSelection);
      }
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, [showWelcome]);

  // Функция для открытия документа через Tauri API
  const handleOpenDocument = async () => {
    try {
      if (!isTauriAvailable || typeof window === 'undefined') {
        // Фолбэк для веб-версии
        setShowWelcome(false);
        setFile('Example Document');
        setContent('Это пример содержимого документа для веб-версии приложения.\n\nВ настольной версии здесь будет отображаться реальное содержимое открытого файла.');
        return;
      }
      
      // Динамический импорт необходимых API
      const { open } = await import('@tauri-apps/api/dialog');
      const { invoke } = await import('@tauri-apps/api/tauri');
      
      // Открываем диалог выбора файла
      const selected = await open({
        multiple: false,
        filters: [{
          name: 'Документы',
          extensions: ['txt', 'pdf', 'epub', 'md']
        }]
      });

      if (selected && !Array.isArray(selected)) {
        // Загружаем содержимое файла через Tauri API
        const fileContent = await invoke('open_document', { filepath: selected });
        
        // Обновляем состояние
        setShowWelcome(false);
        setFile(selected.split('/').pop() || selected.split('\\').pop() || 'Document');
        setContent(fileContent as string);
        console.log('Файл открыт:', selected);
      }
    } catch (error) {
      console.error('Ошибка при открытии документа:', error);
      // Фолбэк в случае ошибки
      setShowWelcome(false);
      setFile('Example Document');
      setContent('Произошла ошибка при открытии документа.\n\nВозможно, запущена веб-версия приложения или функциональность Tauri недоступна.');
    }
  };

  // Функция для открытия документа
  const openDocument = () => {
    handleOpenDocument();
  };

  // Функция для создания нового документа
  const createNewDocument = () => {
    setShowWelcome(false);
    setFile('New Document');
    setContent('Здесь будет ваш новый текст...');
  };
  
  // Функция для добавления заметки
  const addNote = (text: string) => {
    setNotes(prev => [...prev, text]);
    setToolbarPosition(null);
  };
  
  // Функция для объяснения текста (в реальном приложении здесь был бы запрос к AI)
  const explainText = (text: string) => {
    setNotes(prev => [...prev, `AI объяснение: ${text}`]);
    setToolbarPosition(null);
  };
  
  // Функция для выделения текста
  const highlightText = () => {
    // В реальном приложении здесь была бы логика выделения
    setToolbarPosition(null);
  };
  
  // Безопасный вызов функций Tauri
  const safeEmitEvent = async (event: string) => {
    if (!isTauriAvailable || typeof window === 'undefined') return;
    
    try {
      const { appWindow } = await import('@tauri-apps/api/window');
      appWindow.emit('menu-action', event);
    } catch (e) {
      console.error('Error emitting event:', e);
    }
  };
  
  // Функции управления окном Tauri
  const minimizeWindow = async () => {
    if (!isTauriAvailable || typeof window === 'undefined') return;
    
    try {
      const { appWindow } = await import('@tauri-apps/api/window');
      await appWindow.minimize();
    } catch (e) {
      console.error('Error minimizing window:', e);
    }
  };
  
  const maximizeWindow = async () => {
    if (!isTauriAvailable || typeof window === 'undefined') return;
    
    try {
      const { appWindow } = await import('@tauri-apps/api/window');
      
      if (isMaximized) {
        await appWindow.unmaximize();
      } else {
        await appWindow.maximize();
      }
    } catch (e) {
      console.error('Error maximizing window:', e);
    }
  };
  
  const closeWindow = async () => {
    if (!isTauriAvailable || typeof window === 'undefined') return;
    
    try {
      const { appWindow } = await import('@tauri-apps/api/window');
      await appWindow.close();
    } catch (e) {
      console.error('Error closing window:', e);
    }
  };

  return (
    <div className={styles.container}>
      <Head>
        <title>ReaderAI</title>
        <meta name="description" content="Интеллектуальное чтение и управление заметками" />
        <link rel="icon" href="/logo.svg" />
      </Head>
      
      {/* Кастомный заголовок окна для десктопного приложения */}
      {isTauriAvailable && (
        <div className={styles.titleBar} data-tauri-drag-region>
          <div className={styles.titleBarTitle} data-tauri-drag-region>ReaderAI</div>
          <div className={styles.titleBarButtons}>
            <button className={styles.titleBarButton} onClick={minimizeWindow}>─</button>
            <button className={styles.titleBarButton} onClick={maximizeWindow}>
              {isMaximized ? '❐' : '▢'}
            </button>
            <button className={`${styles.titleBarButton} ${styles.closeButton}`} onClick={closeWindow}>✕</button>
          </div>
        </div>
      )}

      {showWelcome ? (
        // Приветственный экран
        <>
          <header className={`${styles.welcomeHeader} ${isTauriAvailable ? styles.withTitleBar : ''}`}>
            <h1>ReaderAI</h1>
            <p>Интеллектуальное чтение и управление заметками</p>
          </header>

          <main className={styles.welcomeMain}>
            <div className={styles.welcome}>
              <h2>Добро пожаловать в ReaderAI</h2>
              <p>
                ReaderAI поможет вам лучше понимать и структурировать информацию при чтении.
                Используйте AI для создания заметок, пояснения сложных абзацев и
                организации полученных знаний.
              </p>
              <div className={styles.actions}>
                <button className={styles.primaryButton} onClick={openDocument}>
                  Открыть документ
                </button>
                <button className={styles.secondaryButton} onClick={createNewDocument}>
                  Создать новый документ
                </button>
              </div>
            </div>
          </main>

          <footer className={styles.footer}>
            <p>ReaderAI v0.1.0 {isTauriAvailable ? '(Desktop)' : '(Web)'}</p>
          </footer>
        </>
      ) : (
        // Интерфейс чтения с тремя колонками
        <>
          <header className={`${styles.header} ${isTauriAvailable ? styles.withTitleBar : ''}`}>
            <div className={styles.search}>
              <input type="text" placeholder="Search..." />
            </div>
            <div className={styles.icons}>
              <button>🔍</button>
              <button>💬</button>
              <button onClick={() => safeEmitEvent('preferences')}>⚙️</button>
            </div>
          </header>

          <div className={styles.content}>
            {/* Боковая панель с библиотекой */}
            <aside className={styles.sidebar}>
              <h2>Library</h2>
              {['Document 1', 'Document 2', 'Document 3'].map((doc, index) => (
                <div 
                  key={index}
                  className={`${styles.doc} ${selectedDoc === index ? styles.active : ''}`}
                  onClick={() => setSelectedDoc(index)}
                >
                  {doc}
                </div>
              ))}
            </aside>

            {/* Основная область чтения */}
            <main className={styles.reader} ref={readerRef}>
              <div className={styles.docHeader}>
                <h1>{file}</h1>
                <span className={styles.pageCounter}>Page 1 of 5</span>
              </div>
              {content ? (
                content.split('\n\n').map((paragraph, index) => (
                  <div key={index} className={styles.paragraph}>
                    {index + 1}. {paragraph}
                  </div>
                ))
              ) : (
                <>
                  <div className={styles.paragraph}>1. Lorem ipsum dolor sit amet, consectetur adipiscing elit.</div>
                  <div className={styles.paragraph}>2. Quisque elementum urna vel tortor cursus, a volutpat mi egestas.</div>
                  <div className={styles.paragraph}>3. Curabitur vitae enim eu purus cursus convallis.</div>
                </>
              )}
              
              {/* Тулбар для выделенного текста */}
              {toolbarPosition && (
                <div 
                  className={styles.selectionToolbar} 
                  style={{ left: toolbarPosition.left, top: toolbarPosition.top }}
                  ref={toolbarRef}
                >
                  <button onClick={() => addNote(selectedText)} title="Добавить заметку">📝</button>
                  <button onClick={() => explainText(selectedText)} title="Объяснить">🔍</button>
                  <button onClick={highlightText} title="Выделить">🖌️</button>
                </div>
              )}
            </main>

            {/* Панель с заметками */}
            <aside className={styles.notes}>
              <h2>Notes</h2>
              {notes.map((note, index) => (
                <div key={index} className={styles.note}>
                  {note}
                </div>
              ))}
            </aside>
          </div>

          <footer className={styles.readerFooter}>
            {/*<div className={styles.progress}>*/}
            {/*  <div className={styles.progressBar}></div>*/}
            {/*</div>*/}
            <div className={styles.fontSizeToggle}>🅰️</div>
          </footer>
        </>
      )}
    </div>
  );
} 