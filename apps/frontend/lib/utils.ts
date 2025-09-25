import { LoginOptionProps } from '@components/ui/login-option';
import { IconHourglass, IconMail, IconUser } from '@tabler/icons-react';
import { clsx, type ClassValue } from 'clsx';
import { twMerge } from 'tailwind-merge';

export function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs));
}

export const SCOPES_MAP: Record<string, LoginOptionProps> = {
    profile: {
        title: 'General profile info',
        icon: IconUser,
    },
    email: {
        title: 'Email address',
        icon: IconMail,
    },
    offline_access: {
        title: 'Stay signed in for longer',
        icon: IconHourglass,
    },
};
