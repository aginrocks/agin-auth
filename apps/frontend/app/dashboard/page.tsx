'use client';

import { useState, useCallback } from 'react';
import { motion } from 'motion/react';
import { useQueryClient } from '@tanstack/react-query';
import { useRouter } from 'next/navigation';
import { $api } from '@lib/providers/api';
import { Button } from '@components/ui/button';
import { IconLogout, IconUser } from '@tabler/icons-react';
import { PasswordRow, TotpRow, WebAuthnRow, PgpRow, RecoveryCodesRow, ProfileRow, DeleteAccountSection } from './components';

export default function DashboardPage() {
    const queryClient = useQueryClient();
    const router = useRouter();
    const [loggingOut, setLoggingOut] = useState(false);

    const { data: profile } = $api.useQuery('get', '/api/settings/profile');
    const { data, isLoading, isError } = $api.useQuery('get', '/api/settings/factors');

    const refetch = () =>
        queryClient.invalidateQueries({ queryKey: ['get', '/api/settings/factors'] });

    const handleLogout = useCallback(async () => {
        setLoggingOut(true);
        await fetch('/api/logout', { method: 'POST' });
        router.push('/login');
    }, [router]);

    return (
        <div className="min-h-screen bg-background">
            <div className="mx-auto max-w-2xl px-6 py-14">

                {/* Header */}
                <motion.div initial={{ opacity: 0, y: -6 }} animate={{ opacity: 1, y: 0 }} transition={{ duration: 0.25 }}
                    className="mb-8 flex items-start justify-between">
                    <div>
                        <h1 className="text-xl font-semibold mb-0.5">Account Security</h1>
                        {profile ? (
                            <div className="flex items-center gap-2 text-sm text-muted-foreground">
                                <IconUser size={14} />
                                <span>{profile.display_name}</span>
                                <span className="text-muted-foreground/40">·</span>
                                <span>{profile.email}</span>
                                {!profile.email_confirmed && (
                                    <span className="text-[10px] bg-destructive/10 text-destructive rounded-full px-1.5 py-0.5">Unverified</span>
                                )}
                            </div>
                        ) : (
                            <p className="text-sm text-muted-foreground">Keep your account info up to date to ensure you always have access.</p>
                        )}
                    </div>
                    <Button variant="ghost" size="sm" onClick={handleLogout} disabled={loggingOut} className="text-muted-foreground hover:text-foreground">
                        <IconLogout size={16} />
                        {loggingOut ? 'Logging out…' : 'Log out'}
                    </Button>
                </motion.div>

                {/* Profile Card */}
                {profile && (
                    <motion.div initial={{ opacity: 0, y: 6 }} animate={{ opacity: 1, y: 0 }} transition={{ duration: 0.25 }}
                        className="mb-6">
                        <h2 className="text-sm font-medium text-muted-foreground mb-3">Profile</h2>
                        <div className="rounded-2xl border border-border bg-card overflow-hidden">
                            <ProfileRow profile={profile} />
                        </div>
                    </motion.div>
                )}

                {/* Skeleton */}
                {isLoading && (
                    <div className="rounded-2xl border border-border bg-card">
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

                {/* Security Factors Card */}
                {data && (
                    <motion.div initial={{ opacity: 0, y: 6 }} animate={{ opacity: 1, y: 0 }} transition={{ duration: 0.25 }}>
                        <h2 className="text-sm font-medium text-muted-foreground mb-3">Security</h2>
                        <div className="rounded-2xl border border-border bg-card overflow-hidden">
                            <PasswordRow isSet={data.password.is_set} onRefetch={refetch} />
                            <TotpRow totp={data.totp} onRefetch={refetch} />
                            <WebAuthnRow keys={data.webauthn} onRefetch={refetch} />
                            <PgpRow pgp={data.pgp} onRefetch={refetch} />
                            <RecoveryCodesRow remaining={data.recovery_codes.remaining_codes} onRefetch={refetch} />
                        </div>
                    </motion.div>
                )}

                {/* Danger Zone */}
                {data && (
                    <motion.div initial={{ opacity: 0 }} animate={{ opacity: 1 }} transition={{ duration: 0.25, delay: 0.1 }}
                        className="mt-8">
                        <h2 className="text-sm font-medium text-destructive/80 mb-3">Danger Zone</h2>
                        <div className="rounded-2xl border border-destructive/20 bg-card overflow-hidden">
                            <DeleteAccountSection />
                        </div>
                    </motion.div>
                )}
            </div>
        </div>
    );
}
