'use client';
import { $api } from '@lib/providers/api';
import { useCallback, useState } from 'react';
import { Base64 } from 'js-base64';

export function useWebAuthn2FA() {
    const [isPending, setIsPending] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const begin = $api.useMutation('post', '/api/login/webauthn/start');
    const finish = $api.useMutation('post', '/api/login/webauthn/finish');

    const loginAsync = useCallback(async () => {
        try {
            setIsPending(true);
            const credentialRequestOptions = await begin.mutateAsync({});

            credentialRequestOptions.publicKey.challenge = Base64.toUint8Array(
                credentialRequestOptions.publicKey.challenge
            ) as any;
            credentialRequestOptions.publicKey.allowCredentials?.forEach(function (listItem) {
                listItem.id = Base64.toUint8Array(listItem.id) as any;
            });

            const assertion = await navigator.credentials.get({
                // @ts-expect-error Someone fucked up Credential type definitions
                publicKey: credentialRequestOptions.publicKey,
            });

            if (!assertion) {
                alert('error!');
                return;
            }

            await finish.mutateAsync({
                body: {
                    id: assertion.id,
                    // @ts-expect-error Someone fucked up Credential type definitions
                    rawId: Base64.fromUint8Array(new Uint8Array(assertion.rawId), true),
                    type: assertion.type,
                    response: {
                        authenticatorData: Base64.fromUint8Array(
                            // @ts-expect-error Someone fucked up Credential type definitions
                            new Uint8Array(assertion.response.authenticatorData),
                            true
                        ),
                        clientDataJSON: Base64.fromUint8Array(
                            // @ts-expect-error Someone fucked up Credential type definitions
                            new Uint8Array(assertion.response.clientDataJSON),
                            true
                        ),
                        signature: Base64.fromUint8Array(
                            // @ts-expect-error Someone fucked up Credential type definitions
                            new Uint8Array(assertion.response.signature),
                            true
                        ),
                        userHandle: Base64.fromUint8Array(
                            // @ts-expect-error Someone fucked up Credential type definitions
                            new Uint8Array(assertion.response.userHandle),
                            true
                        ),
                    },
                },
            });
            setError(null);
        } catch (error) {
            setError(error as any);
        }
        setIsPending(false);
    }, []);

    return { loginAsync, isPending, error };
}
