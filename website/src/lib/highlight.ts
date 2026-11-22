// Lightweight display highlighting for Cinch source on the landing page.
// It splits text into spans for colour only; the text itself is unchanged.
const keywords = ['module','import','if','then','else','proof','spec','stage','generate','emit','where','case','of','forall','using','type','refine','with','implicit','instance','law','specialize','rewrite','match','replace','justify','let','in'];
const keywordPattern = keywords.join('|');
const splitter = new RegExp(`(--[^\\n]*|"[^"\\n]*"|\\b(?:${keywordPattern})\\b|\\b[A-Z][A-Za-z0-9_.]*\\b|\\b\\d+\\b)`, 'g');
const keywordTest = new RegExp(`^(?:${keywordPattern})$`);

export interface Token { text: string; kind?: 'comment' | 'string' | 'number' | 'type' | 'keyword'; }

export function tokenize(code: string): Token[] {
  return code.split(splitter).filter((part) => part !== '').map((text) => ({
    text,
    kind: text.startsWith('--') ? 'comment'
      : text.startsWith('"') ? 'string'
      : /^\d+$/.test(text) ? 'number'
      : /^[A-Z]/.test(text) ? 'type'
      : keywordTest.test(text) ? 'keyword'
      : undefined,
  }));
}
