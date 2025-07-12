async function main() {
    const urlParams = new URLSearchParams(window.location.search);

    const videoID = urlParams.get('v');

    if (!videoID) {
        return;
    }

    const delay = (ms) => new Promise((res) => setTimeout(res, ms));

    await delay(4000);

    const videoTitle = document.querySelector('#title>h1').textContent.trim();
    const channelInfoTag = document
        .querySelector('#upload-info')
        .querySelector('#text>a');
    const channelName = channelInfoTag.textContent.trim();
    var channelID = channelInfoTag.getAttribute('href');

    if (channelID.startsWith('/channel/')) {
        channelID = channelID.replace('/channel/', '');
    } else {
        channelID = channelID.replace('/@', '');
    }

    const videoDurationString =
        document.querySelector('.ytp-time-duration').textContent;

    // Expand description
    document.querySelector('#bottom-row').querySelector('#description').click();

    const descriptionInfoContainer = document
        .querySelector('#description-inner')
        .querySelector('#info');

    const tempVideoViews = descriptionInfoContainer.children[0].textContent;
    const tempVideoPublishDate =
        descriptionInfoContainer.children[2].textContent;

    // Collapse description
    document.querySelector('#collapse').click();

    const payload = {
        // For channel
        channel_id: channelID,
        channel_name: channelName,
        // For video
        video_id: videoID,
        video_title: videoTitle,
        video_duration: Number(
            videoDurationString
                .split(':')
                .reverse()
                .reduce((prev, curr, i) => prev + curr * Math.pow(60, i), 0),
        ),
        published_at: Number(new Date(tempVideoPublishDate).getTime() / 1000),
        view_count: Number(tempVideoViews.split(' ')[0].replaceAll(',', '')),
    };

    browser.runtime.sendMessage({
        type: 'recordHistory',
        payload: payload,
    });
}

// First time load
// if you open the video in a new tab, this will be called
main();

// handle clicking on another video
browser.runtime.onMessage.addListener((message) => {
    if (message.type == 'page-rendered') {
        main();
    }
});
