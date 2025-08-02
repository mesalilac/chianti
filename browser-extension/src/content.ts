import type {
    CreateWatchHistoryRequest,
    CreateWatchHistoryVideo,
} from '@bindings';
import browser from 'webextension-polyfill';
import { delay, extractChannelInfo, extractVideoInfo } from './content-utils';
import type { Message, MessageType } from './types.d';

let payload: CreateWatchHistoryRequest | null = null;
let intervalId: number | null = null;

function isLiveStream() {
    const viewCount = document.querySelector(
        '#view-count > yt-formatted-string:nth-child(3) > span:nth-child(1)',
    )?.textContent;

    if (!viewCount) return false;

    return viewCount.toLowerCase().includes('watching now') || false;
}

async function main() {
    const urlParams = new URLSearchParams(window.location.search);

    const videoId = urlParams.get('v');

    if (!videoId) {
        console.error('[chianti] Video ID not found');
        return;
    }

    {
        let retry = 1;
        while (true) {
            const videoTitleHeadingelement = document.querySelector(
                '#title>h1',
            ) as HTMLHeadingElement;
            if (videoTitleHeadingelement || retry === 10) {
                console.log(`[chianti] Page loaded ${videoId}`);
                break;
            }

            retry++;
            await delay(1000);
        }
    }

    if (isLiveStream()) {
        console.debug('[chianti] Live stream detected, skipping');
        return;
    }

    {
        let retry = 1;
        while (true) {
            const commentsHeaderCountEle = document.querySelector(
                '#comments>#sections>#header #count span',
            );
            if (commentsHeaderCountEle || retry === 10) {
                break;
            } else {
                window.scrollTo({ top: retry * 1000, behavior: 'smooth' });
            }

            retry++;
            await delay(1000);
        }
        window.scrollTo(0, 0);
    }

    const videoInfo = extractVideoInfo(videoId);
    if (videoInfo.error) {
        const msg = `[chianti] Error(video info): '${videoInfo.error}'`;
        console.error(msg);

        browser.runtime.sendMessage({
            type: 'send-notification' as MessageType,
            payload: msg,
        });

        return;
    }

    const channelInfo = extractChannelInfo();
    if (channelInfo.error) {
        const msg = `[chianti] Error(channel info): '${channelInfo.error}'`;
        console.error(msg);

        browser.runtime.sendMessage({
            type: 'send-notification' as MessageType,
            payload: msg,
        });

        return;
    }

    payload = {
        watch_duration_seconds: 0,
        session_start_date: Math.round(Number(Date.now() / 1000)),
        session_end_date: Math.round(Number(Date.now() / 1000)),

        channel: channelInfo.data!,
        video: videoInfo.data!,
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

browser.runtime.onMessage.addListener(async (message: Message<undefined>) => {
    if (message.type === 'page-rendered') {
        pushPayload();

        await delay(4000);
        main();
    }
});

if (document.readyState === 'complete') {
    main();
}

window.addEventListener('beforeunload', () => {
    pushPayload();
});
