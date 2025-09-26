'use client';
import { Button } from '@components/ui/button';
import { LinkComponent } from '@components/ui/link';
import { LoginIcon } from '@components/ui/login-icon';
import { LoginOption } from '@components/ui/login-option';
import { SCOPES_LIST } from '@lib/utils';
import { IconExternalLink, IconPassword } from '@tabler/icons-react';
import Link from 'next/link';
import { useSearchParams } from 'next/navigation';

export default function Page() {
    const params = useSearchParams();
    const scopes = params.get('scopes')?.split(' ') || [];
    const validScopes = SCOPES_LIST.map((s) => s.scope).filter((s) => scopes.includes(s));

    const username = 'User';
    const appName = 'Test App';

    return (
        <>
            <LoginIcon>
                <IconExternalLink />
            </LoginIcon>
            <div className="mt-4 flex flex-col gap-1">
                <h1 className="font-semibold text-xl text-center">Sign in to {appName}</h1>
                <p className="text-sm text-center text-muted-foreground">
                    {validScopes.length !== 0 ? (
                        <>{appName} will be granted the following permissions:</>
                    ) : (
                        <>{appName} will not be granted any permissions.</>
                    )}
                </p>
            </div>

            <div className="w-sm mt-4 flex flex-col gap-3">
                {SCOPES_LIST.filter((s) => scopes.includes(s.scope)).map((s) => (
                    <LoginOption {...s.props} key={s.scope} className="m-0" />
                ))}
                <div className="flex flex-col gap-2.5 text-muted-foreground">
                    <Button className="mt-1">Authorize {appName}</Button>
                    <Button variant="ghost">Cancel</Button>
                </div>
            </div>
        </>
    );
}
