import mongoose, { Schema, model, Document as IDocument } from 'mongoose';

// Интерфейс для заметки
interface INote {
  content: string;
  timestamp: Date;
  aiGenerated: boolean;
}

// Интерфейс для документа
export interface IReaderDocument extends IDocument {
  title: string;
  content: string;
  filepath?: string;
  format: string;
  notes: INote[];
  tags: string[];
  createdAt: Date;
  updatedAt: Date;
}

// Схема для заметки
const NoteSchema = new Schema<INote>({
  content: { type: String, required: true },
  timestamp: { type: Date, default: Date.now },
  aiGenerated: { type: Boolean, default: false }
});

// Схема для документа
const DocumentSchema = new Schema<IReaderDocument>(
  {
    title: { type: String, required: true },
    content: { type: String, required: true },
    filepath: { type: String },
    format: { type: String, required: true, enum: ['txt', 'pdf', 'epub', 'markdown'] },
    notes: [NoteSchema],
    tags: [{ type: String }]
  },
  { timestamps: true }
);

// Проверяем, существует ли модель, прежде чем создавать новую
export default mongoose.models.Document || model<IReaderDocument>('Document', DocumentSchema); 