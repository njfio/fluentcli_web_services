declare module '@novnc/novnc/lib/rfb' {
    interface RFBOptions {
        credentials?: {
            password?: string;
        };
        wsProtocols?: string[];
    }

    interface RFBEvents {
        connect: () => void;
        disconnect: (e?: any) => void;
        credentialsrequired: () => void;
        securityfailure: (e?: any) => void;
        error: (e?: any) => void;
    }

    class RFB {
        constructor(target: HTMLElement, url: string, options?: RFBOptions);
        disconnect(): void;
        addEventListener<K extends keyof RFBEvents>(event: K, callback: RFBEvents[K]): void;
        removeEventListener<K extends keyof RFBEvents>(event: K, callback: RFBEvents[K]): void;
    }

    export default RFB;
}
