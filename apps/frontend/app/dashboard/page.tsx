'use client';

import { motion } from 'motion/react';
import { useQueryClient } from '@tanstack/react-query';
import { $api } from '@lib/providers/api';
import { PasswordRow, TotpRow, WebAuthnRow, PgpRow, RecoveryCodesRow } from './components';

// ── Dashboard ─────────────────────────────────────────────────────────────────

export default function DashboardPage() {
    const queryClient = useQueryClient();
    const { data, isLoading, isError } = $api.useQuery('get', '/api/settings/factors');

    const refetch = () =>
        queryClient.invalidateQueries({ queryKey: ['get', '/api/settings/factors'] });

    return (
        <div className="min-h-screen bg-background">
            <div className="mx-auto max-w-2xl px-6 py-14">

                {/* Header */}
                <motion.div initial={{ opacity: 0, y: -6 }} animate={{ opacity: 1, y: 0 }} transition={{ duration: 0.25 }}
                    className="mb-8">
                    <h1 className="text-xl font-semibold mb-0.5">Account Security</h1>
                    <p className="text-sm text-muted-foreground">Keep your account info up to date to ensure you always have access.</p>
                </motion.div>

                {/* Skeleton */}
                {isLoading && (
                    <div className="rounded-2xl border border-border bg-card overflow-hidden">
                        {[...Array(5)].map((_, i) => (
                            <div key={i} className={`px-5 py-4 flex items-center gap-4 ${i < 4 ? 'border-b border-border/60' : ''}`}>
                                <div className="size-5 rounded bg-muted animate-pulse shrink-0" />
                                <div className="flex-1 space-y-2">
                                    <div className="h-3.5 w-28 rounded bg-muted animate-pulse" />
                                    <div className="h-3 w-40 rounded bg-muted animate-pulse" />
                                </div>
                            </div>
                        ))}
                    </div>
                )}

                {isError && (
                    <div className="rounded-2xl border border-border bg-card px-5 py-4">
                        <p className="text-sm text-destructive">Failed to load — make sure you are signed in.</p>
                    </div>
                )}

                {/* Card */}
                {data && (
                    <motion.div initial={{ opacity: 0, y: 6 }} animate={{ opacity: 1, y: 0 }} transition={{ duration: 0.25 }}
                        className="rounded-2xl border border-border bg-card overflow-hidden">
                        <PasswordRow isSet={data.password.is_set} onRefetch={refetch} />
                        <TotpRow totp={data.totp} onRefetch={refetch} />
                        <WebAuthnRow keys={data.webauthn} onRefetch={refetch} />
                        <PgpRow pgp={data.pgp} onRefetch={refetch} />
                        <RecoveryCodesRow remaining={data.recovery_codes.remaining_codes} onRefetch={refetch} />
                    </motion.div>
                )}
            </div>
        </div>
    );
}
