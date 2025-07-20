export type MessageType = 'recordHistory' | 'page-rendered';

export interface Message<T> {
    type: MessageType;
    payload?: T;
}
