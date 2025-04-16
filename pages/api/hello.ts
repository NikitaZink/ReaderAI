import type { NextApiRequest, NextApiResponse } from 'next';
import clientPromise from '../../lib/mongodb';

type Data = {
  name: string;
  message: string;
  mongoConnected: boolean;
};

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse<Data>
) {
  try {
    // Check MongoDB connection
    const client = await clientPromise;
    const isConnected = !!client;

    res.status(200).json({
      name: 'ReaderAI API',
      message: 'Welcome to the ReaderAI API!',
      mongoConnected: isConnected
    });
  } catch (error) {
    console.error('MongoDB connection error:', error);
    res.status(500).json({
      name: 'ReaderAI API',
      message: 'Error connecting to MongoDB',
      mongoConnected: false
    });
  }
} 