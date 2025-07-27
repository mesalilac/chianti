import type { CreateWatchHistoryChannel } from '@bindings';

export function getChannelInfo(): CreateWatchHistoryChannel | null {
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

    const channelHref = channelATag.getAttribute('href');
    if (!channelHref) {
        console.error('[chianti] Channel href not found');
        return null;
    }

    let channelID = channelHref;

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
        '#owner #avatar #img',
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

    const subscribeButton = document.querySelector('#subscribe-button-shape');
    if (!subscribeButton) {
        console.error('[chianti] Subscribe button not found');
        return null;
    }

    let isSubscribed = false;

    if (subscribeButton.textContent !== 'Subscribe') {
        isSubscribed = true;
    }

    return {
        id: channelID,
        name: channelName,
        url: `https://www.youtube.com${channelHref}`,
        is_subscribed: isSubscribed,
        subscribers_count: Math.round(subscribersCount),
        avater_url: avaterUrl,
    };
}
