window.onload = () => {
    const api_connection_status = document.getElementById(
        'api-connection-status',
    );

    const api_url_input = document.getElementById('api-url-input');

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
        });

    const save_button = document.getElementById('save-button');
    save_button.addEventListener('click', () => {
        const api_url = document.getElementById('api-url-input').value;
        browser.runtime.sendMessage({
            type: 'setApiURL',
            payload: api_url,
        });
    });
};
