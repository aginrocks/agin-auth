import { LinkComponent } from '@components/ui/link';
import { LoginIcon } from '@components/ui/login-icon';
import { LoginOption } from '@components/ui/login-option';
import { SCOPES_MAP } from '@lib/utils';
import { IconExternalLink, IconPassword } from '@tabler/icons-react';
import Link from 'next/link';

export default function Page() {
    const username = 'User';
    const appName = 'Test App';
    const scopes = ['profile', 'email', 'offline_access'];
    return (
        <>
            <LoginIcon>
                <IconExternalLink />
            </LoginIcon>
            <div className="mt-4 flex flex-col gap-1">
                <h1 className="font-semibold text-xl text-center">Sign in to {appName}</h1>
                <p className="text-sm text-center text-muted-foreground">
                    {appName} will have access to this information about you:
                </p>
            </div>

            <div className="w-sm mt-4 flex flex-col gap-3">
                {scopes
                    .filter((s) => !!SCOPES_MAP[s])
                    .map((s) => (
                        <LoginOption {...SCOPES_MAP[s]} key={s} className="m-0" />
                    ))}
            </div>
        </>
    );
}
