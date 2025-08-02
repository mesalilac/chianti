export * from './extract_channel_info';
export * from './extract_video_info';

export const delay = (ms: number) => new Promise((res) => setTimeout(res, ms));

export function parseRelativeDate(dateString: string): number {
    const parts = dateString.split(' ');
    if (parts.length < 3 || parts[2].toLowerCase() !== 'ago') return NaN;

    const value: number = Number(parts[0]);
    let unit: string = parts[1].toLowerCase();
    if (isNaN(value)) return NaN;

    if (unit.endsWith('s') && value === 1) unit = unit.slice(0, -1);

    const resultDate = new Date();

    switch (unit) {
        case 'second':
        case 'seconds':
            resultDate.setSeconds(resultDate.getSeconds() - value);
            break;
        case 'minute':
        case 'minutes':
            resultDate.setMinutes(resultDate.getMinutes() - value);
            break;
        case 'hour':
        case 'hours':
            resultDate.setHours(resultDate.getHours() - value);
            break;
        case 'day':
        case 'days':
            resultDate.setDate(resultDate.getDate() - value);
            break;
        case 'week':
        case 'weeks':
            resultDate.setDate(resultDate.getDate() - value * 7);
            break;
        case 'month':
        case 'months':
            resultDate.setMonth(resultDate.getMonth() - value);
            break;
        case 'year':
        case 'years':
            resultDate.setFullYear(resultDate.getFullYear() - value);
            break;
        default:
            return NaN;
    }

    return Math.floor(resultDate.getTime() / 1000);
}

export function parseLikes(str: string): number {
    str = str.toLowerCase();

    const arr: string[] = str.split('');
    let multiplier = 1;

    switch (arr[arr.length - 1]) {
        case 'k':
            arr.pop();
            multiplier = 1000;
            break;
        case 'm':
            arr.pop();
            multiplier = 1000000;
            break;
        case 'b':
            arr.pop();
            multiplier = 1000000000;
            break;
    }

    if (str.includes('.')) {
        return parseFloat(arr.join('')) * multiplier;
    } else {
        return Number(arr.join('')) * multiplier;
    }
}

export function parseCommentsCount(str: string): number {
    str = str.toLowerCase();
    str = str.replaceAll(',', '');

    const chars = str.split('');

    switch (chars[chars.length - 1]) {
        case 'k':
            chars.pop();
            return Number(chars.join('')) * 1000;
        case 'm':
            chars.pop();
            return Number(chars.join('')) * 1000000;
        case 'b':
            chars.pop();
            return Number(chars.join('')) * 1000000000;
        default:
            return Number(chars.join(''));
    }
}

export function isCommentsDisabled() {
    const spanElement = document.querySelector(
        'yt-formatted-string.ytd-message-renderer:nth-child(3) > span:nth-child(1)',
    )?.textContent;

    if (!spanElement) return false;

    return spanElement.trim().toLowerCase() === 'comments are turned off.';
}
