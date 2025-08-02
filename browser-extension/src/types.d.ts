export type MessageType =
    | 'recordHistory'
    | 'page-rendered'
    | 'sendPendingData'
    | 'send-notification';

export interface Message<T> {
    type: MessageType;
    payload?: T;
}

type SuccessResult<T> = {
    data: T;
    error?: never;
};

type ErrorResult = {
    data?: never;
    error: string;
};

export type Result<T> = SuccessResult<T> | ErrorResult;
