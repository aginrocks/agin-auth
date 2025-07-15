'use client';
import { $api } from '@lib/providers/api';
import { useCallback } from 'react';
import { Base64 } from 'js-base64';

export function useWebAuthnRegistration() {
    const begin = $api.useMutation('post', '/api/settings/factors/webauthn/start');
    const finish = $api.useMutation('post', '/api/settings/factors/webauthn/finish');

    const registerAsync = useCallback(async (name: string) => {
        const credentialCreationOptions = await begin.mutateAsync({
            body: {
                display_name: name,
            },
        });

        credentialCreationOptions.publicKey.challenge = Base64.toUint8Array(
            credentialCreationOptions.publicKey.challenge
        ) as any;
        credentialCreationOptions.publicKey.user.id = Base64.toUint8Array(
            credentialCreationOptions.publicKey.user.id
        ) as any;
        credentialCreationOptions.publicKey.excludeCredentials?.forEach(function (listItem) {
            listItem.id = Base64.toUint8Array(listItem.id) as any;
        });

        const credential = await navigator.credentials.create({
            publicKey: credentialCreationOptions.publicKey as any,
        });

        if (!credential) {
            alert('error!');
            return;
        }

        await finish.mutateAsync({
            body: {
                id: credential.id,
                // @ts-expect-error Someone fucked up Credential type definitions
                rawId: Base64.fromUint8Array(new Uint8Array(credential.rawId), true),
                type: credential.type,
                response: {
                    attestationObject: Base64.fromUint8Array(
                        // @ts-expect-error Someone fucked up Credential type definitions
                        new Uint8Array(credential.response.attestationObject),
                        true
                    ),
                    clientDataJSON: Base64.fromUint8Array(
                        // @ts-expect-error Someone fucked up Credential type definitions
                        new Uint8Array(credential.response.clientDataJSON),
                        true
                    ),
                },
            },
        });
    }, []);

    return { registerAsync };
}
