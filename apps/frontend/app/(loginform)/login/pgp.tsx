'use client';

import { Alert, AlertDescription, AlertTitle } from '@components/ui/alert';
import { Button } from '@components/ui/button';
import { FormControl, FormField, FormItem, FormMessage } from '@components/ui/form';
import { LinkComponent } from '@components/ui/link';
import { LoginIcon } from '@components/ui/login-icon';
import { $api } from '@lib/providers/api';
import {
    IconAlertCircle,
    IconArrowRight,
    IconCheck,
    IconCopy,
    IconKey,
    IconRefresh,
    IconTerminal2,
    IconChevronDown,
} from '@tabler/icons-react';
import { useSetAtom } from 'jotai';
import { useCallback, useEffect, useRef, useState } from 'react';
import { useFormContext } from 'react-hook-form';
import { useLoginSuccess } from '@lib/hooks';
import { FormSchema, screenAtom } from './page';
import { AnimatePresence, motion } from 'motion/react';

function CopyButton({ onCopy, copied }: { onCopy: () => void; copied: boolean }) {
    return (
        <button
            type="button"
            onClick={onCopy}
            className="flex items-center gap-1 text-xs text-muted-foreground hover:text-foreground transition-colors w-[52px]"
        >
            {copied ? <IconCheck className="size-3" /> : <IconCopy className="size-3" />}
            {copied ? 'Copied' : 'Copy'}
        </button>
    );
}

export function Pgp() {
    const setScreen = useSetAtom(screenAtom);
    const form = useFormContext<FormSchema>();
    const username = form.watch('username');
    const { onSuccess } = useLoginSuccess();

    const [challenge, setChallenge] = useState('');
    const [challengeError, setChallengeError] = useState('');
    const [copied, setCopied] = useState(false);
    const [cmdCopied, setCmdCopied] = useState(false);
    const [showCommand, setShowCommand] = useState(false);

    const challengeRequest = $api.useMutation('get', '/api/login/pgp/challenge', {
        onSuccess: ({ challenge }) => {
            setChallenge(challenge);
            setChallengeError('');
            setCopied(false);
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

    const copyText = useCallback(async (text: string, setter: (v: boolean) => void) => {
        try {
            await navigator.clipboard.writeText(text);
            setter(true);
            setTimeout(() => setter(false), 1500);
        } catch {
            /* noop */
        }
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

                {/* Step 1: Challenge */}
                <div className="space-y-1.5">
                    <div className="flex items-center justify-between">
                        <label className="text-sm font-medium">
                            <span className="text-muted-foreground mr-1.5">1.</span>
                            Copy the challenge
                        </label>
                        <div className="flex items-center gap-2">
                            <CopyButton onCopy={() => copyText(challenge, setCopied)} copied={copied} />
                            <button
                                type="button"
                                onClick={refreshChallenge}
                                disabled={challengeRequest.isPending}
                                className="text-muted-foreground hover:text-foreground transition-colors disabled:opacity-50"
                            >
                                <motion.span
                                    key={refreshSpin}
                                    animate={{ rotate: -360 }}
                                    transition={{ type: 'spring', stiffness: 200, damping: 15 }}
                                    className="inline-flex"
                                >
                                    <IconRefresh className="size-3.5" />
                                </motion.span>
                            </button>
                        </div>
                    </div>
                    <div className="rounded-md border border-input dark:bg-input/30 bg-transparent px-3 py-2.5 font-mono text-xs break-all select-all cursor-text">
                        {challenge || <span className="text-muted-foreground">Generating...</span>}
                    </div>
                </div>

                {/* Quick sign helper (collapsible) */}
                {challenge && (
                    <div>
                        <button
                            type="button"
                            onClick={() => setShowCommand(!showCommand)}
                            className="flex items-center gap-1.5 text-xs text-muted-foreground hover:text-foreground transition-colors"
                        >
                            <IconTerminal2 className="size-3.5" />
                            <span>Quick sign command</span>
                            <motion.span
                                animate={{ rotate: showCommand ? 180 : 0 }}
                                transition={{ duration: 0.2 }}
                                className="inline-flex"
                            >
                                <IconChevronDown className="size-3" />
                            </motion.span>
                        </button>
                        <AnimatePresence initial={false}>
                            {showCommand && (
                                <motion.div
                                    initial={{ height: 0, opacity: 0 }}
                                    animate={{ height: 'auto', opacity: 1 }}
                                    exit={{ height: 0, opacity: 0 }}
                                    transition={{ duration: 0.2, ease: 'easeInOut' }}
                                    className="overflow-hidden"
                                >
                                    <div className="mt-1.5 rounded-md border border-input dark:bg-input/30 bg-transparent px-3 py-2.5 flex items-start justify-between gap-2">
                                        <pre className="font-mono text-[11px] text-muted-foreground whitespace-pre-wrap break-all select-all flex-1">
                                            {gpgCommand}
                                        </pre>
                                        <CopyButton onCopy={() => copyText(gpgCommand, setCmdCopied)} copied={cmdCopied} />
                                    </div>
                                </motion.div>
                            )}
                        </AnimatePresence>
                    </div>
                )}

                {/* Step 2: Paste signature */}
                <div className="space-y-1.5">
                    <label className="text-sm font-medium">
                        <span className="text-muted-foreground mr-1.5">2.</span>
                        Paste the signed message
                    </label>
                    <FormField
                        control={form.control}
                        name="pgp_signature"
                        render={({ field }) => (
                            <FormItem>
                                <FormControl>
                                    <textarea
                                        {...field}
                                        autoFocus
                                        placeholder="-----BEGIN PGP SIGNED MESSAGE-----"
                                        className="min-h-28 w-full resize-none rounded-md border border-input dark:bg-input/30 bg-transparent px-3 py-2.5 font-mono text-xs placeholder:text-muted-foreground shadow-xs transition-[color,box-shadow] outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]"
                                    />
                                </FormControl>
                                <FormMessage />
                            </FormItem>
                        )}
                    />
                </div>

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
