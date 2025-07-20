window.onload = () => {
    const api_connection_status = document.getElementById(
        'api-connection-status',
    ) as HTMLSpanElement;

    const api_url_input = document.getElementById(
        'api-url-input',
    ) as HTMLInputElement;

    browser.storage.local
        .get('apiURL')
        .then((response) => {
            api_url_input.value = response.apiURL;
            const fullUrl = new URL('/api/ping', response.apiURL);
            fetch(fullUrl)
                .then(() => {
                    api_connection_status.textContent = 'Connected';
                    api_connection_status.classList.add('status-connected');
                })
                .catch(() => {
                    api_connection_status.textContent = 'Not connected';
                    api_connection_status.classList.add('status-not-connected');
                });
        })
        .catch(() => {
            api_url_input.value = 'http://localhost:8080';
        });

    const save_button = document.getElementById(
        'save-button',
    ) as HTMLButtonElement;

    save_button.addEventListener('click', () => {
        const api_url = document.getElementById(
            'api-url-input',
        ) as HTMLInputElement;

        browser.storage.local.set({ apiURL: api_url.value });
    });

    const pending_data_count = document.getElementById(
        'pending-data-count',
    ) as HTMLSpanElement;

    browser.storage.local.get('pendingData').then((response) => {
        pending_data_count.textContent = response.pendingData.length;
    });

    const clear_pending_data_button = document.getElementById(
        'clear-pending-data-button',
    ) as HTMLButtonElement;

    clear_pending_data_button.addEventListener('click', () => {
        browser.storage.local.remove('pendingData').then(() => {
            pending_data_count.textContent = '0';
        });
    });

    const send_pending_data_button = document.getElementById(
        'send-pending-data-button',
    ) as HTMLButtonElement;

    send_pending_data_button.addEventListener('click', () => {
        browser.runtime.sendMessage({
            type: 'sendPendingData',
        });
    });
};
