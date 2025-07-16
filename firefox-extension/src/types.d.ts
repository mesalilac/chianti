export type MessageType =
    | 'recordHistory'
    | 'page-rendered'
    | 'setApiURL'
    | 'getApiURL';

export interface Message<T> {
    type: MessageType;
    payload?: T;
}

export interface WatchHistoryBody {
    channel_id: string;
    channel_name: string;
    video_id: string;
    video_title: string;
    video_duration: number;
    published_at: number;
    view_count: number;
    watch_duration_seconds: number;
}
