import type { CreateWatchHistoryVideo } from '@bindings';
import type { Result } from '../types';

import { parseCommentsCount, parseLikes, parseRelativeDate } from './index';

export function extractVideoInfo(
    videoId: string,
): Result<CreateWatchHistoryVideo> {
    const videoTitleHeadingelement = document.querySelector(
        '#title>h1',
    ) as HTMLHeadingElement;
    if (!videoTitleHeadingelement.textContent) {
        return { error: 'Video title not found' };
    }

    const videoDurationElement = document.querySelector('.ytp-time-duration');
    if (!videoDurationElement?.textContent) {
        return { error: 'Video duration not found' };
    }

    // Expand description
    const bottomRowElement = document.querySelector('#bottom-row');
    if (!bottomRowElement) {
        return { error: 'Bottom row not found' };
    }

    const descriptionElement = bottomRowElement.querySelector(
        '#description',
    ) as HTMLButtonElement;
    if (!descriptionElement) {
        return { error: 'Description button not found' };
    }

    descriptionElement.click();

    const descriptionInnerElement =
        document.querySelector('#description-inner');
    if (!descriptionInnerElement) {
        return { error: 'Description inner element not found' };
    }

    const descriptionInfoContainer =
        descriptionInnerElement.querySelector('#info');
    if (
        !descriptionInfoContainer ||
        !descriptionInfoContainer.children[0].textContent ||
        !descriptionInfoContainer.children[2].textContent
    ) {
        return { error: 'Description info not found' };
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
        return { error: 'Collapse button not found' };
    }
    collapseElement.click();

    const likesButtonText = document.querySelector(
        'button-view-model .yt-spec-button-shape-next__button-text-content',
    );
    if (!likesButtonText?.textContent || likesButtonText.textContent === '') {
        return { error: 'Likes button text not found' };
    }

    const likesCount = parseLikes(likesButtonText.textContent);
    if (!likesCount) {
        return { error: 'Failed to parse likes count' };
    }

    const thumbnailUrl = `https://img.youtube.com/vi/${videoId}/hqdefault.jpg`;

    const commentsHeaderCountEle = document.querySelector(
        '#comments>#sections>#header #count span',
    );
    if (!commentsHeaderCountEle || !commentsHeaderCountEle.textContent) {
        console.error('[chianti] Comments count not found');
        return { error: 'Comments count not found' };
    }

    const commentsCount = parseCommentsCount(
        commentsHeaderCountEle.textContent,
    );

    return {
        data: {
            id: videoId,
            title: videoTitleHeadingelement.textContent.trim(),
            description: '',
            duration: Number(
                videoDurationElement.textContent
                    .split(':')
                    .reverse()
                    .reduce(
                        (prev, curr, i) => prev + Number(curr) * 60 ** i,
                        0,
                    ),
            ),
            tags: [],
            published_at: videoPublishDate,
            likes_count: likesCount,
            view_count: Number(
                tempVideoViews.split(' ')[0].replaceAll(',', ''),
            ),
            comments_count: commentsCount,
            thumbnail_url: thumbnailUrl,
        },
    };
}
