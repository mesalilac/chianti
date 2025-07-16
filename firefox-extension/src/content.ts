import { Message, WatchHistoryBody } from './types.d';

let payload: WatchHistoryBody | null = null;
let intervalId: number | null = null;

function getVideoInfo(): {
    title: string;
    duration: number;
    published_at: number;
    view_count: number;
} | null {
    const videoTitleHeadingelement = document.querySelector(
        '#title>h1',
    ) as HTMLHeadingElement;
    if (!videoTitleHeadingelement.textContent) return null;

    const videoDurationElement = document.querySelector('.ytp-time-duration');
    if (!videoDurationElement?.textContent) return null;

    // Expand description
    const bottomRowElement = document.querySelector('#bottom-row');
    if (!bottomRowElement) return null;

    const descriptionElement = bottomRowElement.querySelector(
        '#description',
    ) as HTMLButtonElement;
    if (!descriptionElement) return null;

    descriptionElement.click();

    const descriptionInnerElement =
        document.querySelector('#description-inner');
    if (!descriptionInnerElement) return null;

    const descriptionInfoContainer =
        descriptionInnerElement.querySelector('#info');
    if (
        !descriptionInfoContainer ||
        !descriptionInfoContainer.children[0].textContent ||
        !descriptionInfoContainer.children[2].textContent
    )
        return null;

    const tempVideoViews = descriptionInfoContainer.children[0].textContent;
    const tempVideoPublishDate =
        descriptionInfoContainer.children[2].textContent;

    // Collapse description
    const collapseElement = document.querySelector(
        '#collapse',
    ) as HTMLButtonElement;
    if (!collapseElement) return null;
    collapseElement.click();

    return {
        title: videoTitleHeadingelement.textContent.trim(),
        duration: Number(
            videoDurationElement.textContent
                .split(':')
                .reverse()
                .reduce(
                    (prev, curr, i) => prev + Number(curr) * Math.pow(60, i),
                    0,
                ),
        ),
        published_at: Number(new Date(tempVideoPublishDate).getTime() / 1000),
        view_count: Number(tempVideoViews.split(' ')[0].replaceAll(',', '')),
    };
}

function getChannelInfo(): {
    name: string;
    id: string;
    subscribersCount: number;
} | null {
    const channelInfoElement = document.querySelector('#upload-info');
    if (!channelInfoElement) return null;

    const channelATag = channelInfoElement.querySelector(
        '#text>a',
    ) as HTMLLinkElement | null;
    if (!channelATag?.textContent) return null;

    const channelName = channelATag.textContent.trim();

    let channelID = channelATag.getAttribute('href');
    if (!channelID) return null;

    if (channelID.startsWith('/channel/')) {
        channelID = channelID.replace('/channel/', '');
    } else {
        channelID = channelID.replace('/@', '');
    }

    const ownerSubCount = channelInfoElement.querySelector('#owner-sub-count');
    if (!ownerSubCount?.textContent) return null;

    const subscribersCountChars = ownerSubCount.textContent
        .split(' ')[0]
        .toLowerCase()
        .split('');
    if (subscribersCountChars.length === 0) return null;

    let subscribersCount = 0;

    switch (subscribersCountChars[subscribersCountChars.length - 1]) {
        case 'k':
            subscribersCountChars.pop();
            subscribersCount = Number(subscribersCountChars.join('')) * 1e3;
            break;
        case 'm':
            subscribersCountChars.pop();
            subscribersCount = Number(subscribersCountChars.join('')) * 1e6;
            break;
        case 'b':
            subscribersCountChars.pop();
            subscribersCount = Number(subscribersCountChars.join('')) * 1e9;
            break;
        default:
            subscribersCount = Number(subscribersCountChars.join(''));
            break;
    }

    return {
        id: channelID,
        name: channelName,
        subscribersCount: Math.round(subscribersCount),
    };
}

async function main() {
    const urlParams = new URLSearchParams(window.location.search);

    const videoID = urlParams.get('v');

    if (!videoID) {
        return;
    }

    const delay = (ms: number) => new Promise((res) => setTimeout(res, ms));

    await delay(4000);

    const videoInfo = getVideoInfo();
    if (!videoInfo) return;

    const channelInfo = getChannelInfo();
    if (!channelInfo) return;

    payload = {
        // For channel
        channel_id: channelInfo.id,
        channel_name: channelInfo.name,
        channel_subscribers_count: Math.round(channelInfo.subscribersCount),
        // For video
        video_id: videoID,
        video_title: videoInfo.title,
        video_duration: Math.round(videoInfo.duration),
        published_at: Math.round(videoInfo.published_at),
        view_count: Math.round(videoInfo.view_count),
        watch_duration_seconds: 0,
        session_start_date: Math.round(Number(Date.now() / 1000)),
        session_end_date: Math.round(Number(Date.now() / 1000)),
    };

    console.log(payload);

    const moviePlayerElement = document.querySelector('#movie_player');
    if (!moviePlayerElement) return;

    const videoElement = moviePlayerElement.querySelector(
        'video',
    ) as HTMLVideoElement | null;
    if (!videoElement) return;

    intervalId = setInterval(() => {
        if (!videoElement.paused) {
            if (payload) {
                payload.watch_duration_seconds += 1;
            }
        }
    }, 1000);
}

browser.runtime.onMessage.addListener((message: Message<undefined>) => {
    if (message.type === 'page-rendered') {
        if (payload) {
            payload.session_end_date = Math.round(Number(Date.now() / 1000));
            browser.runtime
                .sendMessage({
                    type: 'recordHistory',
                    payload: payload,
                })
                .finally(() => {
                    if (intervalId) {
                        clearInterval(intervalId);
                        intervalId = null;
                    }
                    payload = null;
                });
        }

        main();
    }
});

window.addEventListener('beforeunload', () => {
    if (payload) {
        payload.session_end_date = Math.round(Number(Date.now() / 1000));
        browser.runtime
            .sendMessage({
                type: 'recordHistory',
                payload: payload,
            })
            .finally(() => {
                if (intervalId) {
                    clearInterval(intervalId);
                    intervalId = null;
                }
                payload = null;
            });
    }
});
