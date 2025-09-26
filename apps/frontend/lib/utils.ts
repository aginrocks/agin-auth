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
