import { Message, WatchHistoryBody } from './types.d';

let payload: WatchHistoryBody | null = null;
let intervalId: number | null = null;

const delay = (ms: number) => new Promise((res) => setTimeout(res, ms));

function parseRelativeDate(dateString: string): number {
    const parts = dateString.split(' ');
    if (parts.length < 3 || parts[2].toLowerCase() !== 'ago') return NaN;

    const value: number = Number(parts[0]);
    let unit: string = parts[1].toLowerCase();
    if (isNaN(value)) return NaN;

    if (unit.endsWith('s') && value === 1) unit = unit.slice(0, -1);

    const resultDate = new Date();

    switch (unit) {
        case 'second':
        case 'seconds':
            resultDate.setSeconds(resultDate.getSeconds() - value);
            break;
        case 'minute':
        case 'minutes':
            resultDate.setMinutes(resultDate.getMinutes() - value);
            break;
        case 'hour':
        case 'hours':
            resultDate.setHours(resultDate.getHours() - value);
            break;
        case 'day':
        case 'days':
            resultDate.setDate(resultDate.getDate() - value);
            break;
        case 'week':
        case 'weeks':
            resultDate.setDate(resultDate.getDate() - value * 7);
            break;
        case 'month':
        case 'months':
            resultDate.setMonth(resultDate.getMonth() - value);
            break;
        case 'year':
        case 'years':
            resultDate.setFullYear(resultDate.getFullYear() - value);
            break;
        default:
            return NaN;
    }

    return Math.floor(resultDate.getTime() / 1000);
}

function getVideoInfo(): {
    title: string;
    duration: number;
    published_at: number;
    view_count: number;
} | null {
    const videoTitleHeadingelement = document.querySelector(
        '#title>h1',
    ) as HTMLHeadingElement;
    if (!videoTitleHeadingelement.textContent) {
        console.error('[chianti] Video title not found');
        return null;
    }

    const videoDurationElement = document.querySelector('.ytp-time-duration');
    if (!videoDurationElement?.textContent) {
        console.error('[chianti] Video duration not found');
        return null;
    }

    // Expand description
    const bottomRowElement = document.querySelector('#bottom-row');
    if (!bottomRowElement) {
        console.error('[chianti] Bottom row not found');
        return null;
    }

    const descriptionElement = bottomRowElement.querySelector(
        '#description',
    ) as HTMLButtonElement;
    if (!descriptionElement) {
        console.error('[chianti] Description button not found');
        return null;
    }

    descriptionElement.click();

    const descriptionInnerElement =
        document.querySelector('#description-inner');
    if (!descriptionInnerElement) {
        console.error('[chianti] Description inner element not found');
        return null;
    }

    const descriptionInfoContainer =
        descriptionInnerElement.querySelector('#info');
    if (
        !descriptionInfoContainer ||
        !descriptionInfoContainer.children[0].textContent ||
        !descriptionInfoContainer.children[2].textContent
    ) {
        console.error('[chianti] Description info not found');
        return null;
    }

    const tempVideoViews = descriptionInfoContainer.children[0].textContent;
    let tempVideoPublishDate = descriptionInfoContainer.children[2].textContent;
    let videoPublishDate: number = 0;

    if (tempVideoPublishDate.startsWith('Premiered ')) {
        tempVideoPublishDate = tempVideoPublishDate.replace('Premiered ', '');
    }

    if (tempVideoPublishDate.includes('ago')) {
        videoPublishDate = parseRelativeDate(tempVideoPublishDate);
    } else {
        videoPublishDate = Number(
            new Date(tempVideoPublishDate).getTime() / 1000,
        );
    }

    // Collapse description
    const collapseElement = document.querySelector(
        '#collapse',
    ) as HTMLButtonElement;
    if (!collapseElement) {
        console.error('[chianti] Collapse button not found');
        return null;
    }
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
        published_at: videoPublishDate,
        view_count: Number(tempVideoViews.split(' ')[0].replaceAll(',', '')),
    };
}

function getChannelInfo(): {
    name: string;
    id: string;
    subscribersCount: number;
    avaterUrl: string;
} | null {
    const channelInfoElement = document.querySelector('#upload-info');
    if (!channelInfoElement) {
        console.error('[chianti] Channel info not found');
        return null;
    }

    const channelATag = channelInfoElement.querySelector(
        '#text>a',
    ) as HTMLLinkElement | null;
    if (!channelATag?.textContent) {
        console.error('[chianti] Channel name not found');
        return null;
    }

    const channelName = channelATag.textContent.trim();

    let channelID = channelATag.getAttribute('href');
    if (!channelID) {
        console.error('[chianti] Channel ID not found');
        return null;
    }

    if (channelID.startsWith('/channel/')) {
        console.debug('[chianti] Channel ID starts with /channel/');
        channelID = channelID.replace('/channel/', '');
    } else {
        console.debug('[chianti] Channel ID starts with /@');
        channelID = channelID.replace('/@', '');
    }

    const ownerSubCount = channelInfoElement.querySelector('#owner-sub-count');
    if (!ownerSubCount?.textContent) {
        console.error('[chianti] Channel subscribers count not found');
        return null;
    }

    const subscribersCountChars = ownerSubCount.textContent
        .split(' ')[0]
        .toLowerCase()
        .split('');
    if (subscribersCountChars.length === 0) {
        console.error('[chianti] Channel subscribers count not found');
        return null;
    }

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

    const avaterElement = document.querySelector(
        '#avatar #img',
    ) as HTMLImageElement | null;
    if (!avaterElement) {
        console.error('[chianti] Channel avater not found');
        return null;
    }

    const avaterUrl = avaterElement.src.replace('=s48', '=s280');
    if (!avaterUrl) {
        console.error('[chianti] Channel avater URL not found');
        return null;
    }

    return {
        id: channelID,
        name: channelName,
        subscribersCount: Math.round(subscribersCount),
        avaterUrl: avaterUrl,
    };
}

async function main() {
    const urlParams = new URLSearchParams(window.location.search);

    const videoID = urlParams.get('v');

    if (!videoID) {
        console.error('[chianti] Video ID not found');
        return;
    }

    console.log('[chianti] Waiting for page to load');
    await delay(4000);

    if (document.readyState !== 'complete') {
        console.error('[chianti] Page not fully loaded');
        return;
    }
    const videoInfo = getVideoInfo();
    if (!videoInfo) {
        console.error('[chianti] Failed to collect video info');
        return;
    }

    const channelInfo = getChannelInfo();
    if (!channelInfo) {
        console.error('[chianti] Failed to collect channel info');
        return;
    }

    const thumbnail_url = `https://img.youtube.com/vi/${videoID}/hqdefault.jpg`;

    payload = {
        // For channel
        channel_id: channelInfo.id,
        channel_name: channelInfo.name,
        channel_avater_url: channelInfo.avaterUrl,
        channel_subscribers_count: Math.round(channelInfo.subscribersCount),
        // For video
        video_id: videoID,
        video_title: videoInfo.title,
        video_description: '',
        video_duration: Math.round(videoInfo.duration),
        video_tags: [],
        published_at: Math.round(videoInfo.published_at),
        view_count: Math.round(videoInfo.view_count),
        watch_duration_seconds: 0,
        session_start_date: Math.round(Number(Date.now() / 1000)),
        session_end_date: Math.round(Number(Date.now() / 1000)),
        video_thumbnail_url: thumbnail_url,
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

browser.runtime.onMessage.addListener((message: Message<undefined>) => {
    if (message.type === 'page-rendered') {
        if (payload) {
            payload.session_end_date = Math.round(Number(Date.now() / 1000));
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

        main();
    }
});

if (document.readyState === 'complete') {
    main();
}

window.addEventListener('beforeunload', () => {
    if (payload) {
        payload.session_end_date = Math.round(Number(Date.now() / 1000));
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
});
