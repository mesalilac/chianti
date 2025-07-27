import { createSignal, onMount } from 'solid-js';
import browser from 'webextension-polyfill';

type ServerStatus = 'Connected' | 'Not connected' | 'Unknown';

function App() {
    const [apiUrl, setApiUrl] = createSignal<string | undefined>(undefined);
    const [serverStatus, setServerStatus] =
        createSignal<ServerStatus>('Unknown');
    const [pendingDataCount, setPendingDataCount] = createSignal<number>(0);
    const [watchedVideoCount, setWatchedVideoCount] = createSignal<number>(0);

    function checkServerStatus() {
        const fullUrl = new URL('/api/ping', apiUrl());
        fetch(fullUrl)
            .then(() => {
                setServerStatus('Connected');
            })
            .catch(() => {
                setServerStatus('Not connected');
            });
    }

    function fetchPendingDataCount() {
        browser.storage.local
            .get('pendingData')
            .then((response) => {
                setPendingDataCount(response.pendingData.length || 0);
            })
            .catch(() => {
                setPendingDataCount(0);
            });
    }

    onMount(() => {
        browser.storage.local
            .get('apiURL')
            .then((response) => {
                if (!response.apiURL) setApiUrl('http://localhost:8080');

                setApiUrl(response.apiURL);
                checkServerStatus();
            })
            .catch(() => {
                setApiUrl('http://localhost:8080');
            });
        browser.storage.session
            .get('watchedVideosCount')
            .then((storage) => {
                if (!storage.watchedVideosCount) {
                    browser.storage.session.set({ watchedVideosCount: 0 });
                    return;
                }

                setWatchedVideoCount(storage.watchedVideosCount);
            })
            .catch(() => {
                browser.storage.session.set({ watchedVideosCount: 0 });
            });

        fetchPendingDataCount();
    });

    function saveApiUrl() {
        browser.storage.local.set({ apiURL: apiUrl() }).then(() => {
            checkServerStatus();
        });
    }

    function clearPendingData() {
        browser.storage.local.remove('pendingData').then(() => {
            setPendingDataCount(0);
        });
    }

    function sendPendingData() {
        browser.runtime
            .sendMessage({
                type: 'sendPendingData',
            })
            .then(() => {
                fetchPendingDataCount();
            });
    }

    return (
        <>
            <div id='base-api-url'>
                <span>Base api url</span>
                <input
                    type='text'
                    value={apiUrl() || ''}
                    onChange={(e) => setApiUrl(e.target.value)}
                />
                <button onClick={saveApiUrl}>Save</button>
                <span
                    class={
                        serverStatus() === 'Connected'
                            ? 'status-connected'
                            : 'status-not-connected'
                    }
                >
                    {serverStatus()}
                </span>
            </div>
            <div id='pending-data'>
                <span>
                    Pending data <b>{pendingDataCount()}</b>
                </span>
                <button onClick={clearPendingData}>Clear</button>
                <button onClick={sendPendingData}>Send data</button>
            </div>
            <div>
                <span>
                    Videos watched this session <b>{watchedVideoCount()}</b>
                </span>
            </div>
        </>
    );
}

export default App;
