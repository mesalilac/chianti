import { MessageType, WatchHistoryBody } from './types.d';

let lastProcessedUrl: string | null = null;

// https://medium.com/@softvar/making-chrome-extension-smart-by-supporting-spa-websites-1f76593637e8
// NOTE: Message `page-rendered` is sent twice
//       - When you load a fresh page on /watch?=
//       - And click on a new video
//       - The `main` function runs twice for that new video
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
        const data = payload as WatchHistoryBody | null;
        if (!data) return;

        try {
            const res = await fetch(
                `https://www.youtube.com/watch?v=${data.video_id}`,
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
                    data.video_description =
                        videoDescription.getAttribute('content') || '';
                }

                videoPageDOM
                    .querySelectorAll("meta[property='og:video:tag']")
                    .forEach((meta) => {
                        const tag = meta.getAttribute('content');
                        if (tag) data.video_tags.push(tag);
                    });
            }
        } catch {}

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
            });
        });
    } else if (type === 'setApiURL') {
        const data = payload as string;
        browser.storage.local.set({ apiURL: data });
    } else if (type === 'getApiURL') {
        sendResponse(browser.storage.local.get('apiURL'));
    }
});
