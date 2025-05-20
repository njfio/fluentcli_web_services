import MarkdownIt from 'markdown-it';
import hljs from 'highlight.js';

// Initialize markdown-it with options
const md: MarkdownIt = new MarkdownIt({
  html: true,
  linkify: true,
  typographer: true,
  highlight: function (str: string, lang: string): string {
    if (lang && hljs.getLanguage(lang)) {
      try {
        return '<pre class="hljs"><code>' +
               hljs.highlight(str, { language: lang, ignoreIllegals: true }).value +
               '</code></pre>';
      } catch (__) {
        // Fall through if highlighting fails
      }
    }

    return '<pre class="hljs"><code>' + md.utils.escapeHtml(str) + '</code></pre>';
  }
});

/**
 * Renders markdown content to HTML
 * @param content Markdown content to render
 * @returns Rendered HTML
 */
export async function renderMarkdown(content: string): Promise<string> {
  if (!content) return '';

  try {
    return md.render(content);
  } catch (error) {
    console.error('Error rendering markdown:', error);
    return content;
  }
}
