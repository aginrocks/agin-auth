import { LoginIcon } from '@components/ui/login-icon';
import { IconAlertCircle, IconFingerprint } from '@tabler/icons-react';
import { screenAtom } from './page';
import { LinkComponent } from '@components/ui/link';
import { useSetAtom } from 'jotai';
import { useCallback } from 'react';
import { useWebAuthnPasswordless } from '@lib/hooks';
import { Button } from '@components/ui/button';
import { useHotkeys } from '@mantine/hooks';
import { Alert, AlertDescription, AlertTitle } from '@components/ui/alert';

export function WebAuthnPasswordless() {
    const setScreen = useSetAtom(screenAtom);

    const webauthn = useWebAuthnPasswordless();

    const startAuth = useCallback(async () => {
        await webauthn.loginAsync();
    }, []);

    useHotkeys([['Enter', startAuth]]);

    return (
        <div className="flex flex-col items-center">
            <LoginIcon>
                <IconFingerprint />
            </LoginIcon>
            <div className="mt-4 flex flex-col gap-1">
                <h1 className="font-semibold text-xl text-center">Sign in with a passkey</h1>
                <p className="text-sm text-center text-muted-foreground">
                    Use a passkey stored on your device to sign in without a password.
                </p>
            </div>
            <div className="w-sm mt-6 flex flex-col gap-4">
                {webauthn.error && (
                    <Alert variant="destructive">
                        <IconAlertCircle />
                        <AlertTitle>Authentication Failed</AlertTitle>
                        <AlertDescription>
                            An error occurred during authentication.
                        </AlertDescription>
                    </Alert>
                )}
                <Button onClick={startAuth} disabled={webauthn.isPending}>
                    <IconFingerprint /> Use passkey
                </Button>
                <div className="text-muted-foreground text-center text-sm">
                    <LinkComponent>
                        <div onClick={() => setScreen('welcome')}>Back to login</div>
                    </LinkComponent>
                </div>
            </div>
        </div>
    );
}
