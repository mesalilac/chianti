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
    channel_avater_url: string;
    channel_subscribers_count: number;
    video_id: string;
    video_title: string;
    video_description: string;
    video_duration: number;
    video_tags: string[];
    published_at: number;
    view_count: number;
    watch_duration_seconds: number;
    session_start_date: number;
    session_end_date: number;
    video_thumbnail_url: string;
}
