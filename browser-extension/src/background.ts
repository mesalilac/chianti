import browser from 'webextension-polyfill';

browser.runtime.onInstalled.addListener((details) => {
    console.log('Extension installed:', details);
});

import type { CreateWatchHistoryRequest } from '@bindings';
import type { MessageType } from './types.d';

let lastProcessedUrl: string | null = null;

function pendingDataAdd(data: CreateWatchHistoryRequest[]) {
    browser.storage.local
        .get('pendingData')
        .then((storage) => {
            const pendingData = storage.pendingData || [];
            data.forEach((x) => pendingData.push(x));
            browser.storage.local.set({ pendingData });
        })
        .catch(() => {
            console.error('Failed to get pendingData from storage');
            browser.storage.local.set({ pendingData: [data] });
        });
}

async function sendData(endpoint: URL, data: CreateWatchHistoryRequest[]) {
    try {
        const res = await fetch(endpoint, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(data),
        });

        console.debug(res);
    } catch {
        console.error('Failed to send data');
        pendingDataAdd(data);
    }
}

async function sendPendingData(endpoint: URL) {
    try {
        const storage = (await browser.storage.local.get('pendingData')) as {
            pendingData: CreateWatchHistoryRequest[];
        };

        try {
            await browser.storage.local.remove('pendingData');
        } catch {
            console.error('Failed to remove `pendingData`');
        }

        if (storage.pendingData.length === 0) return;

        sendData(endpoint, storage.pendingData);
    } catch {}
}

function sendNotifications(message: string) {
    browser.notifications.create({
        type: 'basic',
        title: 'Chianti',
        message,
    });
}

browser.storage.local
    .get('apiURL')
    .then((storage) => {
        console.debug('apiURL:', storage.apiURL);
        const pingUrl = new URL('/api/ping', storage.apiURL);
        fetch(pingUrl)
            .then((response) => {
                if (response.ok) {
                    console.log('Connected to api');
                    const endpoint = new URL(
                        '/api/watch_history',
                        storage.apiURL,
                    );
                    sendPendingData(endpoint);
                }
            })
            .catch(() => {
                console.error('Failed to ping api');

                sendNotifications(`Server is offline ${storage.apiURL}`);
            });
    })
    .catch(() => {
        console.error('Failed to get apiURL from storage');

        sendNotifications(`Please set a base api url`);
    });

// https://medium.com/@softvar/making-chrome-extension-smart-by-supporting-spa-websites-1f76593637e8
// NOTE: Message `page-rendered` is sent twice
//       - When you load a fresh page on /watch?=
//       - And click on a new video
//       - The `main` function runs twice for that new video
//
browser.webNavigation.onHistoryStateUpdated.addListener((e) => {
    if (e.frameId !== 0) return;
    if (e.transitionType !== 'link') return;
    if (e.transitionQualifiers.includes('forward_back')) return;

    const url = new URL(e.url);

    if (url.host !== 'www.youtube.com') return;
    if (url.pathname !== '/watch') return;
    if (url.searchParams.get('v') === null) return;

    if (lastProcessedUrl === url.href) return;
    lastProcessedUrl = url.href;

    const tabId = e.tabId;

    browser.tabs.sendMessage(tabId, {
        type: 'page-rendered',
    });
});

browser.runtime.onMessage.addListener(async (message, sender, sendResponse) => {
    const type: MessageType = message.type;
    const payload = message.payload;

    if (type === 'recordHistory') {
        const data = payload as CreateWatchHistoryRequest | null;
        if (!data) return;

        try {
            const res = await fetch(
                `https://www.youtube.com/watch?v=${data.video.id}`,
            );

            if (res.status === 200) {
                const videoPageText = await res.text();
                const videoPageDOM = new DOMParser().parseFromString(
                    videoPageText,
                    'text/html',
                );

                const videoDescription = videoPageDOM.querySelector(
                    "meta[property='og:description']",
                );

                if (videoDescription) {
                    data.video.description =
                        videoDescription.getAttribute('content') || '';
                }

                videoPageDOM
                    .querySelectorAll("meta[property='og:video:tag']")
                    .forEach((meta) => {
                        const tag = meta.getAttribute('content');
                        if (tag) data.video.tags.push(tag);
                    });
            }
        } catch {}

        data.session_end_date = Math.round(Number(Date.now() / 1000));

        console.dir(data);

        browser.storage.session.get('watchedVideosCount').then((storage) => {
            if (!storage) return;

            browser.storage.session.set({
                watchedVideosCount: storage.watchedVideosCount + 1,
            });
        });

        browser.storage.local.get('apiURL').then((storage) => {
            const apiURL = storage.apiURL;
            if (apiURL == null) return;

            const endpoint = new URL('/api/watch_history', apiURL);

            sendData(endpoint, [data]);
        });
    } else if (type === 'sendPendingData') {
        browser.storage.local.get('apiURL').then((storage) => {
            const apiURL = storage.apiURL;
            if (apiURL == null) return;

            const endpoint = new URL('/api/watch_history', apiURL);

            sendPendingData(endpoint);
        });
    } else {
        console.error('Unknown message type:', type);
    }
});
