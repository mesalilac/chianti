export type MessageType = 'recordHistory' | 'page-rendered' | 'sendPendingData';

export interface Message<T> {
    type: MessageType;
    payload?: T;
}
