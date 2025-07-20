import { CreateWatchHistoryRequest } from '@bindings';
import { MessageType } from './types.d';

let lastProcessedUrl: string | null = null;

async function sendPendingData(endpoint: URL) {
    try {
        const storage = (await browser.storage.local.get('pendingData')) as {
            pendingData: CreateWatchHistoryRequest[];
        };

        try {
            await browser.storage.local.remove('pendingData');
        } catch {
            console.error('[background] Failed to remove `pendingData`');
        }

        storage.pendingData.forEach((data) => {
            console.log('[background] Sending pending data:', data);

            fetch(endpoint, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(data),
            })
                .then((res) => {
                    console.debug(res);
                })
                .catch((err) => {
                    console.error(`ERROR: Failed to send data data: ${err}`);
                    pendingDataAdd(data);
                });
        });
    } catch {}
}

function pendingDataAdd(data: CreateWatchHistoryRequest) {
    browser.storage.local
        .get('pendingData')
        .then((storage) => {
            const pendingData = storage.pendingData || [];
            pendingData.push(data);
            browser.storage.local.set({ pendingData });
        })
        .catch(() => {
            console.log('[background] Failed to get pendingData from storage');
            browser.storage.local.set({ pendingData: [data] });
        });
}

browser.storage.local
    .get('apiURL')
    .then((storage) => {
        console.log('[background] apiURL:', storage.apiURL);
        const pingUrl = new URL('/api/ping', storage.apiURL);
        fetch(pingUrl)
            .then(async (response) => {
                if (response.ok) {
                    console.log('[background] Connected to api');
                    const endpoint = new URL(
                        '/api/watch_history',
                        storage.apiURL,
                    );
                    await sendPendingData(endpoint);
                }
            })
            .catch(() => {
                console.log('[background] Failed to ping api');
            });
    })
    .catch(() => {
        console.log('[background] Failed to get apiURL from storage');
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

        data.sessionEndDate = Math.round(Number(Date.now() / 1000));

        console.debug(data);

        browser.storage.local.get('apiURL').then((storage) => {
            const apiURL = storage.apiURL;
            if (apiURL == null) return;

            const fullUrl = new URL('/api/watch_history', apiURL);
            fetch(fullUrl, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(data),
            })
                .then((res) => {
                    console.debug(res);
                })
                .catch((err) => {
                    console.error(`ERROR: Failed to send data: ${err}`);
                    pendingDataAdd(data);
                });
        });
    } else if (type === 'setApiURL') {
        const data = payload as string;
        browser.storage.local.set({ apiURL: data });
    } else if (type === 'getApiURL') {
        sendResponse(browser.storage.local.get('apiURL'));
    }
});
