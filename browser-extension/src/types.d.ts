export type MessageType =
    | 'recordHistory'
    | 'page-rendered'
    | 'sendPendingData'
    | 'send-notification';

export interface Message<T> {
    type: MessageType;
    payload?: T;
}
