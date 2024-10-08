import { GlobalLogs } from "@/context/globals/logs";
import { GlobalSessions } from "@/context/globals/sessions";
import { notifications } from "@mantine/notifications";
import {
    EventSourceMessage,
    fetchEventSource,
} from "@microsoft/fetch-event-source";

export enum SseStatus {
    not_connected,
    connected,
    closed,
}

export class SSE {
    private _abort_controller: AbortController;
    private static _instance: SSE | null = null;
    private static _connecting: boolean = false;

    private constructor(private _host: string, private _bearer: string) {
        this._abort_controller = new AbortController();
        this._abort_controller.abort = this._abort_controller.abort.bind(this._abort_controller);

        // Bind the functions to the class to avoid issues with `this`
        this.onClose = this.onClose.bind(this);
        this.onError = this.onError.bind(this);
        this.onMessage = this.onMessage.bind(this);
        this.connect = this.connect.bind(this);
        this.abort = this.abort.bind(this);
    }

    public static instance(host: string, bearer: string) {
        if (SSE._instance) {
            return SSE._instance;
        }

        SSE._instance = new SSE(host, bearer);
        return SSE._instance;
    }

    private _status = SseStatus.not_connected;

    public get status() {
        return this._status;
    }

    public get host() {
        return this._host;
    }

    public get bearer() {
        return this._bearer;
    }

    public set bearer(bearer: string) {
        this._bearer = bearer;
    }

    public async connect() {
        if (this._status === SseStatus.connected || SSE._connecting) {
            throw new Error("SSE is already connected");
        }
        SSE._connecting = true;

        if (!this._abort_controller) {
            this._abort_controller = new AbortController();
            this._abort_controller.abort = this._abort_controller.abort.bind(this._abort_controller);
        }

        try {
            await fetchEventSource(`http://${ this._host }/sse`, {
                headers:   {
                    Authorization: `Bearer ${ this._bearer }`,
                },
                signal:    this._abort_controller.signal,
                onmessage: this.onMessage,
                onclose:   this.onClose,
                onerror:   this.onError,
            });

            this._status = SseStatus.connected;
            SSE._connecting = false;
        }
        catch (error) {
            this._status = SseStatus.closed;
            SSE._connecting = false;
            throw new Error(`Failed to connect to SSE: ${ error }`);
        }
    }

    public abort() {
        if (this._status !== SseStatus.connected) {
            throw new Error("SSE is not connected, cannot abort");
        }

        this._abort_controller.abort("SSE aborted");
        this._abort_controller = new AbortController();
        this._abort_controller.abort = this._abort_controller.abort.bind(this._abort_controller);
        this._status = SseStatus.closed;
    }

    private onMessage(event: EventSourceMessage) {
        switch (event.event) {
            case "log":
                GlobalLogs.data.push(JSON.parse(event.data));
                break;
            case "session":
                console.log("Session event:", event.data);
                GlobalSessions.data.unshift(JSON.parse(event.data));
                break;
            case "notification":
                console.log("Notification event:", event.data);
                const object = JSON.parse(event.data);

                let color = "violet";

                switch (object.level) {
                    case "INFO":
                        color = "violet";
                        break;
                    case "WARN":
                        color = "orange";
                        break;
                    case "ERROR":
                        color = "red";
                        break;
                    case "DEBUG":
                        color = "blue";
                        break;
                    case "TRACE":
                        color = "gray";
                }

                notifications.show({
                    title:   object.title,
                    message: object.message,
                    color,
                });
                break;
            default:
                console.error(`Unknown event(${ event.event }):`, event);
                break;
        }
    }

    private onClose() {
        console.log("Connection closed");
        this._status = SseStatus.closed;
    }

    private onError(err: any) {
        this._status = SseStatus.closed;
        throw new Error(`SSE Error ${ err }`);
    }
}