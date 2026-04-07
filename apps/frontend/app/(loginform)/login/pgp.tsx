'use client';

import { Alert, AlertDescription, AlertTitle } from '@components/ui/alert';
import { Button } from '@components/ui/button';
import { LinkComponent } from '@components/ui/link';
import { LoginIcon } from '@components/ui/login-icon';
import { $api } from '@lib/providers/api';
import { IconAlertCircle, IconArrowRight, IconKey } from '@tabler/icons-react';
import { useSetAtom } from 'jotai';
import { useCallback, useEffect, useRef, useState } from 'react';
import { useFormContext } from 'react-hook-form';
import { useLoginSuccess } from '@lib/hooks';
import { FormSchema, screenAtom } from './page';
import { ChallengeStep } from './pgp/challenge-step';
import { QuickSignCommand } from './pgp/quick-sign-command';
import { SignatureStep } from './pgp/signature-step';

export function Pgp() {
    const setScreen = useSetAtom(screenAtom);
    const form = useFormContext<FormSchema>();
    const username = form.watch('username');
    const { onSuccess } = useLoginSuccess();

    const [challenge, setChallenge] = useState('');
    const [challengeError, setChallengeError] = useState('');

    const challengeRequest = $api.useMutation('get', '/api/login/pgp/challenge', {
        onSuccess: ({ challenge }) => {
            setChallenge(challenge);
            setChallengeError('');
        },
        onError: (e) => {
            if (!challenge) {
                setChallengeError('Failed to generate challenge. Try again in a moment.');
            }
        },
    });

    const pgpLogin = $api.useMutation('post', '/api/login/pgp/challenge', {
        onSuccess,
        onError: (e) => {
            form.setError('pgp_signature', {
                message: e?.error || 'Login failed.',
            });
        },
    });

    const challengeRequestRef = useRef(challengeRequest);
    challengeRequestRef.current = challengeRequest;

    const [refreshSpin, setRefreshSpin] = useState(0);

    const refreshChallenge = useCallback(() => {
        form.clearErrors('pgp_signature');
        challengeRequestRef.current.mutate({});
        setRefreshSpin((s) => s + 1);
    }, [form]);

    useEffect(() => {
        refreshChallenge();
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, []);

    const gpgCommand = challenge ? `echo "${challenge}" | gpg --clearsign` : '';

    return (
        <form
            className="flex flex-col items-center"
            onSubmit={form.handleSubmit((data) =>
                pgpLogin.mutate({
                    body: {
                        username: data.username,
                        signature: data.pgp_signature ?? '',
                    },
                })
            )}
        >
            <LoginIcon>
                <IconKey />
            </LoginIcon>
            <div className="mt-4 flex flex-col gap-1">
                <h1 className="font-semibold text-xl text-center">Sign in with a PGP key</h1>
                <p className="text-sm text-center text-muted-foreground">
                    Sign the server challenge for {username}{' '}
                    <LinkComponent onClick={() => setScreen('welcome')}>Not you?</LinkComponent>
                </p>
            </div>
            <div className="w-sm mt-6 flex flex-col gap-3">
                {challengeError && (
                    <Alert variant="destructive">
                        <IconAlertCircle />
                        <AlertTitle>Challenge Unavailable</AlertTitle>
                        <AlertDescription>{challengeError}</AlertDescription>
                    </Alert>
                )}

                <ChallengeStep
                    challenge={challenge}
                    refreshChallenge={refreshChallenge}
                    isPending={challengeRequest.isPending}
                    refreshSpin={refreshSpin}
                />

                {challenge && <QuickSignCommand gpgCommand={gpgCommand} />}

                <SignatureStep />

                <Button
                    type="submit"
                    disabled={!challenge || challengeRequest.isPending || pgpLogin.isPending}
                >
                    Next <IconArrowRight />
                </Button>
                <div className="text-muted-foreground text-center text-sm">
                    <LinkComponent>
                        <div onClick={() => setScreen('login-options')}>More Options</div>
                    </LinkComponent>
                </div>
            </div>
        </form>
    );
}
