declare module '@novnc/novnc/lib/rfb' {
    export class RFB {
        constructor(
            target: HTMLCanvasElement,
            url: string,
            options?: {
                credentials?: { password: string };
                wsProtocols?: string[];
            }
        );

        addEventListener(event: string, callback: () => void): void;
        disconnect(): void;
    }
}
