'use client';

import { LoginIcon } from '@components/ui/login-icon';
import { Button } from '@components/ui/button';
import { IconCheck, IconMail, IconX } from '@tabler/icons-react';
import { $api } from '@lib/providers/api';
import Link from 'next/link';
import { useSearchParams } from 'next/navigation';
import { useEffect, useRef } from 'react';
import { Suspense } from 'react';

function ConfirmEmail() {
    const searchParams = useSearchParams();
    const token = searchParams.get('token') ?? '';
    const called = useRef(false);

    const confirm = $api.useMutation('post', '/api/confirm-email');

    useEffect(() => {
        if (token && !called.current) {
            called.current = true;
            confirm.mutate({ body: { token } });
        }
    }, [token]);

    if (!token) {
        return (
            <div className="flex flex-col items-center gap-4">
                <LoginIcon><IconMail /></LoginIcon>
                <div className="mt-4 flex flex-col gap-1 text-center">
                    <h1 className="font-semibold text-xl">Invalid link</h1>
                    <p className="text-sm text-muted-foreground">This confirmation link is missing a token.</p>
                </div>
            </div>
        );
    }

    if (confirm.isPending) {
        return (
            <div className="flex flex-col items-center gap-4">
                <LoginIcon><IconMail /></LoginIcon>
                <div className="mt-4 flex flex-col gap-1 text-center">
                    <h1 className="font-semibold text-xl">Confirming your email…</h1>
                    <p className="text-sm text-muted-foreground">Please wait a moment.</p>
                </div>
            </div>
        );
    }

    if (confirm.isSuccess) {
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

    if (confirm.isError) {
        return (
            <div className="flex flex-col items-center gap-4">
                <LoginIcon><IconX /></LoginIcon>
                <div className="mt-4 flex flex-col gap-1 text-center">
                    <h1 className="font-semibold text-xl">Link expired</h1>
                    <p className="text-sm text-muted-foreground">
                        {(confirm.error as any)?.error ?? 'This confirmation link is invalid or has already been used.'}
                    </p>
                </div>
                <div className="mt-2">
                    <Button variant="outline" asChild>
                        <Link href="/dashboard">Back to dashboard</Link>
                    </Button>
                </div>
            </div>
        );
    }

    return null;
}

export default function Page() {
    return (
        <Suspense>
            <ConfirmEmail />
        </Suspense>
    );
}
