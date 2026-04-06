'use client';

import { LoginIcon } from '@components/ui/login-icon';
import { Button } from '@components/ui/button';
import { IconCheck, IconMail, IconX } from '@tabler/icons-react';
import Link from 'next/link';
import { useSearchParams } from 'next/navigation';
import { Suspense } from 'react';

function ConfirmEmail() {
    const searchParams = useSearchParams();
    const status = searchParams.get('status');
    const reason = searchParams.get('reason');

    if (status === 'success') {
        return (
            <div className="flex flex-col items-center gap-4">
                <LoginIcon><IconCheck /></LoginIcon>
                <div className="mt-4 flex flex-col gap-1 text-center">
                    <h1 className="font-semibold text-xl">Email confirmed!</h1>
                    <p className="text-sm text-muted-foreground">Your email address has been verified.</p>
                </div>
                <div className="mt-2">
                    <Button asChild>
                        <Link href="/dashboard">Go to dashboard</Link>
                    </Button>
                </div>
            </div>
        );
    }

    if (status === 'error') {
        const message = reason === 'expired'
            ? 'This confirmation link has expired.'
            : 'This confirmation link is invalid or has already been used.';

        return (
            <div className="flex flex-col items-center gap-4">
                <LoginIcon><IconX /></LoginIcon>
                <div className="mt-4 flex flex-col gap-1 text-center">
                    <h1 className="font-semibold text-xl">{reason === 'expired' ? 'Link expired' : 'Invalid link'}</h1>
                    <p className="text-sm text-muted-foreground">{message}</p>
                </div>
                <div className="mt-2">
                    <Button variant="outline" asChild>
                        <Link href="/dashboard">Back to dashboard</Link>
                    </Button>
                </div>
            </div>
        );
    }

    return (
        <div className="flex flex-col items-center gap-4">
            <LoginIcon><IconMail /></LoginIcon>
            <div className="mt-4 flex flex-col gap-1 text-center">
                <h1 className="font-semibold text-xl">Invalid link</h1>
                <p className="text-sm text-muted-foreground">This confirmation link is missing required parameters.</p>
            </div>
        </div>
    );
}

export default function Page() {
    return (
        <Suspense>
            <ConfirmEmail />
        </Suspense>
    );
}
