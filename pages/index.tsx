import { useState } from 'react';
import Head from 'next/head';
import styles from '../styles/Home.module.css';

export default function Home() {
  const [file, setFile] = useState<string | null>(null);

  return (
    <div className={styles.container}>
      <Head>
        <title>ReaderAI</title>
        <meta name="description" content="Интеллектуальное чтение и управление заметками" />
        <link rel="icon" href="/logo.svg" />
      </Head>

      <header className={styles.header}>
        <h1>ReaderAI</h1>
        <p>Интеллектуальное чтение и управление заметками</p>
      </header>

      <main className={styles.main}>
        {!file ? (
          <div className={styles.welcome}>
            <h2>Добро пожаловать в ReaderAI</h2>
            <p>
              ReaderAI поможет вам лучше понимать и структурировать информацию при чтении.
              Используйте AI для создания заметок, пояснения сложных абзацев и
              организации полученных знаний.
            </p>
            <div className={styles.actions}>
              <button className={styles.primaryButton}>Открыть документ</button>
              <button className={styles.secondaryButton}>Создать новый документ</button>
            </div>
          </div>
        ) : (
          <div className={styles.reader}>
            {/* Здесь будет интерфейс чтения */}
            <p>Контент документа будет отображаться здесь</p>
          </div>
        )}
      </main>

      <footer className={styles.footer}>
        <p>ReaderAI v0.1.0</p>
      </footer>
    </div>
  );
} 