import type { CreateWatchHistoryVideo } from '@bindings';

import { parseCommentsCount, parseLikes, parseRelativeDate } from './index';

export function getVideoInfo(videoId: string): CreateWatchHistoryVideo | null {
    const videoTitleHeadingelement = document.querySelector(
        '#title>h1',
    ) as HTMLHeadingElement;
    if (!videoTitleHeadingelement.textContent) {
        console.error('[chianti] Video title not found');
        return null;
    }

    const videoDurationElement = document.querySelector('.ytp-time-duration');
    if (!videoDurationElement?.textContent) {
        console.error('[chianti] Video duration not found');
        return null;
    }

    // Expand description
    const bottomRowElement = document.querySelector('#bottom-row');
    if (!bottomRowElement) {
        console.error('[chianti] Bottom row not found');
        return null;
    }

    const descriptionElement = bottomRowElement.querySelector(
        '#description',
    ) as HTMLButtonElement;
    if (!descriptionElement) {
        console.error('[chianti] Description button not found');
        return null;
    }

    descriptionElement.click();

    const descriptionInnerElement =
        document.querySelector('#description-inner');
    if (!descriptionInnerElement) {
        console.error('[chianti] Description inner element not found');
        return null;
    }

    const descriptionInfoContainer =
        descriptionInnerElement.querySelector('#info');
    if (
        !descriptionInfoContainer ||
        !descriptionInfoContainer.children[0].textContent ||
        !descriptionInfoContainer.children[2].textContent
    ) {
        console.error('[chianti] Description info not found');
        return null;
    }

    const tempVideoViews = descriptionInfoContainer.children[0].textContent;
    let tempVideoPublishDate = descriptionInfoContainer.children[2].textContent;
    let videoPublishDate: number = 0;

    if (tempVideoPublishDate.startsWith('Premiered ')) {
        tempVideoPublishDate = tempVideoPublishDate.replace('Premiered ', '');
    }

    if (tempVideoPublishDate.includes('ago')) {
        videoPublishDate = parseRelativeDate(tempVideoPublishDate);
    } else {
        videoPublishDate = Number(
            new Date(tempVideoPublishDate).getTime() / 1000,
        );
    }

    // Collapse description
    const collapseElement = document.querySelector(
        '#collapse',
    ) as HTMLButtonElement;
    if (!collapseElement) {
        console.error('[chianti] Collapse button not found');
        return null;
    }
    collapseElement.click();

    const likesButtonText = document.querySelector(
        'button-view-model .yt-spec-button-shape-next__button-text-content',
    );
    if (!likesButtonText?.textContent || likesButtonText.textContent === '') {
        console.error('[chianti] Likes button text not found');
        return null;
    }

    const likesCount = parseLikes(likesButtonText.textContent);

    const thumbnailUrl = `https://img.youtube.com/vi/${videoId}/hqdefault.jpg`;

    const commentsHeaderCountEle = document.querySelector(
        '#comments>#sections>#header #count span',
    );
    if (!commentsHeaderCountEle || !commentsHeaderCountEle.textContent) {
        console.error('[chianti] Comments count not found');
        return null;
    }

    const commentsCount = parseCommentsCount(
        commentsHeaderCountEle.textContent,
    );

    return {
        id: videoId,
        title: videoTitleHeadingelement.textContent.trim(),
        description: '',
        duration: Number(
            videoDurationElement.textContent
                .split(':')
                .reverse()
                .reduce((prev, curr, i) => prev + Number(curr) * 60 ** i, 0),
        ),
        tags: [],
        published_at: videoPublishDate,
        likes_count: likesCount,
        view_count: Number(tempVideoViews.split(' ')[0].replaceAll(',', '')),
        comments_count: commentsCount,
        thumbnail_url: thumbnailUrl,
    };
}
