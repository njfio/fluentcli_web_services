// Use runtime configuration if available, otherwise fall back to environment variables
export const API_URL = (window as any).API_URL || import.meta.env.VITE_API_URL || 'http://localhost';

// Hardcoded fallback to prevent 8001 port usage
if (API_URL.includes('8001')) {
  console.warn('Detected port 8001 in API_URL, replacing with default port');
  // Replace any occurrence of 8001 with empty string (will use default port)
  (window as any).API_URL = API_URL.replace(':8001', '');
}

export const PROVIDER_TYPES = {
    OPENAI: 'gpt',
    ANTHROPIC: 'claude',
    COHERE: 'command',
    PERPLEXITY: 'perplexity',
    GEMINI: 'gemini',
    GROK: 'grok',
    DALLE: 'dalle',
    LEONARDO: 'leonardo',
    STABILITY: 'stability',
} as const;

export const PROVIDER_NAMES = {
    [PROVIDER_TYPES.OPENAI]: 'OpenAI',
    [PROVIDER_TYPES.ANTHROPIC]: 'Anthropic',
    [PROVIDER_TYPES.COHERE]: 'Cohere',
    [PROVIDER_TYPES.PERPLEXITY]: 'Perplexity',
    [PROVIDER_TYPES.GEMINI]: 'Gemini',
    [PROVIDER_TYPES.GROK]: 'Grok',
    [PROVIDER_TYPES.DALLE]: 'DALL-E',
    [PROVIDER_TYPES.LEONARDO]: 'Leonardo AI',
    [PROVIDER_TYPES.STABILITY]: 'Stability AI',
} as const;

export const PROVIDER_CONFIGS = {
    [PROVIDER_TYPES.OPENAI]: {
        defaultModel: 'gpt-4o-mini',
        models: ['gpt-4o', 'gpt-4o-mini'],
        requiresOrganizationId: true,
    },
    [PROVIDER_TYPES.ANTHROPIC]: {
        defaultModel: 'claude-3-haiku-20240307',
        models: ['claude-3-opus-latest', 'claude-3-5-sonnet-latest', 'claude-3-haiku-20240307'],
        requiresOrganizationId: false,
    },
    [PROVIDER_TYPES.COHERE]: {
        defaultModel: 'command',
        models: ['command', 'command-light', 'command-nightly'],
        requiresOrganizationId: false,
    },
    [PROVIDER_TYPES.PERPLEXITY]: {
        defaultModel: 'llama-3.1-sonar-huge-128k-online',
        models: ['llama-3.1-sonar-huge-128k-online', 'llama-3.1-sonar-large-128k-online', 'llama-3.1-sonar-small-128k-online'],
        requiresOrganizationId: false,
    },
    [PROVIDER_TYPES.GEMINI]: {
        defaultModel: 'gemini-1.5-pro',
        models: ['gemini-1.5-pro', 'gemini-1.5-flash'],
        requiresOrganizationId: false,
    },
    [PROVIDER_TYPES.GROK]: {
        defaultModel: 'grok-beta',
        models: ['grok-beta'],
        requiresOrganizationId: false,
    },
    [PROVIDER_TYPES.DALLE]: {
        defaultModel: 'dall-e-3',
        models: ['dall-e-3', 'dall-e-2'],
        requiresOrganizationId: true,
        isImageGenerator: true,
        configOptions: {
            size: ['1024x1024', '1024x1792', '1792x1024'],
            quality: ['standard', 'hd'],
        },
    },
    [PROVIDER_TYPES.LEONARDO]: {
        defaultModel: '6b645e3a-d64f-4341-a6d8-7a3690fbf042',  // Phoenix model
        models: ['6b645e3a-d64f-4341-a6d8-7a3690fbf042'],  // Phoenix model
        requiresOrganizationId: false,
        isImageGenerator: true,
        configOptions: {
            width: [832, 1024, 1472],
            height: [832, 1024, 1472],
            num_images: [1, 2, 3, 4],
            contrast: [3.0, 3.5, 4.0],
        },
    },
    [PROVIDER_TYPES.STABILITY]: {
        defaultModel: 'stable-diffusion-ultra',
        models: ['stable-diffusion-ultra'],
        requiresOrganizationId: false,
        isImageGenerator: true,
        configOptions: {
            width: [1024, 1536, 2048],
            height: [1024, 1536, 2048],
            cfg_scale: [1, 3, 5, 7, 10],
            steps: [30, 40, 50],
            style: ['enhance', 'anime', 'photographic', 'digital-art', 'comic-book', 'fantasy-art', 'line-art', 'analog-film', 'neon-punk', 'isometric', '3d-model', 'pixel-art', 'tile-texture', 'cinematic'],
        },
    },
} as const;

export type ProviderType = typeof PROVIDER_TYPES[keyof typeof PROVIDER_TYPES];
export type ProviderName = typeof PROVIDER_NAMES[ProviderType];
export type ProviderConfig = typeof PROVIDER_CONFIGS[ProviderType];
