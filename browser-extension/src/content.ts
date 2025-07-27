import type {
    CreateWatchHistoryChannel,
    CreateWatchHistoryRequest,
    CreateWatchHistoryVideo,
} from '@bindings';
import browser from 'webextension-polyfill';
import { delay, extractChannelInfo, extractVideoInfo } from './content-utils';
import type { Message } from './types.d';

let payload: CreateWatchHistoryRequest | null = null;
let intervalId: number | null = null;

async function main() {
    const urlParams = new URLSearchParams(window.location.search);

    const videoId = urlParams.get('v');

    if (!videoId) {
        console.error('[chianti] Video ID not found');
        return;
    }

    console.log('[chianti] Waiting for page to load');
    await delay(8000);

    if (document.readyState !== 'complete') {
        console.error('[chianti] Page not fully loaded');
        return;
    }
    const videoInfo: CreateWatchHistoryVideo | null = extractVideoInfo(videoId);
    if (!videoInfo) {
        console.error('[chianti] Failed to collect video info');
        return;
    }

    const channelInfo: CreateWatchHistoryChannel | null = extractChannelInfo();
    if (!channelInfo) {
        console.error('[chianti] Failed to collect channel info');
        return;
    }

    payload = {
        watch_duration_seconds: 0,
        session_start_date: Math.round(Number(Date.now() / 1000)),
        session_end_date: Math.round(Number(Date.now() / 1000)),

        channel: channelInfo,
        video: videoInfo,
    };

    console.log(payload);

    const moviePlayerElement = document.querySelector('#movie_player');
    if (!moviePlayerElement) {
        console.error('[chianti] Movie player not found');
        return;
    }

    const videoElement = moviePlayerElement.querySelector(
        'video',
    ) as HTMLVideoElement | null;
    if (!videoElement) {
        console.error('[chianti] Video element not found');
        return;
    }

    intervalId = setInterval(() => {
        if (!videoElement.paused) {
            if (payload) {
                payload.watch_duration_seconds += 1;
            }
        }
    }, 1000);
}

function pushPayload() {
    if (payload) {
        browser.runtime.sendMessage({
            type: 'recordHistory',
            payload: payload,
        });
        if (intervalId) {
            clearInterval(intervalId);
            intervalId = null;
        }
        payload = null;
    }
}

browser.runtime.onMessage.addListener((message: Message<undefined>) => {
    if (message.type === 'page-rendered') {
        pushPayload();

        main();
    }
});

if (document.readyState === 'complete') {
    main();
}

window.addEventListener('beforeunload', () => {
    pushPayload();
});
