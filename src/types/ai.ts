export type AIProvider = 'anthropic' | 'openai' | 'qwen' | 'minimax' | 'deepseek';

export type MessageRole = 'user' | 'assistant' | 'system';

export interface AIMessage {
  id: string;
  role: MessageRole;
  content: string;
  timestamp: number;
}

export interface AIConfig {
  provider: AIProvider;
  apiKey: string;
  model: string;
  baseUrl?: string;
}