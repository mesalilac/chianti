let lastProcessedUrl: string | null = null;

// https://medium.com/@softvar/making-chrome-extension-smart-by-supporting-spa-websites-1f76593637e8
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

browser.runtime.onMessage.addListener((message, sender, sendResponse) => {
    const type = message.type;
    const data = message.payload;

    switch (type) {
        case 'recordHistory':
            browser.storage.local.get('apiURL').then((storage) => {
                const apiURL = storage.apiURL;
                if (apiURL === null) return;

                const fullUrl = new URL('/api/watch_history', apiURL);
                fetch(fullUrl, {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify(data),
                });
            });
            break;
        case 'setApiURL':
            browser.storage.local.set({ apiURL: data });
            break;
        case 'getApiURL':
            sendResponse(browser.storage.local.get('apiURL'));
            break;
    }
});
