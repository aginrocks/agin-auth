'use client';
import { $api } from '@lib/providers/api';
import { useLoginSuccess } from './use-login-success';
import { useWebAuthnAssertion } from './use-webauthn-core';

export function useWebAuthnPasswordless() {
    const { onSuccess } = useLoginSuccess();
    const begin = $api.useMutation('post', '/api/login/webauthn/passwordless/start');
    const finish = $api.useMutation('post', '/api/login/webauthn/passwordless/finish', { onSuccess });
    return useWebAuthnAssertion(begin, finish);
}
