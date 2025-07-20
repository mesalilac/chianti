export type MessageType =
    | 'recordHistory'
    | 'page-rendered'
    | 'setApiURL'
    | 'getApiURL';

export interface Message<T> {
    type: MessageType;
    payload?: T;
}
