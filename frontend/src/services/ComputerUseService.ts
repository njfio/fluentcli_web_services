import { axiosInstance } from './apiClient';

export interface ComputerUseMessage {
    role: 'user' | 'assistant' | 'system';
    content: string;
}

export interface ToolOutput {
    error?: string;
    success: boolean;
    stdout?: string;
    stderr?: string;
    text?: string;
    coordinate?: {
        x: number;
        y: number;
    };
    image?: string;
    screenshot?: string;
}

export interface ToolResult {
    content?: string;
    tool_use_id?: string;
    type?: string;
    error?: string;
    image?: string;
    action?: string;
    name?: string;
    command?: string;
    path?: string;
    output?: ToolOutput;
    screenshot?: string;
    coordinate?: {
        x: number;
        y: number;
    };
}

class ComputerUseService {
    private controller: AbortController | null = null;

    private filterMessagesForApi(messages: ComputerUseMessage[]): ComputerUseMessage[] {
        // Find the last screenshot message
        let lastScreenshotIndex = -1;
        for (let i = messages.length - 1; i >= 0; i--) {
            const msg = messages[i];
            if (msg.role === 'assistant' && msg.content.includes('<img>')) {
                lastScreenshotIndex = i;
                break;
            }
        }

        // Filter and transform messages
        return messages.map((msg, index) => {
            // For assistant messages, handle screenshots
            if (msg.role === 'assistant') {
                let content = msg.content;
                const imgMatch = content.match(/<img>(.*?)<\/img>/);
                if (imgMatch) {
                    // Only keep the most recent screenshot
                    if (index !== lastScreenshotIndex) {
                        content = content.replace(/<img>.*?<\/img>/, '');
                    } else {
                        // Replace base64 data with filename reference
                        const toolResult = this.parseToolResult(content);
                        if (toolResult?.output?.screenshot) {
                            content = content.replace(
                                /<img>.*?<\/img>/,
                                `<img>screenshot:${toolResult.output.screenshot}</img>`
                            );
                        }
                    }
                }
                return { role: msg.role, content };
            }
            return msg;
        }).filter((msg, index, arr) => {
            if (index === 0) return true;
            const prevMsg = arr[index - 1];

            // Keep all user messages
            if (msg.role === 'user') return true;

            // Keep assistant messages that are tool results
            if (msg.role === 'assistant' && msg.content.includes('<tool>')) return true;

            // Keep assistant messages that aren't duplicates
            return !(msg.role === prevMsg.role && msg.content === prevMsg.content);
        });
    }

    async chat(messages: ComputerUseMessage[]): Promise<ReadableStream<Uint8Array>> {
        if (this.controller) {
            this.controller.abort();
            this.controller = null;
        }

        this.controller = new AbortController();

        const baseUrl = axiosInstance.defaults.baseURL?.replace(/\/$/, '');
        const filteredMessages = this.filterMessagesForApi(messages);

        const response = await fetch(`${baseUrl}/computer-use/chat`, {
            method: 'POST',
            headers: {
                'Authorization': `Bearer ${localStorage.getItem('token')}`,
                'Content-Type': 'application/json',
                'Accept': 'text/event-stream',
            },
            body: JSON.stringify({
                messages: filteredMessages
            }),
            signal: this.controller.signal
        });

        if (!response.ok) {
            const text = await response.text();
            throw new Error(`HTTP error! status: ${response.status}, body: ${text}`);
        }

        if (!response.body) {
            throw new Error('No response body');
        }

        return response.body;
    }

    parseToolResult(content: string): ToolResult | null {
        try {
            // Find all JSON objects in the content
            const jsonMatches = content.match(/\{[^{}]*\}/g);
            if (!jsonMatches) return null;

            // Parse each JSON object and find the most relevant one
            for (const jsonStr of jsonMatches.reverse()) { // Start from the last one
                try {
                    const result = JSON.parse(jsonStr);

                    // If it's a tool result with nested content, parse that too
                    if (result.content && typeof result.content === 'string') {
                        try {
                            // Look for JSON in the content
                            const nestedMatches = result.content.match(/\{[^{}]*\}/g);
                            if (nestedMatches) {
                                const nestedContent = JSON.parse(nestedMatches[0]);
                                return {
                                    ...result,
                                    ...nestedContent
                                };
                            }
                        } catch (e) {
                            // If nested parsing fails, continue with original result
                        }
                    }

                    // If it has required tool result properties, return it
                    if (result.type === 'tool_result' || result.tool_use_id || result.action) {
                        return result;
                    }
                } catch (e) {
                    // If parsing fails, try the next JSON object
                    continue;
                }
            }

            // If no valid tool result found, parse the last JSON object
            return JSON.parse(jsonMatches[jsonMatches.length - 1]);
        } catch (e) {
            return null;
        }
    }

    getToolError(result: ToolResult | null): string | undefined {
        if (!result) return undefined;

        // Check for direct error
        if (result.error) {
            return result.error;
        }

        // Check for output error
        if (result.output?.error) {
            return result.output.error;
        }

        // Check for error in content
        if (result.content && typeof result.content === 'string') {
            try {
                const contentObj = JSON.parse(result.content);
                if (contentObj.output?.error) {
                    return contentObj.output.error;
                }
            } catch (e) {
                // If parsing fails, ignore
            }
        }

        // Check for action-specific errors
        if (result.action === 'mouse_move' && !result.coordinate && !result.output?.coordinate) {
            return 'Missing coordinates for mouse movement';
        }

        return undefined;
    }

    shouldContinue(content: string): boolean {
        // Check for explicit continue flag
        if (content.includes('<continue>true</continue>')) {
            return true;
        }

        // Check for tool result success
        const toolResult = this.parseToolResult(content);
        if (toolResult?.output?.success) {
            return true;
        }

        // Check for message_stop
        if (content.includes('"type":"message_stop"')) {
            return false;
        }

        return false;
    }

    isToolResult(content: string): boolean {
        // Check for tool tags
        if (content.includes('<tool>') && content.includes('</tool>')) {
            return true;
        }

        // Check for continue flag
        if (this.shouldContinue(content)) {
            return true;
        }

        // Check for message_stop
        if (content.includes('"type":"message_stop"')) {
            return true;
        }

        // Check for tool result object
        const toolResult = this.parseToolResult(content);
        if (!toolResult) return false;
        return toolResult.type === 'tool_result' || !!toolResult.tool_use_id;
    }

    cancelRequest() {
        if (this.controller) {
            this.controller.abort();
            this.controller = null;
        }
    }
}

export default new ComputerUseService();
