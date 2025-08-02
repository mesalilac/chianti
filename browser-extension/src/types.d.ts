export type MessageType =
    | 'recordHistory'
    | 'page-rendered'
    | 'sendPendingData'
    | 'send-notification';

export interface Message<T> {
    type: MessageType;
    payload?: T;
}

export type Result<T, E> =
    | { data: T; error?: never }
    | { data?: never; error: E };
