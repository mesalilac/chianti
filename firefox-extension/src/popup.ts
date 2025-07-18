window.onload = () => {
    const api_connection_status = document.getElementById(
        'api-connection-status',
    ) as HTMLSpanElement;

    const api_url_input = document.getElementById(
        'api-url-input',
    ) as HTMLInputElement;

    if (api_url_input === null || api_connection_status === null) return;

    browser.runtime
        .sendMessage({
            type: 'getApiURL',
        })
        .then((response) => {
            api_url_input.value = response.apiURL;
            const fullUrl = new URL('/api/ping', response.apiURL);
            fetch(fullUrl)
                .then((response) => {
                    if (response.ok) {
                        api_connection_status.textContent = 'Connected';
                    } else {
                        api_connection_status.textContent = 'Not connected';
                    }
                })
                .catch(() => {
                    api_connection_status.textContent = 'Not connected';
                });
        })
        .catch(() => {
            api_url_input.value = '';
        });

    const save_button = document.getElementById(
        'save-button',
    ) as HTMLButtonElement;

    save_button.addEventListener('click', () => {
        const api_url = document.getElementById(
            'api-url-input',
        ) as HTMLInputElement;

        browser.runtime.sendMessage({
            type: 'setApiURL',
            payload: api_url.value,
        });
    });
};
