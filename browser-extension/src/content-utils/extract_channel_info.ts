import type { CreateWatchHistoryChannel } from '@bindings';
import type { Result } from '../types';

export function extractChannelInfo(): Result<
    CreateWatchHistoryChannel,
    string
> {
    const channelInfoElement = document.querySelector('#upload-info');
    if (!channelInfoElement) {
        return { error: 'Channel info not found' };
    }

    const channelATag = channelInfoElement.querySelector(
        '#text>a',
    ) as HTMLLinkElement | null;
    if (!channelATag?.textContent) {
        return { error: 'Channel name not found' };
    }

    const channelName = channelATag.textContent.trim();

    const channelHref = channelATag.getAttribute('href');
    if (!channelHref) {
        return { error: 'Channel href not found' };
    }

    let channelID = channelHref;

    if (channelID.startsWith('/channel/')) {
        channelID = channelID.replace('/channel/', '');
    } else {
        channelID = channelID.replace('/@', '');
    }

    const ownerSubCount = channelInfoElement.querySelector('#owner-sub-count');
    if (!ownerSubCount?.textContent) {
        return { error: 'Channel subscribers count not found' };
    }

    const subscribersCountChars = ownerSubCount.textContent
        .split(' ')[0]
        .toLowerCase()
        .split('');
    if (subscribersCountChars.length === 0) {
        return { error: 'Channel subscribers count not found' };
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
        '#owner #avatar #img',
    ) as HTMLImageElement | null;
    if (!avaterElement) {
        return { error: 'Channel avater not found' };
    }

    const avaterUrl = avaterElement.src.replace('=s48', '=s280');
    if (!avaterUrl) {
        return { error: 'Channel avater URL not found' };
    }

    const subscribeButton = document.querySelector('#subscribe-button-shape');
    if (!subscribeButton) {
        return { error: 'Subscribe button not found' };
    }

    let isSubscribed = false;

    if (subscribeButton.textContent !== 'Subscribe') {
        isSubscribed = true;
    }

    return {
        data: {
            id: channelID,
            name: channelName.trim(),
            url: `https://www.youtube.com${channelHref}`,
            is_subscribed: isSubscribed,
            subscribers_count: Math.round(subscribersCount),
            avater_url: avaterUrl.trim(),
        },
    };
}
