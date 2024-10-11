declare module 'dompurify' {
    const DOMPurify: {
        sanitize: (dirty: string) => string;
    };
    export default DOMPurify;
}

declare module 'marked' {
    const marked: (markdown: string) => string;
    export { marked };
}
