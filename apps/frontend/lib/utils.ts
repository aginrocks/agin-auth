import { LoginOptionProps } from '@components/ui/login-option';
import { IconHourglass, IconMail, IconUser } from '@tabler/icons-react';
import { clsx, type ClassValue } from 'clsx';
import { twMerge } from 'tailwind-merge';

export function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs));
}

export const SCOPES_LIST: { scope: string; props: LoginOptionProps }[] = [
    {
        scope: 'profile',
        props: {
            title: 'View general profile info',
            icon: IconUser,
        },
    },
    {
        scope: 'email',
        props: {
            title: 'View your email address',
            icon: IconMail,
        },
    },
    {
        scope: 'offline_access',
        props: {
            title: 'Stay signed in for longer',
            icon: IconHourglass,
        },
    },
];

export function parseUserAgent(ua: string) {
    if (ua === 'unknown') return { browser: 'Unknown', os: 'Unknown' };

    let browser = 'Unknown';
    let os = 'Unknown';

    if (ua.includes('Firefox/')) browser = 'Firefox';
    else if (ua.includes('Edg/')) browser = 'Edge';
    else if (ua.includes('Chrome/')) browser = 'Chrome';
    else if (ua.includes('Safari/')) browser = 'Safari';

    if (ua.includes('iPhone') || ua.includes('iPad')) os = 'iOS';
    else if (ua.includes('Windows')) os = 'Windows';
    else if (ua.includes('Mac OS')) os = 'macOS';
    else if (ua.includes('Android')) os = 'Android';
    else if (ua.includes('Linux')) os = 'Linux';

    return { browser, os };
}

export function timeAgo(dateStr: string) {
    const date = new Date(dateStr);
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffMin = Math.floor(diffMs / 60000);
    const diffHr = Math.floor(diffMin / 60);
    const diffDay = Math.floor(diffHr / 24);

    if (diffMin < 1) return 'just now';
    if (diffMin < 60) return `${diffMin}m ago`;
    if (diffHr < 24) return `${diffHr}h ago`;
    if (diffDay < 30) return `${diffDay}d ago`;
    return date.toLocaleDateString();
}
