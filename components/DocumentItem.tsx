import React from 'react';
import { IReaderDocument } from '../models/Document';

interface DocumentItemProps {
  document: IReaderDocument;
  onClick: (doc: IReaderDocument) => void;
}

const DocumentItem: React.FC<DocumentItemProps> = ({ document, onClick }) => {
  // Форматирование даты
  const formatDate = (date: Date): string => {
    return new Date(date).toLocaleDateString('ru-RU', {
      year: 'numeric',
      month: 'long',
      day: 'numeric'
    });
  };

  // Получение иконки в зависимости от формата файла
  const getFormatIcon = (format: string): string => {
    switch (format) {
      case 'pdf':
        return '📕';
      case 'epub':
        return '📘';
      case 'markdown':
        return '📝';
      default:
        return '📄';
    }
  };

  return (
    <div 
      className="document-item"
      onClick={() => onClick(document)}
    >
      <div className="document-icon">
        {getFormatIcon(document.format)}
      </div>
      <div className="document-info">
        <h3 className="document-title">{document.title}</h3>
        <div className="document-meta">
          <span>Обновлено: {formatDate(document.updatedAt)}</span>
          <span>{document.notes.length} заметок</span>
        </div>
        <div className="document-tags">
          {document.tags.map((tag, index) => (
            <span key={index} className="tag">
              {tag}
            </span>
          ))}
        </div>
      </div>
    </div>
  );
};

export default DocumentItem; 