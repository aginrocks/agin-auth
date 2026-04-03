'use client';
import { $api } from '@lib/providers/api';
import { useLoginSuccess } from './use-login-success';
import { useWebAuthnAssertion } from './use-webauthn-core';

export function useWebAuthn2FA() {
    const { onSuccess } = useLoginSuccess();
    const begin = $api.useMutation('post', '/api/login/webauthn/start');
    const finish = $api.useMutation('post', '/api/login/webauthn/finish', { onSuccess });
    return useWebAuthnAssertion(begin, finish);
}
